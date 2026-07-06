use crate::{BlockHeight, Extensions, KeyScope};
use minicbor::{Decode, Encode};

use super::ProtocolAddress;

/// A wallet address wrapping a protocol-specific address.
///
/// This is the entry in an account's address list, pairing the
/// protocol-specific address data with wallet metadata about it.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Address {
    #[n(0)]
    address: ProtocolAddress,
    /// The role of the address within its account. For HD-derived transparent
    /// addresses this must be consistent with the derivation change component;
    /// it is the authoritative scope for shielded and unified addresses.
    #[n(1)]
    scope: Option<KeyScope>,
    /// The block height at or around which this address was first exposed to
    /// a user or counterparty. Not recoverable from the chain; importers use
    /// it for gap-limit reasoning (maps to zcash_client_sqlite
    /// addresses.exposed_at_height). None = never exposed — e.g. zcashd
    /// keypool keys, which are pre-generated and not yet handed out.
    #[n(2)]
    exposed_at_height: Option<BlockHeight>,
    #[cbor(n(3), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
}

impl Address {
    pub fn new(address: ProtocolAddress) -> Self {
        Self {
            address,
            scope: None,
            exposed_at_height: None,
            extensions: Extensions::new(),
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

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockHeight, Extensions, KeyScope, ProtocolAddress, test_cbor_roundtrip};

    use super::Address;

    impl crate::RandomInstance for Address {
        fn random() -> Self {
            Self {
                address: ProtocolAddress::random(),
                scope: KeyScope::opt_random(),
                exposed_at_height: BlockHeight::opt_random(),
                extensions: Extensions::random(),
            }
        }
    }

    test_cbor_roundtrip!(Address);
}
