use minicbor::{Decode, Encode};

use crate::{
    UnifiedFullViewingKey, sapling::SaplingExtendedFullViewingKey, sprout::SproutViewingKey,
};

/// The viewing capability associated with an account.
///
/// This determines what the account can observe on-chain. Each variant
/// corresponds to a different era or style of Zcash key management.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum AccountViewingKey {
    /// A Unified Full Viewing Key (canonical ZIP-316 encoding), which may
    /// contain Orchard, Sapling, and/or transparent components.
    #[n(0)]
    Ufvk(#[n(0)] UnifiedFullViewingKey),
    /// A standalone Sapling extended full viewing key (canonical encoding).
    #[n(1)]
    SaplingExtFvk(#[n(0)] SaplingExtFvk),
    /// A Sprout viewing key (64 bytes: `a_pk` + `sk_enc`). Sufficient to
    /// detect incoming Sprout notes.
    #[n(2)]
    SproutViewingKey(#[n(0)] SproutViewingKey),
    /// A set of transparent addresses with no unified key structure
    /// (legacy zcashd random-key wallet).
    #[n(3)]
    TransparentAddressSet,
}

/// A standalone Sapling extended full viewing key in its canonical ZIP 32
/// encoding.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SaplingExtFvk {
    #[n(0)]
    fvk: SaplingExtendedFullViewingKey,
}

impl SaplingExtFvk {
    pub fn new(fvk: SaplingExtendedFullViewingKey) -> Self {
        Self { fvk }
    }

    pub fn fvk(&self) -> &SaplingExtendedFullViewingKey {
        &self.fvk
    }
}

impl From<SaplingExtendedFullViewingKey> for SaplingExtFvk {
    fn from(fvk: SaplingExtendedFullViewingKey) -> Self {
        Self::new(fvk)
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, test_cbor_roundtrip};

    use super::{AccountViewingKey, SaplingExtFvk};

    impl RandomInstance for AccountViewingKey {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..4u32) {
                0 => AccountViewingKey::Ufvk(crate::UnifiedFullViewingKey::random()),
                1 => AccountViewingKey::SaplingExtFvk(SaplingExtFvk::new(
                    crate::sapling::SaplingExtendedFullViewingKey::random(),
                )),
                2 => AccountViewingKey::SproutViewingKey(crate::sprout::SproutViewingKey::random()),
                _ => AccountViewingKey::TransparentAddressSet,
            }
        }
    }

    test_cbor_roundtrip!(AccountViewingKey);
}
