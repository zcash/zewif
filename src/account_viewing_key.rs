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
    /// A standalone Sapling extended full viewing key (canonical ZIP 32
    /// encoding).
    #[n(1)]
    SaplingExtFvk(#[n(0)] SaplingExtendedFullViewingKey),
    /// A Sprout viewing key in its canonical encoding. Sufficient to
    /// detect incoming Sprout notes.
    #[n(2)]
    SproutViewingKey(#[n(0)] SproutViewingKey),
    /// A set of transparent addresses with no unified key structure
    /// (legacy zcashd random-key wallet).
    #[n(3)]
    TransparentAddressSet,
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, test_cbor_roundtrip};

    use super::AccountViewingKey;

    impl RandomInstance for AccountViewingKey {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..4u32) {
                0 => AccountViewingKey::Ufvk(crate::UnifiedFullViewingKey::random()),
                1 => AccountViewingKey::SaplingExtFvk(
                    crate::sapling::SaplingExtendedFullViewingKey::random(),
                ),
                2 => AccountViewingKey::SproutViewingKey(crate::sprout::SproutViewingKey::random()),
                _ => AccountViewingKey::TransparentAddressSet,
            }
        }
    }

    test_cbor_roundtrip!(AccountViewingKey);
}
