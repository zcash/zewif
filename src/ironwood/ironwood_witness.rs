use minicbor::{Decode, Encode};

use crate::{IncrementalWitness, blob};

const IRONWOOD_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashIronwood,
    32,
    "A node in the Ironwood note commitment tree."
);
crate::blob_hex!(MerkleHashIronwood, forward);
impl Copy for MerkleHashIronwood {}

/// An incremental witness for an Ironwood note commitment, proving its
/// inclusion in the global note commitment tree. Required to spend the note.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cbor(transparent)]
pub struct IronwoodWitness(
    #[n(0)] IncrementalWitness<IRONWOOD_COMMITMENT_TREE_DEPTH, MerkleHashIronwood>,
);

impl IronwoodWitness {
    /// The position of the witnessed note commitment in the note
    /// commitment tree.
    pub fn note_position(&self) -> u32 {
        self.0.note_position()
    }
}

impl From<IncrementalWitness<IRONWOOD_COMMITMENT_TREE_DEPTH, MerkleHashIronwood>>
    for IronwoodWitness
{
    fn from(
        witness: IncrementalWitness<IRONWOOD_COMMITMENT_TREE_DEPTH, MerkleHashIronwood>,
    ) -> Self {
        Self(witness)
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncrementalWitness, RandomInstance, test_cbor_roundtrip};

    use super::{IronwoodWitness, MerkleHashIronwood};

    impl RandomInstance for IronwoodWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_cbor_roundtrip!(IronwoodWitness);
    test_cbor_roundtrip!(MerkleHashIronwood, test_merkle_hash_ironwood);
}
