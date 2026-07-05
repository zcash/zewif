//! Minimal Sprout protocol types for legacy wallet migration.
//!
//! Sprout is the original shielded protocol in Zcash, now deprecated.
//! These types store Sprout key data in its canonical encodings,
//! sufficient for preserving Sprout wallet data during migration from
//! zcashd.

use minicbor::{Decode, Encode};

use crate::blob;

blob!(
    SproutViewingKey,
    67,
    "A Sprout viewing key in its canonical encoding: a 3-byte version prefix
followed by a_pk and sk_enc. It is sufficient to detect incoming Sprout
notes."
);
crate::blob_encoding!(SproutViewingKey, bytes);

blob!(
    SproutSpendingKey,
    34,
    "A Sprout spending key in its canonical encoding: a 2-byte network
version prefix followed by a_sk. The spending key can derive the viewing
key and payment address."
);
crate::blob_encoding!(SproutSpendingKey, redacted);

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
