use super::TransparentSpendAuthority;
use crate::{Data, Script};
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
    /// The secp256k1 public key (33 or 65 bytes) for a watch-only P2PK/P2PKH
    /// address imported without its private key (zcashd importpubkey). Omit
    /// when spend authority is present, since the public key is then
    /// derivable.
    pubkey: Option<Data>,
    /// The redeem script for a watch-only P2SH address imported by script
    /// (zcashd importaddress). A watch-only entry imported by bare address
    /// string carries neither pubkey nor redeem_script.
    redeem_script: Option<Script>,
}

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Address {
            address: address.into(),
            spend_authority: None,
            pubkey: None,
            redeem_script: None,
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

    pub fn pubkey(&self) -> Option<&Data> {
        self.pubkey.as_ref()
    }

    pub fn set_pubkey(&mut self, pubkey: Data) {
        self.pubkey = Some(pubkey);
    }

    pub fn redeem_script(&self) -> Option<&Script> {
        self.redeem_script.as_ref()
    }

    pub fn set_redeem_script(&mut self, redeem_script: Script) {
        self.redeem_script = Some(redeem_script);
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        Envelope::new(value.address)
            .add_type("TransparentAddress")
            .add_optional_assertion("spend_authority", value.spend_authority)
            .add_optional_assertion("pubkey", value.pubkey)
            .add_optional_assertion("redeem_script", value.redeem_script)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransparentAddress")?;
        let address = envelope.extract_subject()?;
        let spend_authority = envelope.try_optional_object_for_predicate("spend_authority")?;
        let pubkey = envelope.extract_optional_object_for_predicate("pubkey")?;
        let redeem_script = envelope.extract_optional_object_for_predicate("redeem_script")?;
        Ok(Address {
            address,
            spend_authority,
            pubkey,
            redeem_script,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            address: String::random(),
            spend_authority: TransparentSpendAuthority::opt_random(),
            pubkey: crate::Data::opt_random_with_size(33),
            redeem_script: crate::Script::opt_random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::test_envelope_roundtrip;

    test_envelope_roundtrip!(Address);
}
