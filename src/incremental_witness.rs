/// A Merkle authentication path proving a note commitment's inclusion in the
/// commitment tree, along with the tree state at the time it was computed.
///
/// Without valid witness data, unspent shielded notes cannot be spent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncrementalWitness<const DEPTH: usize, Node> {
    note_commitment: Node,
    note_position: u32,
    merkle_path: Vec<Node>,
    anchor: Node,
    anchor_tree_size: u32,
    anchor_frontier: Vec<Node>,
}

impl<const DEPTH: usize, Node> IncrementalWitness<DEPTH, Node> {
    /// Constructs an incremental witness from its constituent parts.
    pub fn from_parts(
        note_commitment: Node,
        note_position: u32,
        merkle_path: Vec<Node>,
        anchor: Node,
        anchor_tree_size: u32,
        anchor_frontier: Vec<Node>,
    ) -> Self {
        Self {
            note_commitment,
            note_position,
            merkle_path,
            anchor,
            anchor_tree_size,
            anchor_frontier,
        }
    }

    /// The note commitment that this witness provides an inclusion proof for.
    pub fn note_commitment(&self) -> &Node {
        &self.note_commitment
    }

    /// The position (index) of the note commitment in the note commitment tree.
    pub fn note_position(&self) -> u32 {
        self.note_position
    }

    /// The Merkle path to the note commitment that produces given anchor.
    ///
    /// Ordered from leaf to root. As Zcash witnesses are computed by hashing with empty nodes on
    /// on the right-hand margin of the tree, this witness will contain data that is the result of
    /// such hashing. Such nodes must be discarded in order to update the witness.
    pub fn merkle_path(&self) -> &[Node] {
        &self.merkle_path[..]
    }

    /// The anchor for which this witness's merkle path is valid.
    ///
    /// Note that in the case of a reorg, this may not correspond to any anchor in the main chain.
    pub fn anchor(&self) -> &Node {
        &self.anchor
    }

    /// The size of the note commitment tree as of the given anchor. This can be used to determine
    /// which of the witness nodes include dummy hashes so that these nodes may be discarded when
    /// restoring the witness in a new wallet.
    pub fn anchor_tree_size(&self) -> u32 {
        self.anchor_tree_size
    }

    /// The `frontier` of the note commitment tree as of the anchor tree size.
    ///
    /// Ordered from leaf to root. If the anchor corresponds to a stable anchor in the main chain,
    /// then these frontier nodes also correspond to stable nodes in the note commitment tree and
    /// can be used as a starting point for updating the witness, obviating the need to .
    pub fn anchor_frontier(&self) -> &[Node] {
        &self.anchor_frontier
    }
}

#[cfg(test)]
mod tests {
    use bc_rand::rng_next_with_upper_bound;

    use super::IncrementalWitness;
    use crate::RandomInstance;

    impl<const DEPTH: usize, Node: RandomInstance> RandomInstance for IncrementalWitness<DEPTH, Node> {
        fn random() -> Self {
            let mut rng = bc_rand::thread_rng();
            let note_position = rng_next_with_upper_bound(&mut rng, u32::MAX / 4);
            let anchor_tree_size =
                note_position + rng_next_with_upper_bound(&mut rng, u32::MAX / 16);
            Self {
                note_commitment: Node::random(),
                note_position,
                merkle_path: Vec::random(), // TODO: this should have DEPTH entries
                anchor: Node::random(),
                anchor_tree_size,
                anchor_frontier: Vec::random(),
            }
        }
    }
}
