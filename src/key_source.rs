use minicbor::{Decode, Encode};

use crate::SeedFingerprint;

/// How an account's keys were obtained.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum KeySource {
    /// Derived from an HD seed via ZIP-32.
    #[n(0)]
    Derived(#[n(0)] DerivedKeySource),
    /// Imported directly (e.g. a standalone viewing key).
    #[n(1)]
    Imported,
}

/// The derivation metadata for an account whose keys are recoverable from
/// an HD seed.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct DerivedKeySource {
    #[n(0)]
    seed_fingerprint: SeedFingerprint,
    /// ZIP-32 account index (e.g. 0 for normal accounts,
    /// 0x7FFFFFFF for the legacy zcashd account).
    #[n(1)]
    account_index: u32,
    /// For accounts derived via zcashd's legacy post-v4.7.0 path
    /// m/32h/coin_type_h/0x7FFFFFFFh/address_index_h, the address
    /// index. Always hardened in derivation; valid values are below
    /// 2^31. Maps to zcash_keys keys::zcashd::LegacyAddressIndex and
    /// zcash_client_sqlite accounts.zcashd_legacy_address_index.
    #[n(2)]
    legacy_address_index: Option<u32>,
}

impl DerivedKeySource {
    pub fn new(
        seed_fingerprint: SeedFingerprint,
        account_index: u32,
        legacy_address_index: Option<u32>,
    ) -> Self {
        Self {
            seed_fingerprint,
            account_index,
            legacy_address_index,
        }
    }

    pub fn seed_fingerprint(&self) -> &SeedFingerprint {
        &self.seed_fingerprint
    }

    pub fn account_index(&self) -> u32 {
        self.account_index
    }

    pub fn legacy_address_index(&self) -> Option<u32> {
        self.legacy_address_index
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, SeedFingerprint, test_cbor_roundtrip};

    use super::{DerivedKeySource, KeySource};

    impl RandomInstance for KeySource {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.7) {
                KeySource::Derived(DerivedKeySource::new(
                    SeedFingerprint::random(),
                    rng.random_range(0..10u32),
                    rng.random_bool(0.3)
                        .then(|| rng.random_range(0..(1u32 << 31))),
                ))
            } else {
                KeySource::Imported
            }
        }
    }

    test_cbor_roundtrip!(KeySource);
}
