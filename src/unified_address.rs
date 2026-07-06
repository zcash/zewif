use crate::DiversifierIndex;
use minicbor::{Decode, Encode};

/// A Zcash Unified Address (u-address) as defined in ZIP-316.
///
/// Bundles receivers for multiple protocols (transparent, Sapling, Orchard)
/// into a single address string. Derivation path information is stored at
/// the account level via `KeySource`.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct UnifiedAddress {
    #[n(0)]
    address: String,
    /// The diversifier index used to derive this address, if known,
    /// stored as 11 bytes in little-endian order.
    #[n(1)]
    diversifier_index: Option<DiversifierIndex>,
}

impl UnifiedAddress {
    pub fn new(address: impl Into<String>) -> Self {
        UnifiedAddress {
            address: address.into(),
            diversifier_index: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn diversifier_index(&self) -> Option<&DiversifierIndex> {
        self.diversifier_index.as_ref()
    }

    pub fn set_diversifier_index(&mut self, diversifier_index: DiversifierIndex) {
        self.diversifier_index = Some(diversifier_index);
    }
}

#[cfg(test)]
mod tests {
    use crate::{DiversifierIndex, test_cbor_roundtrip};

    use super::UnifiedAddress;

    impl crate::RandomInstance for UnifiedAddress {
        fn random() -> Self {
            Self {
                address: String::random(),
                diversifier_index: DiversifierIndex::opt_random(),
            }
        }
    }

    test_cbor_roundtrip!(UnifiedAddress);
}
