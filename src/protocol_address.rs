use crate::{UnifiedAddress, sapling, sprout, transparent};
use minicbor::{Decode, Encode};

/// A protocol-specific Zcash address.
///
/// Distinguishes between the address protocols supported in Zcash:
/// transparent (t-), Sprout (zc-), Sapling (zs-), and unified (u-).
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum ProtocolAddress {
    /// A transparent address (t-address).
    #[n(0)]
    Transparent(#[n(0)] transparent::Address),

    /// A Sprout shielded address (zc-address, legacy).
    #[n(1)]
    Sprout(#[n(0)] sprout::SproutAddress),

    /// A Sapling shielded address (zs-address).
    #[n(2)]
    Sapling(#[n(0)] Box<sapling::Address>),

    /// A unified address (u-address) containing multiple receiver types.
    #[n(3)]
    Unified(#[n(0)] Box<UnifiedAddress>),
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

#[cfg(test)]
mod tests {
    use super::ProtocolAddress;
    use crate::{UnifiedAddress, sapling, sprout, test_cbor_roundtrip, transparent};

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

    test_cbor_roundtrip!(ProtocolAddress);
}
