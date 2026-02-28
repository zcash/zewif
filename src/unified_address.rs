use crate::Blob;
use bc_envelope::prelude::*;

/// A Zcash Unified Address (u-address) as defined in ZIP-316.
///
/// Bundles receivers for multiple protocols (transparent, Sapling, Orchard)
/// into a single address string. Derivation path information is stored at
/// the account level via `KeySource`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnifiedAddress {
    address: String,
    /// The diversifier index used to derive this address, if known,
    /// stored as 11 bytes in little-endian order.
    diversifier_index: Option<Blob<11>>,
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

    pub fn diversifier_index(&self) -> Option<&Blob<11>> {
        self.diversifier_index.as_ref()
    }

    pub fn set_diversifier_index(&mut self, diversifier_index: Blob<11>) {
        self.diversifier_index = Some(diversifier_index);
    }
}

impl From<UnifiedAddress> for Envelope {
    fn from(value: UnifiedAddress) -> Self {
        Envelope::new(value.address)
            .add_type("UnifiedAddress")
            .add_optional_assertion("diversifier_index", value.diversifier_index)
    }
}

impl TryFrom<Envelope> for UnifiedAddress {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("UnifiedAddress")?;
        let address = envelope.extract_subject()?;
        let diversifier_index =
            envelope.try_optional_object_for_predicate("diversifier_index")?;
        Ok(UnifiedAddress { address, diversifier_index })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blob, test_envelope_roundtrip};

    use super::UnifiedAddress;

    impl crate::RandomInstance for UnifiedAddress {
        fn random() -> Self {
            Self {
                address: String::random(),
                diversifier_index: Blob::opt_random(),
            }
        }
    }

    test_envelope_roundtrip!(UnifiedAddress);
}
