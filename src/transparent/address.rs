use super::TransparentSpendAuthority;
use super::TransparentPubKey;
use crate::Script;
use minicbor::{Decode, Encode};

/// A transparent Zcash address (t-address).
///
/// Contains the address string and optionally how the key was obtained
/// (HD-derived with derivation info, or imported).
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Address {
    #[n(0)]
    address: String,
    /// How the key for this address was obtained, if known.
    #[n(1)]
    spend_authority: Option<TransparentSpendAuthority>,
    /// The secp256k1 public key (33 or 65 bytes) for a watch-only P2PK/P2PKH
    /// address imported without its private key (zcashd importpubkey). Omit
    /// when spend authority is present, since the public key is then
    /// derivable.
    #[n(2)]
    pubkey: Option<TransparentPubKey>,
    /// The redeem script for a watch-only P2SH address imported by script
    /// (zcashd importaddress). A watch-only entry imported by bare address
    /// string carries neither pubkey nor redeem_script.
    #[n(3)]
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

    pub fn pubkey(&self) -> Option<&TransparentPubKey> {
        self.pubkey.as_ref()
    }

    pub fn set_pubkey(&mut self, pubkey: TransparentPubKey) {
        self.pubkey = Some(pubkey);
    }

    pub fn redeem_script(&self) -> Option<&Script> {
        self.redeem_script.as_ref()
    }

    pub fn set_redeem_script(&mut self, redeem_script: Script) {
        self.redeem_script = Some(redeem_script);
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            address: String::random(),
            spend_authority: super::TransparentSpendAuthority::opt_random(),
            pubkey: crate::transparent::TransparentPubKey::opt_random(),
            redeem_script: crate::Script::opt_random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(Address);
}
