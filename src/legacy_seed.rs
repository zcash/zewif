use minicbor::{Decode, Encode};

use crate::Data;

/// A raw pre-mnemonic HD seed.
///
/// The seed's ZIP 32 fingerprint is not stored here: the seed entry in the
/// secret store carries it, and it is derivable from the seed bytes.
#[derive(Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct LegacySeed {
    #[n(0)]
    seed_data: Data,
}

impl std::fmt::Debug for LegacySeed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegacySeed")
            .field("seed_data", &"<elided>".to_string())
            .finish()
    }
}

impl LegacySeed {
    pub fn new(seed_data: Data) -> Self {
        Self { seed_data }
    }

    pub fn seed_data(&self) -> &Data {
        &self.seed_data
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, test_cbor_roundtrip};

    use super::LegacySeed;

    impl crate::RandomInstance for LegacySeed {
        fn random() -> Self {
            Self {
                seed_data: Data::random(),
            }
        }
    }

    test_cbor_roundtrip!(LegacySeed);
}
