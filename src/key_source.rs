use bc_envelope::prelude::*;

use crate::SeedFingerprint;

/// How an account's keys were obtained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    /// Derived from an HD seed via ZIP-32.
    Derived {
        seed_fingerprint: SeedFingerprint,
        /// ZIP-32 account index (e.g. 0 for normal accounts,
        /// 0x7FFFFFFF for the legacy zcashd account).
        account_index: u32,
    },
    /// Imported directly (e.g. a standalone viewing key).
    Imported,
}

impl From<KeySource> for Envelope {
    fn from(value: KeySource) -> Self {
        match value {
            KeySource::Derived { seed_fingerprint, account_index } => {
                Envelope::new(seed_fingerprint)
                    .add_type("KeySource")
                    .add_assertion("variant", "derived")
                    .add_assertion("account_index", account_index)
            }
            KeySource::Imported => {
                Envelope::new("imported")
                    .add_type("KeySource")
                    .add_assertion("variant", "imported")
            }
        }
    }
}

impl TryFrom<Envelope> for KeySource {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("KeySource")?;
        let variant: String = envelope.extract_object_for_predicate("variant")?;
        match variant.as_str() {
            "derived" => {
                let seed_fingerprint: SeedFingerprint = envelope.extract_subject()?;
                let account_index: u32 =
                    envelope.extract_object_for_predicate("account_index")?;
                Ok(KeySource::Derived { seed_fingerprint, account_index })
            }
            "imported" => Ok(KeySource::Imported),
            other => Err(bc_envelope::Error::General(
                format!("unknown KeySource variant: {}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, RandomInstance, SeedFingerprint};

    use super::KeySource;

    impl RandomInstance for KeySource {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.7) {
                KeySource::Derived {
                    seed_fingerprint: SeedFingerprint::random(),
                    account_index: rng.random_range(0..10u32),
                }
            } else {
                KeySource::Imported
            }
        }
    }

    test_envelope_roundtrip!(KeySource);
}
