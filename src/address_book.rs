use crate::Indexed;
use bc_envelope::prelude::*;

/// A user-facing association between a Zcash address and descriptive metadata.
///
/// Entries may describe wallet-owned addresses (e.g. recording whom an address
/// was given to) or external counterparty addresses (contacts).
#[derive(Debug, Clone, PartialEq)]
pub struct AddressBookEntry {
    index: usize,
    /// The address in its canonical string encoding (any protocol).
    address: String,
    /// User-assigned name for the address (zcashd address book name records).
    label: Option<String>,
    /// Usage tag, e.g. "receive", "send", "unknown" (zcashd purpose records).
    purpose: Option<String>,
    attachments: Attachments,
}

impl Indexed for AddressBookEntry {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

bc_envelope::impl_attachable!(AddressBookEntry);

impl AddressBookEntry {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            index: 0,
            address: address.into(),
            label: None,
            purpose: None,
            attachments: Attachments::new(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = Some(label.into());
    }

    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    pub fn set_purpose(&mut self, purpose: impl Into<String>) {
        self.purpose = Some(purpose.into());
    }
}

impl From<AddressBookEntry> for Envelope {
    fn from(value: AddressBookEntry) -> Self {
        let envelope = Envelope::new(value.index)
            .add_type("AddressBookEntry")
            .add_assertion("address", value.address)
            .add_optional_assertion("label", value.label)
            .add_optional_assertion("purpose", value.purpose);
        value.attachments.add_to_envelope(envelope)
    }
}

impl TryFrom<Envelope> for AddressBookEntry {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("AddressBookEntry")?;
        let index = envelope.extract_subject()?;
        let address = envelope.extract_object_for_predicate("address")?;
        let label = envelope.extract_optional_object_for_predicate("label")?;
        let purpose = envelope.extract_optional_object_for_predicate("purpose")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;
        Ok(Self {
            index,
            address,
            label,
            purpose,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use crate::test_envelope_roundtrip;

    use super::AddressBookEntry;

    impl crate::RandomInstance for AddressBookEntry {
        fn random() -> Self {
            Self {
                index: 0,
                address: String::random(),
                label: String::opt_random(),
                purpose: String::opt_random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(AddressBookEntry);
}
