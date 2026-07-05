use minicbor::{Decode, Encode};

use crate::{IncrementalWitness, blob};

const SAPLING_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashSapling,
    32,
    "A node in the Sapling note commitment tree."
);
crate::blob_encoding!(MerkleHashSapling, bytes);
impl Copy for MerkleHashSapling {}

/// An incremental witness for a Sapling note commitment, proving its
/// inclusion in the global note commitment tree. Required to spend the note.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cbor(transparent)]
pub struct SaplingWitness(
    #[n(0)] IncrementalWitness<SAPLING_COMMITMENT_TREE_DEPTH, MerkleHashSapling>,
);

impl SaplingWitness {
    /// The position of the witnessed note commitment in the note
    /// commitment tree.
    pub fn note_position(&self) -> u32 {
        self.0.note_position()
    }
}

impl From<IncrementalWitness<SAPLING_COMMITMENT_TREE_DEPTH, MerkleHashSapling>> for SaplingWitness {
    fn from(witness: IncrementalWitness<SAPLING_COMMITMENT_TREE_DEPTH, MerkleHashSapling>) -> Self {
        Self(witness)
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncrementalWitness, RandomInstance, test_cbor_roundtrip};

    use super::{MerkleHashSapling, SaplingWitness};

    impl RandomInstance for SaplingWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_cbor_roundtrip!(SaplingWitness);
    test_cbor_roundtrip!(MerkleHashSapling, test_merkle_hash_sapling);
}
