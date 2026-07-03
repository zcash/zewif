use minicbor::{Decode, Encode};

use crate::BlockHash;

/// The unique identifier of a transaction on the blockchain in terms of the hash of the block that
/// includes it and the index of the transaction within the block.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TxBlockPosition {
    /// The hash of the block containing the transaction.
    #[n(0)]
    block_hash: BlockHash,
    /// The 0-based index of the transaction within the block.
    #[n(1)]
    index: u32,
}

impl TxBlockPosition {
    pub fn new(block_hash: BlockHash, index: u32) -> Self {
        Self { block_hash, index }
    }

    pub fn block_hash(&self) -> &BlockHash {
        &self.block_hash
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockHash, test_cbor_roundtrip};

    use super::TxBlockPosition;

    impl crate::RandomInstance for TxBlockPosition {
        fn random() -> Self {
            Self {
                block_hash: BlockHash::random(),
                index: u32::random(),
            }
        }
    }

    test_cbor_roundtrip!(TxBlockPosition);
}
