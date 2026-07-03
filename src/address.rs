use crate::{BlockHeight, Indexed, KeyScope};
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
    /// The role of the address within its account. For HD-derived transparent
    /// addresses this must be consistent with the derivation change component;
    /// it is the authoritative scope for shielded and unified addresses.
    scope: Option<KeyScope>,
    /// The block height at or around which this address was first exposed to
    /// a user or counterparty. Not recoverable from the chain; importers use
    /// it for gap-limit reasoning (maps to zcash_client_sqlite
    /// addresses.exposed_at_height). None = never exposed — e.g. zcashd
    /// keypool keys, which are pre-generated and not yet handed out.
    exposed_at_height: Option<BlockHeight>,
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
            scope: None,
            exposed_at_height: None,
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

    pub fn scope(&self) -> Option<KeyScope> {
        self.scope
    }

    pub fn set_scope(&mut self, scope: KeyScope) {
        self.scope = Some(scope);
    }

    pub fn exposed_at_height(&self) -> Option<BlockHeight> {
        self.exposed_at_height
    }

    pub fn set_exposed_at_height(&mut self, height: BlockHeight) {
        self.exposed_at_height = Some(height);
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        let envelope = Envelope::new(value.index)
            .add_type("Address")
            .add_assertion("address", value.address)
            .add_optional_assertion("scope", value.scope)
            .add_optional_assertion("exposed_at_height", value.exposed_at_height);
        value.attachments.add_to_envelope(envelope)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Address")?;
        let index = envelope.extract_subject()?;
        let address = envelope.try_object_for_predicate("address")?;
        let scope = envelope.try_optional_object_for_predicate("scope")?;
        let exposed_at_height =
            envelope.extract_optional_object_for_predicate("exposed_at_height")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;
        Ok(Address {
            index,
            address,
            scope,
            exposed_at_height,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use crate::{BlockHeight, KeyScope, ProtocolAddress, test_envelope_roundtrip};

    use super::Address;

    impl crate::RandomInstance for Address {
        fn random() -> Self {
            Self {
                index: 0,
                address: ProtocolAddress::random(),
                scope: KeyScope::opt_random(),
                exposed_at_height: BlockHeight::opt_random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Address);
}
