use minicbor::{Decode, Encode};

use crate::BlockHeight;

/// A contiguous range of fully-scanned block heights (inclusive on both ends).
///
/// Used to track which portions of the blockchain have been scanned for a
/// given account, allowing an importing wallet to skip already-scanned ranges
/// and focus on gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct ScanRange {
    #[n(0)]
    start: BlockHeight,
    #[n(1)]
    end: BlockHeight,
}

impl ScanRange {
    pub fn new(start: BlockHeight, end: BlockHeight) -> Self {
        assert!(start <= end, "ScanRange start must not exceed end");
        Self { start, end }
    }

    pub fn start(&self) -> BlockHeight {
        self.start
    }

    pub fn end(&self) -> BlockHeight {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockHeight, test_cbor_roundtrip};

    use super::ScanRange;

    impl crate::RandomInstance for ScanRange {
        fn random() -> Self {
            let a = BlockHeight::random();
            let b = BlockHeight::random();
            if a <= b {
                Self { start: a, end: b }
            } else {
                Self { start: b, end: a }
            }
        }
    }

    test_cbor_roundtrip!(ScanRange);
}
