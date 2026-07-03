//! Minimal Sprout protocol types for legacy wallet migration.
//!
//! Sprout is the original shielded protocol in Zcash, now deprecated.
//! These types store Sprout key data as opaque bytes, sufficient for
//! preserving Sprout wallet data during migration from zcashd.

use minicbor::{Decode, Encode};

use crate::Data;

/// A Sprout viewing key in its canonical serialized encoding (64 bytes).
///
/// In zcashd this is `(a_pk, sk_enc)` — the paying key and the
/// receiving key. It is sufficient to detect incoming Sprout notes.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SproutViewingKey {
    #[n(0)]
    data: Data,
}

impl SproutViewingKey {
    pub fn new(data: Data) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

/// A Sprout spending key in its canonical encoding (32 bytes).
///
/// The spending key can derive the viewing key and payment address.
/// Stored separately from viewing data to support spending key
/// separability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SproutSpendingKey {
    data: Data,
}

impl SproutSpendingKey {
    pub fn new(data: Data) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl<C> minicbor::Encode<C> for SproutSpendingKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(self.data.as_ref())?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for SproutSpendingKey {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        Ok(Self::new(Data::from_slice(d.bytes()?)))
    }
}

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
impl crate::RandomInstance for SproutViewingKey {
    fn random() -> Self {
        SproutViewingKey::new(Data::random())
    }
}

#[cfg(test)]
impl crate::RandomInstance for SproutSpendingKey {
    fn random() -> Self {
        SproutSpendingKey::new(Data::random())
    }
}

#[cfg(test)]
impl crate::RandomInstance for SproutAddress {
    fn random() -> Self {
        SproutAddress::new(String::random())
    }
}

#[cfg(test)]
mod viewing_key_tests {
    use super::SproutViewingKey;
    use crate::test_cbor_roundtrip;
    test_cbor_roundtrip!(SproutViewingKey);
}

#[cfg(test)]
mod spending_key_tests {
    use super::SproutSpendingKey;
    use crate::test_cbor_roundtrip;
    test_cbor_roundtrip!(SproutSpendingKey);
}

#[cfg(test)]
mod address_tests {
    use super::SproutAddress;
    use crate::test_cbor_roundtrip;
    test_cbor_roundtrip!(SproutAddress);
}
