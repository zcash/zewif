use minicbor::{Decode, Encode};

use crate::{
    Data, Extensions, SeedFingerprint, SeedMaterial, sapling::SaplingExtendedSpendingKey,
    sprout::SproutSpendingKey, transparent::TransparentSpendingKey,
};

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

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
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

/// A transparent private key stored under its secp256k1 public key
/// (33 or 65 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TransparentKeyEntry {
    #[n(0)]
    pubkey: Data,
    #[n(1)]
    key: TransparentSpendingKey,
}

impl TransparentKeyEntry {
    pub fn new(pubkey: Data, key: TransparentSpendingKey) -> Self {
        Self { pubkey, key }
    }

    pub fn pubkey(&self) -> &Data {
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
    fvk: Data,
    #[n(1)]
    key: SaplingExtendedSpendingKey,
}

impl SaplingKeyEntry {
    pub fn new(fvk: Data, key: SaplingExtendedSpendingKey) -> Self {
        Self { fvk, key }
    }

    pub fn fvk(&self) -> &Data {
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
        sapling::SaplingExtendedSpendingKey, sprout::SproutSpendingKey, test_cbor_roundtrip,
        transparent::TransparentSpendingKey,
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
            Self::new(Data::random_with_size(33), TransparentSpendingKey::random())
        }
    }

    impl RandomInstance for SaplingKeyEntry {
        fn random() -> Self {
            Self::new(Data::random(), SaplingExtendedSpendingKey::random())
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
}
