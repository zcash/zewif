use minicbor::{Decode, Encode};

use crate::{BlockHash, BlockHeight, blob};

blob!(MerkleNode, 32, "A node hash in a note commitment tree.");
crate::blob_hex!(MerkleNode, forward);
impl Copy for MerkleNode {}

/// The state of a note commitment tree as of some block (mirrors
/// incrementalmerkletree's `Frontier`).
///
/// An unknown frontier is represented by a containing `Option` being `None`
/// — `Frontier::Empty` specifically means an empty tree.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Frontier {
    /// The tree contains no note commitments as of the reference block.
    #[n(0)]
    Empty,
    #[n(1)]
    NonEmpty(#[n(0)] FrontierData),
}

/// The contents of a non-empty note commitment tree frontier.
///
/// The number of ommers is fully determined by the position; this invariant
/// is documented, not enforced.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct FrontierData {
    /// 0-based index of the most recently appended leaf.
    #[n(0)]
    position: u64,
    /// The most recently appended leaf value.
    #[n(1)]
    leaf: MerkleNode,
    /// Hashes of the roots of completed left subtrees, in
    /// leaf-to-root order.
    #[n(2)]
    ommers: Vec<MerkleNode>,
}

impl FrontierData {
    pub fn from_parts(position: u64, leaf: MerkleNode, ommers: Vec<MerkleNode>) -> Self {
        Self {
            position,
            leaf,
            ommers,
        }
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn leaf(&self) -> &MerkleNode {
        &self.leaf
    }

    pub fn ommers(&self) -> &[MerkleNode] {
        &self.ommers
    }
}

/// Note commitment tree state at the end of a particular block, from which
/// an importing wallet can begin scanning without processing earlier blocks.
///
/// Maps to zcash_client_backend `ChainState`; frontiers for future shielded
/// pools will be added as further fields.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct ChainState {
    /// The block whose end-state this describes.
    #[n(0)]
    height: BlockHeight,
    /// Hash of the block at that height.
    #[n(1)]
    block_hash: Option<BlockHash>,
    /// Sapling frontier; `None` = unknown.
    #[n(2)]
    sapling_tree: Option<Frontier>,
    /// Orchard frontier; `None` = unknown.
    #[n(3)]
    orchard_tree: Option<Frontier>,
}

impl ChainState {
    pub fn new(height: BlockHeight) -> Self {
        Self {
            height,
            block_hash: None,
            sapling_tree: None,
            orchard_tree: None,
        }
    }

    pub fn height(&self) -> BlockHeight {
        self.height
    }

    pub fn block_hash(&self) -> Option<BlockHash> {
        self.block_hash
    }

    pub fn set_block_hash(&mut self, hash: BlockHash) {
        self.block_hash = Some(hash);
    }

    pub fn sapling_tree(&self) -> Option<&Frontier> {
        self.sapling_tree.as_ref()
    }

    pub fn set_sapling_tree(&mut self, frontier: Frontier) {
        self.sapling_tree = Some(frontier);
    }

    pub fn orchard_tree(&self) -> Option<&Frontier> {
        self.orchard_tree.as_ref()
    }

    pub fn set_orchard_tree(&mut self, frontier: Frontier) {
        self.orchard_tree = Some(frontier);
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockHash, BlockHeight, MerkleNode, RandomInstance, test_cbor_roundtrip};

    use super::{ChainState, Frontier, FrontierData};

    impl RandomInstance for Frontier {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.3) {
                Frontier::Empty
            } else {
                let num_ommers = rng.random_range(0..5usize);
                Frontier::NonEmpty(FrontierData {
                    position: rng.random_range(0..(1u64 << 40)),
                    leaf: MerkleNode::random(),
                    ommers: (0..num_ommers).map(|_| MerkleNode::random()).collect(),
                })
            }
        }
    }

    impl RandomInstance for ChainState {
        fn random() -> Self {
            Self {
                height: BlockHeight::random(),
                block_hash: BlockHash::opt_random(),
                sapling_tree: Frontier::opt_random(),
                orchard_tree: Frontier::opt_random(),
            }
        }
    }

    test_cbor_roundtrip!(Frontier, test_frontier_cbor);
    test_cbor_roundtrip!(ChainState, test_chain_state_cbor);
}
