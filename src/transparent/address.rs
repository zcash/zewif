use super::TransparentSpendAuthority;
use bc_envelope::prelude::*;

/// A transparent Zcash address (t-address).
///
/// Contains the address string and optionally how the key was obtained
/// (HD-derived with derivation info, or imported with the private key).
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    address: String,
    /// How the key for this address was obtained, if known.
    spend_authority: Option<TransparentSpendAuthority>,
}

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Address {
            address: address.into(),
            spend_authority: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn spend_authority(&self) -> Option<&TransparentSpendAuthority> {
        self.spend_authority.as_ref()
    }

    pub fn set_spend_authority(&mut self, spend_authority: TransparentSpendAuthority) {
        self.spend_authority = Some(spend_authority);
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        Envelope::new(value.address)
            .add_type("TransparentAddress")
            .add_optional_assertion("spend_authority", value.spend_authority)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransparentAddress")?;
        let address = envelope.extract_subject()?;
        let spend_authority =
            envelope.try_optional_object_for_predicate("spend_authority")?;
        Ok(Address { address, spend_authority })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            address: String::random(),
            spend_authority: TransparentSpendAuthority::opt_random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::test_envelope_roundtrip;

    test_envelope_roundtrip!(Address);
}
