use bc_envelope::prelude::*;

use crate::{
    orchard::OrchardWitness,
    sapling::SaplingWitness,
};

/// A received output within a transaction that belongs to an account.
///
/// Pairs the output's index within its pool with pool-specific metadata
/// (currently just an optional witness for shielded outputs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedOutput {
    /// Index of the output within the appropriate pool's output list
    /// in the transaction.
    output_index: u32,
    /// Which pool the output belongs to, with pool-specific metadata.
    pool: ReceivedOutputPool,
}

/// Identifies which pool a received output belongs to and carries
/// pool-specific metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedOutputPool {
    Transparent,
    Sapling { witness: Option<SaplingWitness> },
    Orchard { witness: Option<OrchardWitness> },
}

impl ReceivedOutput {
    pub fn new(output_index: u32, pool: ReceivedOutputPool) -> Self {
        Self { output_index, pool }
    }

    pub fn output_index(&self) -> u32 {
        self.output_index
    }

    pub fn pool(&self) -> &ReceivedOutputPool {
        &self.pool
    }
}

impl From<ReceivedOutput> for Envelope {
    fn from(value: ReceivedOutput) -> Self {
        let e = Envelope::new(value.output_index)
            .add_type("ReceivedOutput");
        match value.pool {
            ReceivedOutputPool::Transparent => {
                e.add_assertion("pool", "transparent")
            }
            ReceivedOutputPool::Sapling { witness } => {
                let e = e.add_assertion("pool", "sapling");
                match witness {
                    Some(w) => e.add_assertion("witness", w),
                    None => e,
                }
            }
            ReceivedOutputPool::Orchard { witness } => {
                let e = e.add_assertion("pool", "orchard");
                match witness {
                    Some(w) => e.add_assertion("witness", w),
                    None => e,
                }
            }
        }
    }
}

impl TryFrom<Envelope> for ReceivedOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("ReceivedOutput")?;
        let output_index: u32 = envelope.extract_subject()?;
        let pool_tag: String = envelope.extract_object_for_predicate("pool")?;
        let pool = match pool_tag.as_str() {
            "transparent" => ReceivedOutputPool::Transparent,
            "sapling" => {
                let witness = envelope.try_optional_object_for_predicate("witness")?;
                ReceivedOutputPool::Sapling { witness }
            }
            "orchard" => {
                let witness = envelope.try_optional_object_for_predicate("witness")?;
                ReceivedOutputPool::Orchard { witness }
            }
            other => {
                return Err(bc_envelope::Error::General(
                    format!("unknown pool type: {}", other),
                ));
            }
        };
        Ok(Self { output_index, pool })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

    use super::{ReceivedOutput, ReceivedOutputPool};

    impl crate::RandomInstance for ReceivedOutput {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let output_index = rng.random_range(0..100u32);
            let pool = match rng.random_range(0..3u32) {
                0 => ReceivedOutputPool::Transparent,
                1 => ReceivedOutputPool::Sapling {
                    witness: crate::sapling::SaplingWitness::opt_random(),
                },
                _ => ReceivedOutputPool::Orchard {
                    witness: crate::orchard::OrchardWitness::opt_random(),
                },
            };
            ReceivedOutput::new(output_index, pool)
        }
    }

    test_envelope_roundtrip!(ReceivedOutput);
}
