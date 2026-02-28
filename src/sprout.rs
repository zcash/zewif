//! Minimal Sprout protocol types for legacy wallet migration.
//!
//! Sprout is the original shielded protocol in Zcash, now deprecated.
//! These types store Sprout key data as opaque bytes, sufficient for
//! preserving Sprout wallet data during migration from zcashd.

use bc_envelope::prelude::*;

use crate::Data;

/// A Sprout spending key in its canonical encoding.
///
/// Sprout did not have a separate viewing key type; the spending key
/// is needed to detect incoming notes. Stored as opaque bytes.
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

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, Data, RandomInstance};

    use super::SproutSpendingKey;

    impl RandomInstance for SproutSpendingKey {
        fn random() -> Self {
            SproutSpendingKey::new(Data::random())
        }
    }

    test_envelope_roundtrip!(SproutSpendingKey);
}
