use crate::{
    UnifiedAddress,
    error::Error,
    sapling, sprout, transparent,
};
use bc_envelope::prelude::*;

/// A protocol-specific Zcash address.
///
/// Distinguishes between the address protocols supported in Zcash:
/// transparent (t-), Sprout (zc-), Sapling (zs-), and unified (u-).
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolAddress {
    /// A transparent address (t-address).
    Transparent(transparent::Address),

    /// A Sprout shielded address (zc-address, legacy).
    Sprout(sprout::SproutAddress),

    /// A Sapling shielded address (zs-address).
    Sapling(Box<sapling::Address>),

    /// A unified address (u-address) containing multiple receiver types.
    Unified(Box<UnifiedAddress>),
}

impl ProtocolAddress {
    /// Returns the address in its canonical string encoding.
    pub fn as_string(&self) -> String {
        match self {
            ProtocolAddress::Transparent(addr) => addr.address().to_string(),
            ProtocolAddress::Sprout(addr) => addr.address().to_string(),
            ProtocolAddress::Sapling(addr) => addr.address().to_string(),
            ProtocolAddress::Unified(addr) => addr.address().to_string(),
        }
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, ProtocolAddress::Transparent(_))
    }

    pub fn is_sprout(&self) -> bool {
        matches!(self, ProtocolAddress::Sprout(_))
    }

    pub fn is_sapling(&self) -> bool {
        matches!(self, ProtocolAddress::Sapling(_))
    }

    pub fn is_unified(&self) -> bool {
        matches!(self, ProtocolAddress::Unified(_))
    }
}

impl From<ProtocolAddress> for Envelope {
    fn from(value: ProtocolAddress) -> Self {
        match value {
            ProtocolAddress::Transparent(addr) => addr.into(),
            ProtocolAddress::Sprout(addr) => addr.into(),
            ProtocolAddress::Sapling(addr) => (*addr).into(),
            ProtocolAddress::Unified(addr) => (*addr).into(),
        }
    }
}

impl TryFrom<Envelope> for ProtocolAddress {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        if envelope.has_type("TransparentAddress") {
            Ok(ProtocolAddress::Transparent(envelope.try_into()?))
        } else if envelope.has_type("SproutAddress") {
            Ok(ProtocolAddress::Sprout(envelope.try_into()?))
        } else if envelope.has_type("SaplingAddress") {
            Ok(ProtocolAddress::Sapling(Box::new(envelope.try_into()?)))
        } else if envelope.has_type("UnifiedAddress") {
            Ok(ProtocolAddress::Unified(Box::new(envelope.try_into()?)))
        } else {
            Err(Error::InvalidProtocolAddress.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProtocolAddress;
    use crate::{
        UnifiedAddress, sapling, sprout, test_envelope_roundtrip, transparent,
    };

    impl crate::RandomInstance for ProtocolAddress {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..4u32) {
                0 => ProtocolAddress::Transparent(transparent::Address::random()),
                1 => ProtocolAddress::Sprout(sprout::SproutAddress::random()),
                2 => ProtocolAddress::Sapling(Box::new(sapling::Address::random())),
                _ => ProtocolAddress::Unified(Box::new(UnifiedAddress::random())),
            }
        }
    }

    test_envelope_roundtrip!(ProtocolAddress);
}
