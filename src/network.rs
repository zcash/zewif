use std::collections::BTreeMap;

use minicbor::{Decode, Encode};

/// The Zcash network a wallet belongs to: mainnet, testnet, or regtest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Encode, Decode)]
#[cbor(flat)]
pub enum Network {
    #[n(0)]
    Mainnet,
    #[n(1)]
    Testnet,
    #[n(2)]
    Regtest(#[n(0)] RegtestParams),
}

/// The network-upgrade activation schedule that defines a regtest network.
///
/// Regtest networks vary in their activation schedules, and wallet data
/// recorded against one schedule is in general incompatible with a chain
/// using another. Importers should refuse or flag data whose activation
/// schedule does not match the chain they operate against.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Encode, Decode)]
#[cbor(map)]
pub struct RegtestParams {
    /// Maps each consensus branch ID (as defined by ZIP 200 and successors)
    /// to the activation height of the corresponding network upgrade.
    #[n(0)]
    activations: BTreeMap<u32, u32>,
}

impl RegtestParams {
    pub fn new(activations: BTreeMap<u32, u32>) -> Self {
        Self { activations }
    }

    pub fn activations(&self) -> &BTreeMap<u32, u32> {
        &self.activations
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::{Network, RegtestParams};

    impl crate::RandomInstance for RegtestParams {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let activations = (0..rng.random_range(0..4usize))
                .map(|_| (rng.random::<u32>(), rng.random::<u32>()))
                .collect();
            Self { activations }
        }
    }

    impl crate::RandomInstance for Network {
        fn random() -> Self {
            match rand::random::<u8>() % 3 {
                0 => Network::Mainnet,
                1 => Network::Testnet,
                _ => Network::Regtest(RegtestParams::random()),
            }
        }
    }

    test_cbor_roundtrip!(Network);

    /// Tagged unions encode as `[variant-id, body?]`: a one-element array
    /// for payload-free variants, identifier plus a single body record for
    /// data-bearing ones.
    #[test]
    fn union_wire_shape() {
        assert_eq!(minicbor::to_vec(Network::Mainnet).unwrap(), [0x81, 0x00]);
        assert_eq!(minicbor::to_vec(Network::Testnet).unwrap(), [0x81, 0x01]);
        let regtest = Network::Regtest(RegtestParams::new([(0xc2d6d0b4, 1)].into()));
        assert_eq!(
            minicbor::to_vec(&regtest).unwrap(),
            // [2, {0: {0xc2d6d0b4: 1}}]
            [
                0x82, 0x02, 0xa1, 0x00, 0xa1, 0x1a, 0xc2, 0xd6, 0xd0, 0xb4, 0x01
            ]
        );
    }
}
