use bc_envelope::prelude::*;

use crate::{
    Amount, Blob, BlockHeight, Memo, Script, TxId, orchard::OrchardWitness, sapling::SaplingWitness,
};

/// A received output within a transaction that belongs to an account.
///
/// Pairs the output's index within its pool with pool-specific metadata
/// and wallet-tracked information such as value, memo, and spending status.
///
/// The value, memo, change-status, and nullifier fields are optional
/// enrichment: they are recoverable from the raw transaction plus the
/// account's viewing key, so exporters SHOULD include them when the raw
/// transaction data is absent. Where both are present, the raw transaction
/// is authoritative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedOutput {
    /// Index of the output within the appropriate pool's output list
    /// in the transaction.
    output_index: u32,
    /// Which pool the output belongs to, with pool-specific metadata.
    pool: ReceivedOutputPool,
    /// The value of the output in zatoshis, if known.
    value: Option<Amount>,
    /// Memo attached to a shielded output, if any.
    memo: Option<Memo>,
    /// Whether this output is change sent back to the sending account.
    /// None means the exporter had no change information.
    is_change: Option<bool>,
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
///
/// This enum is non-exhaustive because future network upgrades may add
/// pools.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReceivedOutputPool {
    /// These fields support representing UTXOs whose containing
    /// transaction's full data is unavailable; when the raw transaction is
    /// available it is authoritative for the script.
    Transparent {
        script: Option<Script>,
        /// The greatest block height at which this output was observed
        /// unspent (maps to zcash_client_sqlite
        /// transparent_received_outputs.max_observed_unspent_height).
        max_observed_unspent_height: Option<BlockHeight>,
    },
    /// For Sprout, output_index identifies the JoinSplit output as
    /// 2 * joinsplit_index + output_index_within_joinsplit (every JoinSplit
    /// has exactly two outputs). Sprout note spendability is not
    /// reconstructible from this data alone; a Sprout-capable importer must
    /// rescan.
    Sprout { nullifier: Option<Blob<32>> },
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
    pub fn new(output_index: u32, pool: ReceivedOutputPool) -> Self {
        Self {
            output_index,
            pool,
            value: None,
            memo: None,
            is_change: None,
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

    pub fn value(&self) -> Option<Amount> {
        self.value
    }

    pub fn set_value(&mut self, value: Amount) {
        self.value = Some(value);
    }

    pub fn memo(&self) -> Option<&Memo> {
        self.memo.as_ref()
    }

    pub fn set_memo(&mut self, memo: Option<Memo>) {
        self.memo = memo;
    }

    /// Whether this output is change sent back to the sending account;
    /// None means the exporter had no change information.
    pub fn is_change(&self) -> Option<bool> {
        self.is_change
    }

    pub fn set_is_change(&mut self, is_change: bool) {
        self.is_change = Some(is_change);
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
            .add_optional_assertion("value", value.value);
        let e = match value.pool {
            ReceivedOutputPool::Transparent {
                script,
                max_observed_unspent_height,
            } => {
                let e = e.add_assertion("pool", "transparent");
                let e = match script {
                    Some(s) => e.add_assertion("script", s),
                    None => e,
                };
                match max_observed_unspent_height {
                    Some(h) => e.add_assertion("max_observed_unspent_height", h),
                    None => e,
                }
            }
            ReceivedOutputPool::Sprout { nullifier } => {
                let e = e.add_assertion("pool", "sprout");
                match nullifier {
                    Some(nf) => e.add_assertion("nullifier", nf),
                    None => e,
                }
            }
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
        let e = e.add_optional_assertion("is_change", value.is_change);
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
        let value: Option<Amount> = envelope.extract_optional_object_for_predicate("value")?;
        let pool_tag: String = envelope.extract_object_for_predicate("pool")?;
        let pool = match pool_tag.as_str() {
            "transparent" => {
                let script = envelope.extract_optional_object_for_predicate("script")?;
                let max_observed_unspent_height = envelope
                    .extract_optional_object_for_predicate("max_observed_unspent_height")?;
                ReceivedOutputPool::Transparent {
                    script,
                    max_observed_unspent_height,
                }
            }
            "sprout" => {
                let nullifier = envelope.try_optional_object_for_predicate("nullifier")?;
                ReceivedOutputPool::Sprout { nullifier }
            }
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
        let is_change = envelope.extract_optional_object_for_predicate::<bool>("is_change")?;
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
    use crate::{
        Amount, Blob, BlockHeight, Memo, Script, TxId, orchard::OrchardWitness,
        sapling::SaplingWitness,
    };

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
            let pool = match rng.random_range(0..4u32) {
                0 => ReceivedOutputPool::Transparent {
                    script: Script::opt_random(),
                    max_observed_unspent_height: BlockHeight::opt_random(),
                },
                1 => ReceivedOutputPool::Sprout {
                    nullifier: Blob::opt_random(),
                },
                2 => ReceivedOutputPool::Sapling {
                    tree_data: CommitmentTreeData::opt_random(),
                    nullifier: Blob::opt_random(),
                },
                _ => ReceivedOutputPool::Orchard {
                    tree_data: CommitmentTreeData::opt_random(),
                    nullifier: Blob::opt_random(),
                },
            };
            let mut output = ReceivedOutput::new(output_index, pool);
            if let Some(value) = Amount::opt_random() {
                output.set_value(value);
            }
            if rng.random_bool(0.5) {
                output.set_memo(Some(Memo::random()));
            }
            if rng.random_bool(0.6) {
                output.set_is_change(rng.random_bool(0.5));
            }
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
