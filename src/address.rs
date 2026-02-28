use crate::Indexed;
use bc_envelope::prelude::*;

use super::ProtocolAddress;

/// A wallet address wrapping a protocol-specific address.
///
/// This is the entry in an account's address list. It pairs the
/// protocol-specific address data with an index for ordered serialization.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    index: usize,
    address: ProtocolAddress,
    attachments: Attachments,
}

impl Indexed for Address {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

bc_envelope::impl_attachable!(Address);

impl Address {
    pub fn new(address: ProtocolAddress) -> Self {
        Self {
            index: 0,
            address,
            attachments: Attachments::new(),
        }
    }

    /// Returns the address in canonical string format.
    pub fn as_string(&self) -> String {
        self.address.as_string()
    }

    pub fn address(&self) -> &ProtocolAddress {
        &self.address
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        let envelope = Envelope::new(value.index)
            .add_type("Address")
            .add_assertion("address", value.address);
        value.attachments.add_to_envelope(envelope)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Address")?;
        let index = envelope.extract_subject()?;
        let address = envelope.try_object_for_predicate("address")?;
        let attachments =
            Attachments::try_from_envelope(&envelope).map_err(|e| {
                bc_envelope::Error::General(format!("attachments: {}", e))
            })?;
        Ok(Address { index, address, attachments })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use crate::{ProtocolAddress, test_envelope_roundtrip};

    use super::Address;

    impl crate::RandomInstance for Address {
        fn random() -> Self {
            Self {
                index: 0,
                address: ProtocolAddress::random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Address);
}
