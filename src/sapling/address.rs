use crate::Blob;
use bc_envelope::prelude::*;

/// A Sapling shielded address (zs-address).
///
/// Contains the address string and optional derivation metadata. Viewing
/// and spending keys are stored at the account level, not per-address.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    address: String,
    /// The diversifier index used to derive this address, if known,
    /// stored as 11 bytes in little-endian order.
    diversifier_index: Option<Blob<11>>,
}

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Address {
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

    pub fn set_diversifier_index(&mut self, d: Blob<11>) {
        self.diversifier_index = Some(d);
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        Envelope::new(value.address)
            .add_type("SaplingAddress")
            .add_optional_assertion("diversifier_index", value.diversifier_index)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SaplingAddress")?;
        let address = envelope.extract_subject()?;
        let diversifier_index =
            envelope.try_optional_object_for_predicate("diversifier_index")?;
        Ok(Address { address, diversifier_index })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            address: String::random(),
            diversifier_index: Blob::<11>::opt_random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::test_envelope_roundtrip;

    test_envelope_roundtrip!(Address);
}
