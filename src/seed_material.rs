use minicbor::{Decode, Encode};

use crate::{Bip39Mnemonic, LegacySeed};

/// The secret material from which a wallet derives its keys.
///
/// Either a BIP-39 mnemonic phrase or a pre-BIP39 raw seed.
#[derive(Clone, PartialEq, Encode, Decode)]
#[cbor(flat)]
pub enum SeedMaterial {
    /// A BIP-39 mnemonic phrase (typically 12 or 24 words) used as a
    /// human-readable seed
    #[n(0)]
    Bip39Mnemonic(#[n(0)] Bip39Mnemonic),
    /// A raw seed predating the BIP-39 standard
    #[n(1)]
    LegacySeed(#[n(0)] LegacySeed),
}

impl std::fmt::Debug for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => {
                write!(f, "SeedMaterial::Bip39Mnemonic(\"{phrase:?}\")")
            }
            Self::LegacySeed(seed) => {
                write!(f, "SeedMaterial::LegacySeed({seed:?})")
            }
        }
    }
}

impl std::fmt::Display for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => {
                write!(f, "SeedMaterial::Bip39Mnemonic(\"{phrase:?}\")")
            }
            Self::LegacySeed(seed) => {
                write!(f, "SeedMaterial::LegacySeed({seed:?})")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeedMaterial;
    use crate::{Bip39Mnemonic, LegacySeed, test_cbor_roundtrip};

    impl crate::RandomInstance for SeedMaterial {
        fn random() -> Self {
            if rand::random::<bool>() {
                SeedMaterial::Bip39Mnemonic(Bip39Mnemonic::random())
            } else {
                SeedMaterial::LegacySeed(LegacySeed::random())
            }
        }
    }

    test_cbor_roundtrip!(SeedMaterial);
}
