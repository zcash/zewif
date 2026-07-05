use minicbor::{Decode, Encode};

use crate::{
    ironwood::IronwoodSentOutput, orchard::OrchardSentOutput, sapling::SaplingSentOutput,
    transparent::TransparentSentOutput,
};

/// A sent output from a transaction, tagged by pool.
///
/// Groups the pool-specific sent output types into a single enum for
/// uniform storage in the txid-grouped sent output map.
///
/// No Sprout variant is defined: Sprout provides no outgoing-viewing-key
/// mechanism by which a sender could later recover its non-change outputs.
///
/// This enum is non-exhaustive because future network upgrades may add
/// pools.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[non_exhaustive]
pub enum SentOutput {
    #[n(0)]
    Transparent(#[n(0)] TransparentSentOutput),
    #[n(1)]
    Sapling(#[n(0)] SaplingSentOutput),
    #[n(2)]
    Orchard(#[n(0)] OrchardSentOutput),
    #[n(3)]
    Ironwood(#[n(0)] IronwoodSentOutput),
}

impl SentOutput {
    /// Returns the index of the underlying sent output within its pool's
    /// output list in the transaction.
    pub fn output_index(&self) -> u32 {
        match self {
            SentOutput::Transparent(o) => o.output_index(),
            SentOutput::Sapling(o) => o.output_index(),
            SentOutput::Orchard(o) => o.output_index(),
            SentOutput::Ironwood(o) => o.output_index(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::SentOutput;

    impl crate::RandomInstance for SentOutput {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..4u32) {
                0 => SentOutput::Transparent(crate::transparent::TransparentSentOutput::random()),
                1 => SentOutput::Sapling(crate::sapling::SaplingSentOutput::random()),
                2 => SentOutput::Orchard(crate::orchard::OrchardSentOutput::random()),
                _ => SentOutput::Ironwood(crate::ironwood::IronwoodSentOutput::random()),
            }
        }
    }

    test_cbor_roundtrip!(SentOutput);
}
