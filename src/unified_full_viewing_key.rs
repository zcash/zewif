use bc_envelope::prelude::*;

/// A Unified Full Viewing Key in its canonical ZIP-316 encoding.
///
/// UFVKs bundle viewing keys for multiple Zcash protocols (transparent,
/// Sapling, Orchard) into a single key. The encoded string is the
/// canonical representation and can be parsed by any UFVK-aware wallet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedFullViewingKey {
    encoding: String,
}

impl UnifiedFullViewingKey {
    pub fn new(encoding: impl Into<String>) -> Self {
        Self { encoding: encoding.into() }
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

impl From<UnifiedFullViewingKey> for Envelope {
    fn from(value: UnifiedFullViewingKey) -> Self {
        Envelope::new(value.encoding)
            .add_type("UnifiedFullViewingKey")
    }
}

impl TryFrom<Envelope> for UnifiedFullViewingKey {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("UnifiedFullViewingKey")?;
        let encoding: String = envelope.extract_subject()?;
        Ok(Self { encoding })
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, RandomInstance};

    use super::UnifiedFullViewingKey;

    impl RandomInstance for UnifiedFullViewingKey {
        fn random() -> Self {
            // Use a placeholder UFVK-like string for testing
            Self {
                encoding: format!("uview1{}", String::random()),
            }
        }
    }

    test_envelope_roundtrip!(UnifiedFullViewingKey);
}
