use bc_envelope::prelude::*;

use crate::{Bip39Mnemonic, LegacySeed, error::Error};

/// The secret material from which a wallet derives its keys.
///
/// Either a BIP-39 mnemonic phrase or a pre-BIP39 raw seed.
#[derive(Clone, PartialEq)]
pub enum SeedMaterial {
    /// A BIP-39 mnemonic phrase (typically 12 or 24 words) used as a
    /// human-readable seed
    Bip39Mnemonic(Bip39Mnemonic),
    /// A raw 32-byte seed predating the BIP-39 standard
    LegacySeed(LegacySeed),
}

impl std::fmt::Debug for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => {
                write!(f, "SeedMaterial::Bip39Mnemonic(\"{:?}\")", phrase)
            }
            Self::LegacySeed(seed) => {
                write!(f, "SeedMaterial::LegacySeed({:?})", seed)
            }
        }
    }
}

impl std::fmt::Display for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => {
                write!(f, "SeedMaterial::Bip39Mnemonic(\"{:?}\")", phrase)
            }
            Self::LegacySeed(seed) => {
                write!(f, "SeedMaterial::LegacySeed({:?})", seed)
            }
        }
    }
}

impl From<SeedMaterial> for Envelope {
    fn from(value: SeedMaterial) -> Self {
        match value {
            SeedMaterial::Bip39Mnemonic(mnemonic) => Envelope::new(mnemonic),
            SeedMaterial::LegacySeed(seed) => Envelope::new(seed),
        }
        .add_type("SeedMaterial")
    }
}

impl TryFrom<Envelope> for SeedMaterial {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SeedMaterial")?;
        if let Ok(mnemonic) = Bip39Mnemonic::try_from(envelope.clone()) {
            Ok(SeedMaterial::Bip39Mnemonic(mnemonic))
        } else if let Ok(seed) = LegacySeed::try_from(envelope.clone()) {
            Ok(SeedMaterial::LegacySeed(seed))
        } else {
            Err(Error::InvalidSeedMaterial.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeedMaterial;
    use crate::{Bip39Mnemonic, LegacySeed, test_envelope_roundtrip};

    impl crate::RandomInstance for SeedMaterial {
        fn random() -> Self {
            if rand::random::<bool>() {
                SeedMaterial::Bip39Mnemonic(Bip39Mnemonic::random())
            } else {
                SeedMaterial::LegacySeed(LegacySeed::random())
            }
        }
    }

    test_envelope_roundtrip!(SeedMaterial);
}
