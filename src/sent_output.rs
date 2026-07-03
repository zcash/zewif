use bc_envelope::prelude::*;

use crate::{
    Indexed,
    orchard::OrchardSentOutput,
    sapling::SaplingSentOutput,
    transparent::TransparentSentOutput,
};

/// A sent output from a transaction, tagged by pool.
///
/// Groups the pool-specific sent output types into a single enum for
/// uniform storage in the txid-grouped sent output map.
#[derive(Debug, Clone, PartialEq)]
pub enum SentOutput {
    Transparent(TransparentSentOutput),
    Sapling(SaplingSentOutput),
    Orchard(OrchardSentOutput),
}

impl SentOutput {
    /// Returns the index of the underlying sent output.
    pub fn index(&self) -> usize {
        match self {
            SentOutput::Transparent(o) => o.index(),
            SentOutput::Sapling(o) => o.index(),
            SentOutput::Orchard(o) => o.index(),
        }
    }

    /// Sets the index of the underlying sent output.
    pub fn set_index(&mut self, index: usize) {
        match self {
            SentOutput::Transparent(o) => o.set_index(index),
            SentOutput::Sapling(o) => o.set_index(index),
            SentOutput::Orchard(o) => o.set_index(index),
        }
    }
}

impl From<SentOutput> for Envelope {
    fn from(value: SentOutput) -> Self {
        match value {
            SentOutput::Transparent(output) => {
                Envelope::from(output).add_assertion("sent_pool", "transparent")
            }
            SentOutput::Sapling(output) => {
                Envelope::from(output).add_assertion("sent_pool", "sapling")
            }
            SentOutput::Orchard(output) => {
                Envelope::from(output).add_assertion("sent_pool", "orchard")
            }
        }
    }
}

impl TryFrom<Envelope> for SentOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        let pool_tag: String = envelope.extract_object_for_predicate("sent_pool")?;
        match pool_tag.as_str() {
            "transparent" => {
                let output = TransparentSentOutput::try_from(envelope)?;
                Ok(SentOutput::Transparent(output))
            }
            "sapling" => {
                let output = SaplingSentOutput::try_from(envelope)?;
                Ok(SentOutput::Sapling(output))
            }
            "orchard" => {
                let output = OrchardSentOutput::try_from(envelope)?;
                Ok(SentOutput::Orchard(output))
            }
            other => Err(bc_envelope::Error::General(
                format!("unknown sent output pool: {}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

    use super::SentOutput;

    impl crate::RandomInstance for SentOutput {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            match rng.random_range(0..3u32) {
                0 => SentOutput::Transparent(
                    crate::transparent::TransparentSentOutput::random(),
                ),
                1 => SentOutput::Sapling(
                    crate::sapling::SaplingSentOutput::random(),
                ),
                _ => SentOutput::Orchard(
                    crate::orchard::OrchardSentOutput::random(),
                ),
            }
        }
    }

    test_envelope_roundtrip!(SentOutput);
}
