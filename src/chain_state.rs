use bc_envelope::prelude::*;

use crate::{Blob, BlockHash, BlockHeight};

/// The state of a note commitment tree as of some block (mirrors
/// incrementalmerkletree's `Frontier`).
///
/// The number of ommers is fully determined by the position; this invariant
/// is documented, not enforced. An unknown frontier is represented by a
/// containing `Option` being `None` — `Frontier::Empty` specifically means
/// an empty tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frontier {
    /// The tree contains no note commitments as of the reference block.
    Empty,
    NonEmpty {
        /// 0-based index of the most recently appended leaf.
        position: u64,
        /// The most recently appended leaf value.
        leaf: Blob<32>,
        /// Hashes of the roots of completed left subtrees, in
        /// leaf-to-root order.
        ommers: Vec<Blob<32>>,
    },
}

impl From<Frontier> for Envelope {
    fn from(value: Frontier) -> Self {
        match value {
            Frontier::Empty => Envelope::new("empty")
                .add_type("Frontier")
                .add_assertion("variant", "empty"),
            Frontier::NonEmpty {
                position,
                leaf,
                ommers,
            } => Envelope::new(position)
                .add_type("Frontier")
                .add_assertion("variant", "nonempty")
                .add_assertion("leaf", leaf)
                .add_assertion("ommers", ommers),
        }
    }
}

impl TryFrom<Envelope> for Frontier {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Frontier")?;
        let variant: String = envelope.extract_object_for_predicate("variant")?;
        match variant.as_str() {
            "empty" => Ok(Frontier::Empty),
            "nonempty" => {
                let position: u64 = envelope.extract_subject()?;
                let leaf: Blob<32> = envelope.extract_object_for_predicate("leaf")?;
                let ommers: Vec<Blob<32>> = envelope.extract_object_for_predicate("ommers")?;
                Ok(Frontier::NonEmpty {
                    position,
                    leaf,
                    ommers,
                })
            }
            other => Err(bc_envelope::Error::General(format!(
                "unknown Frontier variant: {}",
                other
            ))),
        }
    }
}

/// Note commitment tree state at the end of a particular block, from which
/// an importing wallet can begin scanning without processing earlier blocks.
///
/// Maps to zcash_client_backend `ChainState`; frontiers for future shielded
/// pools will be added as further fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainState {
    /// The block whose end-state this describes.
    height: BlockHeight,
    /// Hash of the block at that height.
    block_hash: Option<BlockHash>,
    /// Sapling frontier; `None` = unknown.
    sapling_tree: Option<Frontier>,
    /// Orchard frontier; `None` = unknown.
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

impl From<ChainState> for Envelope {
    fn from(value: ChainState) -> Self {
        Envelope::new(value.height)
            .add_type("ChainState")
            .add_optional_assertion("block_hash", value.block_hash)
            .add_optional_assertion("sapling_tree", value.sapling_tree)
            .add_optional_assertion("orchard_tree", value.orchard_tree)
    }
}

impl TryFrom<Envelope> for ChainState {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("ChainState")?;
        let height = envelope.extract_subject()?;
        let block_hash = envelope.extract_optional_object_for_predicate("block_hash")?;
        let sapling_tree = envelope.try_optional_object_for_predicate("sapling_tree")?;
        let orchard_tree = envelope.try_optional_object_for_predicate("orchard_tree")?;
        Ok(Self {
            height,
            block_hash,
            sapling_tree,
            orchard_tree,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blob, BlockHash, BlockHeight, RandomInstance, test_envelope_roundtrip};

    use super::{ChainState, Frontier};

    impl RandomInstance for Frontier {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.3) {
                Frontier::Empty
            } else {
                let num_ommers = rng.random_range(0..5usize);
                Frontier::NonEmpty {
                    position: rng.random_range(0..(1u64 << 40)),
                    leaf: Blob::random(),
                    ommers: (0..num_ommers).map(|_| Blob::random()).collect(),
                }
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

    test_envelope_roundtrip!(Frontier, 20, false, test_frontier_envelope);
    test_envelope_roundtrip!(ChainState, 20, false, test_chain_state_envelope);
}
