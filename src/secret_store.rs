use minicbor::{Decode, Encode};

use crate::{
    Data, Extensions, SeedFingerprint, SeedMaterial, UnifiedFullViewingKey,
    sapling::SaplingExtendedSpendingKey,
    sapling::SaplingExtendedFullViewingKey, sprout::SproutSpendingKey,
    transparent::{TransparentPubKey, TransparentSpendingKey},
};

#[cfg(feature = "encryption")]
use crate::Error;

/// The sensitive key material of a ZeWIF document, either in plain CBOR or
/// as an opaque ciphertext.
///
/// A viewing-only export omits the secrets node entirely; importers treat
/// the absence of an expected secret-store entry as a viewing-only import
/// of the affected item, not as an error.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum Secrets {
    /// The secret store in plain CBOR.
    #[n(0)]
    Plain(#[n(0)] SecretStore),
    /// An age ciphertext whose plaintext is the CBOR encoding of a
    /// [`SecretStore`].
    #[n(1)]
    Encrypted(#[n(0)] EncryptedStore),
}

#[cfg(feature = "encryption")]
impl Secrets {
    /// Returns the plain secret store, decrypting an encrypted store with the
    /// given age identities if necessary.
    pub fn decrypt<'a>(
        &self,
        identities: impl Iterator<Item = &'a dyn age::Identity>,
    ) -> Result<SecretStore, Error> {
        match self {
            Secrets::Plain(store) => Ok(store.clone()),
            Secrets::Encrypted(encrypted) => encrypted.decrypt(identities),
        }
    }
}

/// An encrypted secret store: an age ciphertext whose plaintext is the CBOR
/// encoding of a [`SecretStore`].
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct EncryptedStore {
    #[n(0)]
    ciphertext: Data,
}

impl EncryptedStore {
    pub fn new(ciphertext: Data) -> Self {
        Self { ciphertext }
    }

    pub fn ciphertext(&self) -> &Data {
        &self.ciphertext
    }
}

#[cfg(feature = "encryption")]
impl EncryptedStore {
    /// Decrypts this age ciphertext with the given identities and decodes the
    /// plaintext as the CBOR encoding of a [`SecretStore`].
    pub fn decrypt<'a>(
        &self,
        identities: impl Iterator<Item = &'a dyn age::Identity>,
    ) -> Result<SecretStore, Error> {
        use std::io::Read;

        let decryptor = age::Decryptor::new_buffered(self.ciphertext.as_slice())?;
        let mut reader = decryptor.decrypt(identities)?;
        let mut plaintext = Vec::new();
        reader
            .read_to_end(&mut plaintext)
            .map_err(age::DecryptError::from)?;
        minicbor::decode(&plaintext).map_err(Error::SecretStoreDecode)
    }
}

/// All secret key material in a ZeWIF document, referenced from the public
/// wallet structure by public identifiers: seeds by their ZIP 32 seed
/// fingerprint, transparent private keys by their public key, Sapling
/// spending keys by their full viewing key encoding, and Sprout spending
/// keys by their address.
#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
#[cbor(map)]
pub struct SecretStore {
    #[n(0)]
    seeds: Vec<SeedEntry>,
    #[n(1)]
    transparent_keys: Vec<TransparentKeyEntry>,
    #[n(2)]
    sapling_keys: Vec<SaplingKeyEntry>,
    #[n(3)]
    sprout_keys: Vec<SproutKeyEntry>,
    #[cbor(n(4), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
    /// Extracted single-account unified spending keys. Empty for wallets
    /// whose spend authority is held as seeds; omitted from the encoding
    /// when empty.
    #[cbor(n(5), with = "unified_keys_field", has_nil)]
    unified_keys: Vec<UnifiedKeyEntry>,
}

/// Field codec for the unified-key list: the map entry is omitted when the
/// list is empty, and an absent (or null) entry decodes as empty.
///
/// Use as `#[cbor(n(IDX), with = "unified_keys_field", has_nil)]`.
#[doc(hidden)]
pub mod unified_keys_field {
    use minicbor::decode::Error as DecodeError;
    use minicbor::encode::{Error as EncodeError, Write};
    use minicbor::{Decoder, Encoder};

    use super::UnifiedKeyEntry;

    pub fn encode<Ctx, W: Write>(
        v: &Vec<UnifiedKeyEntry>,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), EncodeError<W::Error>> {
        minicbor::Encode::encode(v, e, ctx)
    }

    pub fn decode<'b, Ctx>(
        d: &mut Decoder<'b>,
        ctx: &mut Ctx,
    ) -> Result<Vec<UnifiedKeyEntry>, DecodeError> {
        if d.datatype()? == minicbor::data::Type::Null {
            d.skip()?;
            return Ok(Vec::new());
        }
        minicbor::Decode::decode(d, ctx)
    }

    pub fn is_nil(v: &[UnifiedKeyEntry]) -> bool {
        v.is_empty()
    }

    pub fn nil() -> Option<Vec<UnifiedKeyEntry>> {
        Some(Vec::new())
    }
}

impl SecretStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seeds(&self) -> &[SeedEntry] {
        &self.seeds
    }

    pub fn add_seed(&mut self, entry: SeedEntry) {
        self.seeds.push(entry);
    }

    pub fn transparent_keys(&self) -> &[TransparentKeyEntry] {
        &self.transparent_keys
    }

    pub fn add_transparent_key(&mut self, entry: TransparentKeyEntry) {
        self.transparent_keys.push(entry);
    }

    pub fn sapling_keys(&self) -> &[SaplingKeyEntry] {
        &self.sapling_keys
    }

    pub fn add_sapling_key(&mut self, entry: SaplingKeyEntry) {
        self.sapling_keys.push(entry);
    }

    pub fn sprout_keys(&self) -> &[SproutKeyEntry] {
        &self.sprout_keys
    }

    pub fn add_sprout_key(&mut self, entry: SproutKeyEntry) {
        self.sprout_keys.push(entry);
    }

    pub fn unified_keys(&self) -> &[UnifiedKeyEntry] {
        &self.unified_keys
    }

    pub fn add_unified_key(&mut self, entry: UnifiedKeyEntry) {
        self.unified_keys.push(entry);
    }

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

crate::text_key!(
    UnifiedSpendingKey,
    "An extracted single-account unified spending key in the Bech32m text
encoding obtained by applying F4Jumble to its raw encoding, per the
unified raw encodings draft ZIP (zcash/zips#660).",
    "usk1",
    redacted
);

/// An extracted single-account unified spending key, stored under the
/// unified full viewing key it corresponds to.
///
/// Most wallets hold spend authority as seeds (from which unified spending
/// keys are ZIP 32-derived on demand); this entry represents the rarer case
/// of a wallet holding only an extracted per-account spending key.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct UnifiedKeyEntry {
    #[n(0)]
    fvk: UnifiedFullViewingKey,
    #[n(1)]
    key: UnifiedSpendingKey,
}

impl UnifiedKeyEntry {
    pub fn new(fvk: UnifiedFullViewingKey, key: UnifiedSpendingKey) -> Self {
        Self { fvk, key }
    }

    pub fn fvk(&self) -> &UnifiedFullViewingKey {
        &self.fvk
    }

    pub fn spending_key(&self) -> &UnifiedSpendingKey {
        &self.key
    }
}

#[cfg(feature = "encryption")]
impl SecretStore {
    /// Encrypts the CBOR encoding of this secret store to the given age
    /// recipients.
    ///
    /// The choice of recipients (passphrase-derived or X25519) is the
    /// caller's; the document carries only the resulting ciphertext.
    pub fn encrypt<'a>(
        &self,
        recipients: impl Iterator<Item = &'a dyn age::Recipient>,
    ) -> Result<EncryptedStore, Error> {
        use std::io::Write;

        let plaintext = minicbor::to_vec(self).expect("encoding to a byte vector cannot fail");
        let encryptor = age::Encryptor::with_recipients(recipients)?;
        let mut ciphertext = Vec::new();
        let mut writer = encryptor
            .wrap_output(&mut ciphertext)
            .map_err(age::EncryptError::from)?;
        writer
            .write_all(&plaintext)
            .map_err(age::EncryptError::from)?;
        writer.finish().map_err(age::EncryptError::from)?;
        Ok(EncryptedStore::new(Data::from_vec(ciphertext)))
    }
}

/// Seed material stored under its ZIP 32 seed fingerprint, as referenced by
/// derived key sources.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct SeedEntry {
    #[n(0)]
    fingerprint: SeedFingerprint,
    #[n(1)]
    material: SeedMaterial,
}

impl SeedEntry {
    pub fn new(fingerprint: SeedFingerprint, material: SeedMaterial) -> Self {
        Self {
            fingerprint,
            material,
        }
    }

    pub fn fingerprint(&self) -> &SeedFingerprint {
        &self.fingerprint
    }

    pub fn material(&self) -> &SeedMaterial {
        &self.material
    }
}

/// A transparent private key stored under its secp256k1 public key.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TransparentKeyEntry {
    #[n(0)]
    pubkey: TransparentPubKey,
    #[n(1)]
    key: TransparentSpendingKey,
}

impl TransparentKeyEntry {
    pub fn new(pubkey: TransparentPubKey, key: TransparentSpendingKey) -> Self {
        Self { pubkey, key }
    }

    pub fn pubkey(&self) -> &TransparentPubKey {
        &self.pubkey
    }

    pub fn spending_key(&self) -> &TransparentSpendingKey {
        &self.key
    }
}

/// A Sapling extended spending key stored under the canonical encoding of
/// the extended full viewing key it corresponds to.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SaplingKeyEntry {
    #[n(0)]
    fvk: SaplingExtendedFullViewingKey,
    #[n(1)]
    key: SaplingExtendedSpendingKey,
}

impl SaplingKeyEntry {
    pub fn new(fvk: SaplingExtendedFullViewingKey, key: SaplingExtendedSpendingKey) -> Self {
        Self { fvk, key }
    }

    pub fn fvk(&self) -> &SaplingExtendedFullViewingKey {
        &self.fvk
    }

    pub fn spending_key(&self) -> &SaplingExtendedSpendingKey {
        &self.key
    }
}

/// A Sprout spending key stored under its Sprout address.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SproutKeyEntry {
    #[n(0)]
    address: String,
    #[n(1)]
    key: SproutSpendingKey,
}

impl SproutKeyEntry {
    pub fn new(address: impl Into<String>, key: SproutSpendingKey) -> Self {
        Self {
            address: address.into(),
            key,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn spending_key(&self) -> &SproutSpendingKey {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Data, Extensions, RandomInstance, SeedFingerprint, SeedMaterial,
        sapling::{SaplingExtendedFullViewingKey, SaplingExtendedSpendingKey},
        sprout::SproutSpendingKey, test_cbor_roundtrip,
        transparent::{TransparentPubKey, TransparentSpendingKey},
    };

    use super::{
        EncryptedStore, SaplingKeyEntry, SecretStore, Secrets, SeedEntry, SproutKeyEntry,
        TransparentKeyEntry,
    };

    impl RandomInstance for SeedEntry {
        fn random() -> Self {
            Self::new(SeedFingerprint::random(), SeedMaterial::random())
        }
    }

    impl RandomInstance for TransparentKeyEntry {
        fn random() -> Self {
            Self::new(TransparentPubKey::random(), TransparentSpendingKey::random())
        }
    }

    impl RandomInstance for SaplingKeyEntry {
        fn random() -> Self {
            Self::new(
                SaplingExtendedFullViewingKey::random(),
                SaplingExtendedSpendingKey::random(),
            )
        }
    }

    impl RandomInstance for SproutKeyEntry {
        fn random() -> Self {
            Self::new(String::random(), SproutSpendingKey::random())
        }
    }

    impl RandomInstance for SecretStore {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let mut store = SecretStore::new();
            for _ in 0..rng.random_range(0..3) {
                store.add_seed(SeedEntry::random());
            }
            for _ in 0..rng.random_range(0..3) {
                store.add_transparent_key(TransparentKeyEntry::random());
            }
            for _ in 0..rng.random_range(0..3) {
                store.add_sapling_key(SaplingKeyEntry::random());
            }
            for _ in 0..rng.random_range(0..3) {
                store.add_sprout_key(SproutKeyEntry::random());
            }
            *store.extensions_mut() = Extensions::random();
            store
        }
    }

    impl RandomInstance for Secrets {
        fn random() -> Self {
            use rand::Rng;
            if rand::rng().random_bool(0.8) {
                Secrets::Plain(SecretStore::random())
            } else {
                Secrets::Encrypted(EncryptedStore::new(Data::random()))
            }
        }
    }

    test_cbor_roundtrip!(Secrets);
    test_cbor_roundtrip!(SecretStore, test_secret_store);
    test_cbor_roundtrip!(SeedEntry, test_seed_entry);
    test_cbor_roundtrip!(TransparentKeyEntry, test_transparent_key_entry);
    test_cbor_roundtrip!(SaplingKeyEntry, test_sapling_key_entry);
    test_cbor_roundtrip!(SproutKeyEntry, test_sprout_key_entry);

    #[cfg(feature = "encryption")]
    mod encryption {
        use std::iter;

        use super::super::{
            EncryptedStore, SaplingKeyEntry, SecretStore, Secrets, SeedEntry, SproutKeyEntry,
            TransparentKeyEntry,
        };
        use crate::{Error, Extensions, RandomInstance};

        /// A secret store containing at least one entry of each kind.
        fn sample_store() -> SecretStore {
            let mut store = SecretStore::new();
            store.add_seed(SeedEntry::random());
            store.add_transparent_key(TransparentKeyEntry::random());
            store.add_sapling_key(SaplingKeyEntry::random());
            store.add_sprout_key(SproutKeyEntry::random());
            *store.extensions_mut() = Extensions::random();
            store
        }

        fn encrypt_to(store: &SecretStore, identity: &age::x25519::Identity) -> EncryptedStore {
            let recipient = identity.to_public();
            store
                .encrypt(iter::once(&recipient as &dyn age::Recipient))
                .unwrap()
        }

        #[test]
        fn encrypt_decrypt_roundtrip() {
            let store = sample_store();
            let identity = age::x25519::Identity::generate();
            let encrypted = encrypt_to(&store, &identity);

            let decrypted = encrypted
                .decrypt(iter::once(&identity as &dyn age::Identity))
                .unwrap();
            assert_eq!(decrypted, store);
        }

        #[test]
        fn secrets_decrypt() {
            let store = sample_store();
            let identity = age::x25519::Identity::generate();

            let encrypted = Secrets::Encrypted(encrypt_to(&store, &identity));
            let decrypted = encrypted
                .decrypt(iter::once(&identity as &dyn age::Identity))
                .unwrap();
            assert_eq!(decrypted, store);

            let plain = Secrets::Plain(store.clone());
            let passed_through = plain.decrypt(iter::empty()).unwrap();
            assert_eq!(passed_through, store);
        }

        #[test]
        fn decrypt_with_wrong_identity_fails() {
            let store = sample_store();
            let identity = age::x25519::Identity::generate();
            let wrong_identity = age::x25519::Identity::generate();
            let encrypted = encrypt_to(&store, &identity);

            let result = encrypted.decrypt(iter::once(&wrong_identity as &dyn age::Identity));
            assert!(matches!(result, Err(Error::SecretStoreDecryption(_))));
        }
    }
}
