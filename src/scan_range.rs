use bc_envelope::prelude::*;

use crate::BlockHeight;

/// A contiguous range of fully-scanned block heights (inclusive on both ends).
///
/// Used to track which portions of the blockchain have been scanned for a
/// given account, allowing an importing wallet to skip already-scanned ranges
/// and focus on gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanRange {
    start: BlockHeight,
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

impl From<ScanRange> for Envelope {
    fn from(value: ScanRange) -> Self {
        Envelope::new(value.start)
            .add_type("ScanRange")
            .add_assertion("end", value.end)
    }
}

impl TryFrom<Envelope> for ScanRange {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("ScanRange")?;
        let start = envelope.extract_subject()?;
        let end = envelope.extract_object_for_predicate("end")?;
        Ok(Self { start, end })
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_envelope_roundtrip, BlockHeight};

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

    test_envelope_roundtrip!(ScanRange);
}
