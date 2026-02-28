use bc_envelope::prelude::*;

use crate::{IncrementalWitness, blob, blob_envelope};

const SAPLING_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashSapling,
    32,
    "A node in the Sapling note commitment tree."
);
impl Copy for MerkleHashSapling {}

blob_envelope!(MerkleHashSapling);

/// An incremental witness for a Sapling note commitment, proving its
/// inclusion in the global note commitment tree. Required to spend the note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaplingWitness(
    IncrementalWitness<SAPLING_COMMITMENT_TREE_DEPTH, MerkleHashSapling>,
);

impl From<SaplingWitness> for Envelope {
    fn from(value: SaplingWitness) -> Self {
        Envelope::new(*value.0.note_commitment())
            .add_type("SaplingWitness")
            .add_assertion("note_position", value.0.note_position())
            .add_assertion("merkle_path", value.0.merkle_path().to_vec())
            .add_assertion("anchor", *value.0.anchor())
            .add_assertion("anchor_tree_size", value.0.anchor_tree_size())
            .add_assertion(
                "anchor_frontier",
                value.0.anchor_frontier().to_vec(),
            )
    }
}

impl TryFrom<Envelope> for SaplingWitness {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SaplingWitness")?;
        let note_commitment =
            envelope.extract_subject::<MerkleHashSapling>()?;
        let note_position =
            envelope.extract_object_for_predicate("note_position")?;
        let merkle_path =
            envelope.extract_object_for_predicate("merkle_path")?;
        let anchor = envelope.extract_object_for_predicate("anchor")?;
        let anchor_tree_size =
            envelope.extract_object_for_predicate("anchor_tree_size")?;
        let anchor_frontier =
            envelope.extract_object_for_predicate("anchor_frontier")?;
        Ok(Self(IncrementalWitness::from_parts(
            note_commitment,
            note_position,
            merkle_path,
            anchor,
            anchor_tree_size,
            anchor_frontier,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncrementalWitness, RandomInstance, test_envelope_roundtrip};

    use super::SaplingWitness;

    impl RandomInstance for SaplingWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_envelope_roundtrip!(SaplingWitness);
}
