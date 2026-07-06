use minicbor::{Decode, Encode};

use crate::{IncrementalWitness, blob};

const ORCHARD_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashOrchard,
    32,
    "A node in the Orchard note commitment tree."
);
crate::blob_encoding!(MerkleHashOrchard, bytes);
impl Copy for MerkleHashOrchard {}

/// An incremental witness for an Orchard note commitment, proving its
/// inclusion in the global note commitment tree. Required to spend the note.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cbor(transparent)]
pub struct OrchardWitness(
    #[n(0)] IncrementalWitness<ORCHARD_COMMITMENT_TREE_DEPTH, MerkleHashOrchard>,
);

impl OrchardWitness {
    /// The position of the witnessed note commitment in the note
    /// commitment tree.
    pub fn note_position(&self) -> u32 {
        self.0.note_position()
    }
}

impl From<IncrementalWitness<ORCHARD_COMMITMENT_TREE_DEPTH, MerkleHashOrchard>> for OrchardWitness {
    fn from(witness: IncrementalWitness<ORCHARD_COMMITMENT_TREE_DEPTH, MerkleHashOrchard>) -> Self {
        Self(witness)
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncrementalWitness, RandomInstance, test_cbor_roundtrip};

    use super::{MerkleHashOrchard, OrchardWitness};

    impl RandomInstance for OrchardWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_cbor_roundtrip!(OrchardWitness);
    test_cbor_roundtrip!(MerkleHashOrchard, test_merkle_hash_orchard);
}
