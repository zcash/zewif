//! Minimal Sprout protocol types for legacy wallet migration.
//!
//! Sprout is the original shielded protocol in Zcash, now deprecated.
//! These types store Sprout key data as opaque bytes, sufficient for
//! preserving Sprout wallet data during migration from zcashd.

use bc_envelope::prelude::*;

use crate::Data;

/// A Sprout viewing key in its canonical serialized encoding (64 bytes).
///
/// In zcashd this is `(a_pk, sk_enc)` — the paying key and the
/// receiving key. It is sufficient to detect incoming Sprout notes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SproutViewingKey {
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

impl From<SproutViewingKey> for Envelope {
    fn from(value: SproutViewingKey) -> Self {
        Envelope::new(value.data)
            .add_type("SproutViewingKey")
    }
}

impl TryFrom<Envelope> for SproutViewingKey {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SproutViewingKey")?;
        let data: Data = envelope.extract_subject()?;
        Ok(Self { data })
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

impl From<SproutSpendingKey> for Envelope {
    fn from(value: SproutSpendingKey) -> Self {
        Envelope::new(value.data)
            .add_type("SproutSpendingKey")
    }
}

impl TryFrom<Envelope> for SproutSpendingKey {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SproutSpendingKey")?;
        let data: Data = envelope.extract_subject()?;
        Ok(Self { data })
    }
}

/// A Sprout shielded address (zc-address).
///
/// Stored as the address string. No internal structure is parsed;
/// the importing wallet either understands Sprout or it doesn't.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SproutAddress {
    address: String,
}

impl SproutAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into() }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

impl From<SproutAddress> for Envelope {
    fn from(value: SproutAddress) -> Self {
        Envelope::new(value.address)
            .add_type("SproutAddress")
    }
}

impl TryFrom<Envelope> for SproutAddress {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SproutAddress")?;
        let address: String = envelope.extract_subject()?;
        Ok(Self { address })
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
    use crate::test_envelope_roundtrip;
    use super::SproutViewingKey;
    test_envelope_roundtrip!(SproutViewingKey);
}

#[cfg(test)]
mod spending_key_tests {
    use crate::test_envelope_roundtrip;
    use super::SproutSpendingKey;
    test_envelope_roundtrip!(SproutSpendingKey);
}

#[cfg(test)]
mod address_tests {
    use crate::test_envelope_roundtrip;
    use super::SproutAddress;
    test_envelope_roundtrip!(SproutAddress);
}
