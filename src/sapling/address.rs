use crate::Blob;
use minicbor::{Decode, Encode};

/// A Sapling shielded address (zs-address).
///
/// Contains the address string and optional derivation metadata. Viewing
/// and spending keys are stored at the account level, not per-address.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Address {
    #[n(0)]
    address: String,
    /// The diversifier index used to derive this address, if known,
    /// stored as 11 bytes in little-endian order.
    #[n(1)]
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
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(Address);
}
