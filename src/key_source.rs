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
        /// For accounts derived via zcashd's legacy post-v4.7.0 path
        /// m/32h/coin_type_h/0x7FFFFFFFh/address_index_h, the address
        /// index. Always hardened in derivation; valid values are below
        /// 2^31. Maps to zcash_keys keys::zcashd::LegacyAddressIndex and
        /// zcash_client_sqlite accounts.zcashd_legacy_address_index.
        legacy_address_index: Option<u32>,
    },
    /// Imported directly (e.g. a standalone viewing key).
    Imported,
}

impl From<KeySource> for Envelope {
    fn from(value: KeySource) -> Self {
        match value {
            KeySource::Derived {
                seed_fingerprint,
                account_index,
                legacy_address_index,
            } => Envelope::new(seed_fingerprint)
                .add_type("KeySource")
                .add_assertion("variant", "derived")
                .add_assertion("account_index", account_index)
                .add_optional_assertion("legacy_address_index", legacy_address_index),
            KeySource::Imported => Envelope::new("imported")
                .add_type("KeySource")
                .add_assertion("variant", "imported"),
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
                let account_index: u32 = envelope.extract_object_for_predicate("account_index")?;
                let legacy_address_index: Option<u32> =
                    envelope.extract_optional_object_for_predicate("legacy_address_index")?;
                Ok(KeySource::Derived {
                    seed_fingerprint,
                    account_index,
                    legacy_address_index,
                })
            }
            "imported" => Ok(KeySource::Imported),
            other => Err(bc_envelope::Error::General(format!(
                "unknown KeySource variant: {}",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, SeedFingerprint, test_envelope_roundtrip};

    use super::KeySource;

    impl RandomInstance for KeySource {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.7) {
                KeySource::Derived {
                    seed_fingerprint: SeedFingerprint::random(),
                    account_index: rng.random_range(0..10u32),
                    legacy_address_index: if rng.random_bool(0.3) {
                        Some(rng.random_range(0..(1u32 << 31)))
                    } else {
                        None
                    },
                }
            } else {
                KeySource::Imported
            }
        }
    }

    test_envelope_roundtrip!(KeySource);
}
