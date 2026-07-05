use bc_envelope::prelude::*;

use crate::{IncrementalWitness, blob, blob_envelope};

const ORCHARD_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashOrchard,
    32,
    "A node in the Orchard note commitment tree."
);
impl Copy for MerkleHashOrchard {}

blob_envelope!(MerkleHashOrchard);

/// An incremental witness for an Orchard note commitment, proving its
/// inclusion in the global note commitment tree. Required to spend the note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrchardWitness(IncrementalWitness<ORCHARD_COMMITMENT_TREE_DEPTH, MerkleHashOrchard>);

impl OrchardWitness {
    /// The position of the witnessed note commitment in the note
    /// commitment tree.
    pub fn note_position(&self) -> u32 {
        self.0.note_position()
    }
}

impl From<OrchardWitness> for Envelope {
    fn from(value: OrchardWitness) -> Self {
        Envelope::new(*value.0.note_commitment())
            .add_type("OrchardWitness")
            .add_assertion("note_position", value.0.note_position())
            .add_assertion("merkle_path", value.0.merkle_path().to_vec())
            .add_assertion("anchor", *value.0.anchor())
            .add_assertion("anchor_tree_size", value.0.anchor_tree_size())
            .add_assertion("anchor_frontier", value.0.anchor_frontier().to_vec())
    }
}

impl TryFrom<Envelope> for OrchardWitness {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("OrchardWitness")?;
        let note_commitment = envelope.extract_subject::<MerkleHashOrchard>()?;
        let note_position = envelope.extract_object_for_predicate("note_position")?;
        let merkle_path = envelope.extract_object_for_predicate("merkle_path")?;
        let anchor = envelope.extract_object_for_predicate("anchor")?;
        let anchor_tree_size = envelope.extract_object_for_predicate("anchor_tree_size")?;
        let anchor_frontier = envelope.extract_object_for_predicate("anchor_frontier")?;
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

    use super::OrchardWitness;

    impl RandomInstance for OrchardWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_envelope_roundtrip!(OrchardWitness);
}
