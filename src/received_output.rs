use bc_envelope::prelude::*;

use crate::{Amount, Blob, Memo, TxId, orchard::OrchardWitness, sapling::SaplingWitness};

/// A received output within a transaction that belongs to an account.
///
/// Pairs the output's index within its pool with pool-specific metadata
/// and wallet-tracked information such as value, memo, and spending status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedOutput {
    /// Index of the output within the appropriate pool's output list
    /// in the transaction.
    output_index: u32,
    /// Which pool the output belongs to, with pool-specific metadata.
    pool: ReceivedOutputPool,
    /// The value of the output in zatoshis.
    value: Amount,
    /// Memo attached to a shielded output, if any.
    memo: Option<Memo>,
    /// Whether this output is change sent back to the sending account.
    is_change: bool,
    /// The txid of the transaction that spent this output, if it has been spent.
    spent_by: Option<TxId>,
}

/// Locates a note commitment within its pool's note commitment tree, either
/// as a bare position or via a full incremental witness (which carries the
/// position along with an inclusion proof).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitmentTreeData<W> {
    /// The 0-based leaf position of the note commitment in the tree.
    Position(u64),
    /// A full incremental witness; the position is recoverable from it.
    Witness(W),
}

impl<W> From<CommitmentTreeData<W>> for Envelope
where
    W: Into<Envelope>,
{
    fn from(value: CommitmentTreeData<W>) -> Self {
        match value {
            CommitmentTreeData::Position(position) => Envelope::new(position)
                .add_type("CommitmentTreeData")
                .add_assertion("variant", "position"),
            CommitmentTreeData::Witness(w) => {
                let witness: Envelope = w.into();
                Envelope::new("witness")
                    .add_type("CommitmentTreeData")
                    .add_assertion("witness", witness)
            }
        }
    }
}

impl<W> TryFrom<Envelope> for CommitmentTreeData<W>
where
    W: TryFrom<Envelope, Error = bc_envelope::Error>,
{
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("CommitmentTreeData")?;
        match envelope.try_optional_object_for_predicate::<W>("witness")? {
            Some(w) => Ok(CommitmentTreeData::Witness(w)),
            None => {
                let variant: String = envelope.extract_object_for_predicate("variant")?;
                match variant.as_str() {
                    "position" => Ok(CommitmentTreeData::Position(envelope.extract_subject()?)),
                    other => Err(bc_envelope::Error::General(format!(
                        "unknown CommitmentTreeData variant: {}",
                        other
                    ))),
                }
            }
        }
    }
}

/// Identifies which pool a received output belongs to and carries
/// pool-specific metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedOutputPool {
    Transparent,
    Sapling {
        tree_data: Option<CommitmentTreeData<SaplingWitness>>,
        nullifier: Option<Blob<32>>,
    },
    Orchard {
        tree_data: Option<CommitmentTreeData<OrchardWitness>>,
        nullifier: Option<Blob<32>>,
    },
}

impl ReceivedOutput {
    pub fn new(output_index: u32, pool: ReceivedOutputPool, value: Amount) -> Self {
        Self {
            output_index,
            pool,
            value,
            memo: None,
            is_change: false,
            spent_by: None,
        }
    }

    pub fn output_index(&self) -> u32 {
        self.output_index
    }

    pub fn pool(&self) -> &ReceivedOutputPool {
        &self.pool
    }

    /// The position of the output's note commitment in its pool's note
    /// commitment tree, if known. Returns `None` for non-shielded pools.
    pub fn commitment_tree_position(&self) -> Option<u64> {
        match &self.pool {
            ReceivedOutputPool::Sapling { tree_data, .. } => {
                tree_data.as_ref().map(|td| match td {
                    CommitmentTreeData::Position(p) => *p,
                    CommitmentTreeData::Witness(w) => w.note_position() as u64,
                })
            }
            ReceivedOutputPool::Orchard { tree_data, .. } => {
                tree_data.as_ref().map(|td| match td {
                    CommitmentTreeData::Position(p) => *p,
                    CommitmentTreeData::Witness(w) => w.note_position() as u64,
                })
            }
            _ => None,
        }
    }

    pub fn value(&self) -> Amount {
        self.value
    }

    pub fn memo(&self) -> Option<&Memo> {
        self.memo.as_ref()
    }

    pub fn set_memo(&mut self, memo: Option<Memo>) {
        self.memo = memo;
    }

    pub fn is_change(&self) -> bool {
        self.is_change
    }

    pub fn set_is_change(&mut self, is_change: bool) {
        self.is_change = is_change;
    }

    pub fn spent_by(&self) -> Option<TxId> {
        self.spent_by
    }

    pub fn set_spent_by(&mut self, txid: TxId) {
        self.spent_by = Some(txid);
    }
}

impl From<ReceivedOutput> for Envelope {
    fn from(value: ReceivedOutput) -> Self {
        let e = Envelope::new(value.output_index)
            .add_type("ReceivedOutput")
            .add_assertion("value", value.value);
        let e = match value.pool {
            ReceivedOutputPool::Transparent => e.add_assertion("pool", "transparent"),
            ReceivedOutputPool::Sapling {
                tree_data,
                nullifier,
            } => {
                let e = e.add_assertion("pool", "sapling");
                let e = match tree_data {
                    Some(td) => e.add_assertion("tree_data", td),
                    None => e,
                };
                match nullifier {
                    Some(nf) => e.add_assertion("nullifier", nf),
                    None => e,
                }
            }
            ReceivedOutputPool::Orchard {
                tree_data,
                nullifier,
            } => {
                let e = e.add_assertion("pool", "orchard");
                let e = match tree_data {
                    Some(td) => e.add_assertion("tree_data", td),
                    None => e,
                };
                match nullifier {
                    Some(nf) => e.add_assertion("nullifier", nf),
                    None => e,
                }
            }
        };
        let e = match value.memo {
            Some(m) => e.add_assertion("memo", m),
            None => e,
        };
        let e = if value.is_change {
            e.add_assertion("is_change", true)
        } else {
            e
        };
        match value.spent_by {
            Some(txid) => e.add_assertion("spent_by", txid),
            None => e,
        }
    }
}

impl TryFrom<Envelope> for ReceivedOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("ReceivedOutput")?;
        let output_index: u32 = envelope.extract_subject()?;
        let value: Amount = envelope.extract_object_for_predicate("value")?;
        let pool_tag: String = envelope.extract_object_for_predicate("pool")?;
        let pool = match pool_tag.as_str() {
            "transparent" => ReceivedOutputPool::Transparent,
            "sapling" => {
                let tree_data = envelope.try_optional_object_for_predicate("tree_data")?;
                let nullifier = envelope.try_optional_object_for_predicate("nullifier")?;
                ReceivedOutputPool::Sapling {
                    tree_data,
                    nullifier,
                }
            }
            "orchard" => {
                let tree_data = envelope.try_optional_object_for_predicate("tree_data")?;
                let nullifier = envelope.try_optional_object_for_predicate("nullifier")?;
                ReceivedOutputPool::Orchard {
                    tree_data,
                    nullifier,
                }
            }
            other => {
                return Err(bc_envelope::Error::General(format!(
                    "unknown pool type: {}",
                    other
                )));
            }
        };
        let memo = envelope.extract_optional_object_for_predicate("memo")?;
        let is_change = envelope
            .extract_optional_object_for_predicate::<bool>("is_change")?
            .unwrap_or(false);
        let spent_by = envelope.try_optional_object_for_predicate("spent_by")?;
        Ok(Self {
            output_index,
            pool,
            value,
            memo,
            is_change,
            spent_by,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

    use super::{CommitmentTreeData, ReceivedOutput, ReceivedOutputPool};
    use crate::{Amount, Blob, Memo, TxId, orchard::OrchardWitness, sapling::SaplingWitness};

    impl<W: crate::RandomInstance> crate::RandomInstance for CommitmentTreeData<W> {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                CommitmentTreeData::Position(rng.random_range(0..u32::MAX as u64))
            } else {
                CommitmentTreeData::Witness(W::random())
            }
        }
    }

    impl crate::RandomInstance for ReceivedOutput {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            let output_index = rng.random_range(0..100u32);
            let pool = match rng.random_range(0..3u32) {
                0 => ReceivedOutputPool::Transparent,
                1 => ReceivedOutputPool::Sapling {
                    tree_data: CommitmentTreeData::opt_random(),
                    nullifier: Some(Blob::random()),
                },
                _ => ReceivedOutputPool::Orchard {
                    tree_data: CommitmentTreeData::opt_random(),
                    nullifier: Some(Blob::random()),
                },
            };
            let mut output = ReceivedOutput::new(output_index, pool, Amount::random());
            if rng.random_bool(0.5) {
                output.set_memo(Some(Memo::random()));
            }
            output.set_is_change(rng.random_bool(0.3));
            if rng.random_bool(0.4) {
                output.set_spent_by(TxId::random());
            }
            output
        }
    }

    test_envelope_roundtrip!(ReceivedOutput);
    test_envelope_roundtrip!(
        CommitmentTreeData<SaplingWitness>,
        20,
        false,
        test_commitment_tree_data_sapling
    );
    test_envelope_roundtrip!(
        CommitmentTreeData<OrchardWitness>,
        20,
        false,
        test_commitment_tree_data_orchard
    );
}
