use crate::Extensions;
use minicbor::{Decode, Encode};

/// A user-facing association between a Zcash address and descriptive metadata.
///
/// Entries may describe wallet-owned addresses (e.g. recording whom an address
/// was given to) or external counterparty addresses (contacts).
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct AddressBookEntry {
    /// The address in its canonical string encoding (any protocol).
    #[n(0)]
    address: String,
    /// User-assigned name for the address (zcashd address book name records).
    #[n(1)]
    label: Option<String>,
    /// Usage tag, e.g. "receive", "send", "unknown" (zcashd purpose records).
    #[n(2)]
    purpose: Option<String>,
    #[cbor(n(3), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
}

impl AddressBookEntry {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            label: None,
            purpose: None,
            extensions: Extensions::new(),
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

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

#[cfg(test)]
mod tests {
    use crate::{Extensions, test_cbor_roundtrip};

    use super::AddressBookEntry;

    impl crate::RandomInstance for AddressBookEntry {
        fn random() -> Self {
            Self {
                address: String::random(),
                label: String::opt_random(),
                purpose: String::opt_random(),
                extensions: Extensions::random(),
            }
        }
    }

    test_cbor_roundtrip!(AddressBookEntry);
}
