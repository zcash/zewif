//! Minimal Sprout protocol types for legacy wallet migration.
//!
//! Sprout is the original shielded protocol in Zcash, now deprecated.
//! Sprout keys are stored in their canonical Base58Check text encodings,
//! sufficient for preserving Sprout wallet data during migration from
//! zcashd.

use minicbor::{Decode, Encode};

use crate::text_key;

text_key!(
    SproutViewingKey,
    "A Sprout viewing key in its canonical Base58Check encoding (mainnet
strings begin with \"ZiVK\", testnet with \"ZiVt\"). Sufficient to detect
incoming Sprout notes.",
    "ZiVK"
);

text_key!(
    SproutSpendingKey,
    "A Sprout spending key in its canonical Base58Check encoding (mainnet
strings begin with \"SK\", testnet with \"ST\"). The spending key can derive
the viewing key and payment address.",
    "SK",
    redacted
);

/// A Sprout shielded address (zc-address).
///
/// Stored as the address string. No internal structure is parsed;
/// the importing wallet either understands Sprout or it doesn't.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SproutAddress {
    #[n(0)]
    address: String,
}

impl SproutAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
impl crate::RandomInstance for SproutAddress {
    fn random() -> Self {
        SproutAddress::new(String::random())
    }
}

#[cfg(test)]
mod tests {
    use super::{SproutAddress, SproutSpendingKey, SproutViewingKey};
    use crate::test_cbor_roundtrip;

    test_cbor_roundtrip!(SproutViewingKey);
    test_cbor_roundtrip!(SproutSpendingKey, test_sprout_spending_key);
    test_cbor_roundtrip!(SproutAddress, test_sprout_address);
}
