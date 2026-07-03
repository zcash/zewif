use minicbor::{Decode, Encode};

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
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct ReceivedOutput {
    /// Index of the output within the appropriate pool's output list
    /// in the transaction.
    #[n(0)]
    output_index: u32,
    /// Which pool the output belongs to, with pool-specific metadata.
    #[n(1)]
    pool: ReceivedOutputPool,
    /// The value of the output in zatoshis, if known.
    #[n(2)]
    value: Option<Amount>,
    /// Memo attached to a shielded output, if any.
    #[n(3)]
    memo: Option<Memo>,
    /// Whether this output is change sent back to the sending account.
    /// None means the exporter had no change information.
    #[n(4)]
    is_change: Option<bool>,
    /// The txid of the transaction that spent this output, if it has been spent.
    #[n(5)]
    spent_by: Option<TxId>,
}

/// Locates a note commitment within its pool's note commitment tree, either
/// as a bare position or via a full incremental witness (which carries the
/// position along with an inclusion proof).
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum CommitmentTreeData<W> {
    /// The 0-based leaf position of the note commitment in the tree.
    #[n(0)]
    Position(#[n(0)] TreePosition),
    /// A full incremental witness; the position is recoverable from it.
    #[n(1)]
    Witness(#[n(0)] W),
}

/// The 0-based leaf position of a note commitment in its pool's note
/// commitment tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TreePosition {
    #[n(0)]
    position: u64,
}

impl TreePosition {
    pub fn new(position: u64) -> Self {
        Self { position }
    }

    pub fn position(&self) -> u64 {
        self.position
    }
}

impl From<u64> for TreePosition {
    fn from(position: u64) -> Self {
        Self::new(position)
    }
}

/// Identifies which pool a received output belongs to and carries
/// pool-specific metadata.
///
/// This enum is non-exhaustive because future network upgrades may add
/// pools.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[non_exhaustive]
pub enum ReceivedOutputPool {
    #[n(0)]
    Transparent(#[n(0)] TransparentOutputData),
    #[n(1)]
    Sprout(#[n(0)] SproutOutputData),
    #[n(2)]
    Sapling(#[n(0)] SaplingOutputData),
    #[n(3)]
    Orchard(#[n(0)] OrchardOutputData),
}

/// Metadata for a received transparent output.
///
/// These fields support representing UTXOs whose containing transaction's
/// full data is unavailable; when the raw transaction is available it is
/// authoritative for the script.
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct TransparentOutputData {
    #[n(0)]
    script: Option<Script>,
    /// The greatest block height at which this output was observed
    /// unspent (maps to zcash_client_sqlite
    /// transparent_received_outputs.max_observed_unspent_height).
    #[n(1)]
    max_observed_unspent_height: Option<BlockHeight>,
}

impl TransparentOutputData {
    pub fn new(script: Option<Script>, max_observed_unspent_height: Option<BlockHeight>) -> Self {
        Self {
            script,
            max_observed_unspent_height,
        }
    }

    pub fn script(&self) -> Option<&Script> {
        self.script.as_ref()
    }

    pub fn max_observed_unspent_height(&self) -> Option<BlockHeight> {
        self.max_observed_unspent_height
    }
}

/// Metadata for a received Sprout output.
///
/// For Sprout, output_index identifies the JoinSplit output as
/// 2 * joinsplit_index + output_index_within_joinsplit (every JoinSplit
/// has exactly two outputs). Sprout note spendability is not
/// reconstructible from this data alone; a Sprout-capable importer must
/// rescan.
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SproutOutputData {
    #[n(0)]
    nullifier: Option<Blob<32>>,
}

impl SproutOutputData {
    pub fn new(nullifier: Option<Blob<32>>) -> Self {
        Self { nullifier }
    }

    pub fn nullifier(&self) -> Option<&Blob<32>> {
        self.nullifier.as_ref()
    }
}

/// Metadata for a received Sapling output.
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct SaplingOutputData {
    #[n(0)]
    tree_data: Option<CommitmentTreeData<SaplingWitness>>,
    #[n(1)]
    nullifier: Option<Blob<32>>,
}

impl SaplingOutputData {
    pub fn new(
        tree_data: Option<CommitmentTreeData<SaplingWitness>>,
        nullifier: Option<Blob<32>>,
    ) -> Self {
        Self {
            tree_data,
            nullifier,
        }
    }

    pub fn tree_data(&self) -> Option<&CommitmentTreeData<SaplingWitness>> {
        self.tree_data.as_ref()
    }

    pub fn nullifier(&self) -> Option<&Blob<32>> {
        self.nullifier.as_ref()
    }
}

/// Metadata for a received Orchard output.
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
#[cbor(map)]
pub struct OrchardOutputData {
    #[n(0)]
    tree_data: Option<CommitmentTreeData<OrchardWitness>>,
    #[n(1)]
    nullifier: Option<Blob<32>>,
}

impl OrchardOutputData {
    pub fn new(
        tree_data: Option<CommitmentTreeData<OrchardWitness>>,
        nullifier: Option<Blob<32>>,
    ) -> Self {
        Self {
            tree_data,
            nullifier,
        }
    }

    pub fn tree_data(&self) -> Option<&CommitmentTreeData<OrchardWitness>> {
        self.tree_data.as_ref()
    }

    pub fn nullifier(&self) -> Option<&Blob<32>> {
        self.nullifier.as_ref()
    }
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
        fn position_of<W: NotePosition>(td: &CommitmentTreeData<W>) -> u64 {
            match td {
                CommitmentTreeData::Position(p) => p.position(),
                CommitmentTreeData::Witness(w) => w.note_position() as u64,
            }
        }

        match &self.pool {
            ReceivedOutputPool::Sapling(data) => data.tree_data().map(position_of),
            ReceivedOutputPool::Orchard(data) => data.tree_data().map(position_of),
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

/// Internal helper for extracting the note position from a witness type.
trait NotePosition {
    fn note_position(&self) -> u32;
}

impl NotePosition for SaplingWitness {
    fn note_position(&self) -> u32 {
        SaplingWitness::note_position(self)
    }
}

impl NotePosition for OrchardWitness {
    fn note_position(&self) -> u32 {
        OrchardWitness::note_position(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::{
        CommitmentTreeData, OrchardOutputData, ReceivedOutput, ReceivedOutputPool,
        SaplingOutputData, SproutOutputData, TransparentOutputData, TreePosition,
    };
    use crate::{
        Amount, Blob, BlockHeight, Memo, Script, TxId, orchard::OrchardWitness,
        sapling::SaplingWitness,
    };

    impl<W: crate::RandomInstance> crate::RandomInstance for CommitmentTreeData<W> {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            if rng.random_bool(0.5) {
                CommitmentTreeData::Position(TreePosition::new(
                    rng.random_range(0..u32::MAX as u64),
                ))
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
                0 => ReceivedOutputPool::Transparent(TransparentOutputData::new(
                    Script::opt_random(),
                    BlockHeight::opt_random(),
                )),
                1 => ReceivedOutputPool::Sprout(SproutOutputData::new(Blob::opt_random())),
                2 => ReceivedOutputPool::Sapling(SaplingOutputData::new(
                    CommitmentTreeData::opt_random(),
                    Blob::opt_random(),
                )),
                _ => ReceivedOutputPool::Orchard(OrchardOutputData::new(
                    CommitmentTreeData::opt_random(),
                    Blob::opt_random(),
                )),
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

    test_cbor_roundtrip!(ReceivedOutput);
    test_cbor_roundtrip!(
        CommitmentTreeData<SaplingWitness>,
        test_commitment_tree_data_sapling
    );
    test_cbor_roundtrip!(
        CommitmentTreeData<OrchardWitness>,
        test_commitment_tree_data_orchard
    );
}
