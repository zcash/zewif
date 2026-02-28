use crate::{Amount, BlockHeight, TransactionData, TxBlockPosition, TxId};
use bc_envelope::prelude::*;

/// A Zcash transaction's metadata as tracked by a wallet.
///
/// Stores the transaction identifier along with optional blockchain context
/// (mining height, block position, fee) and the transaction data itself
/// (either full raw bytes or compact light-wallet representation).
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    txid: TxId,
    /// Full or compact transaction data, if available.
    tx_data: Option<TransactionData>,
    /// The consensus branch height this transaction targets, if known.
    target_height: Option<BlockHeight>,
    /// The height at which this transaction was mined, if known.
    /// May become invalid after a rollback near the export height.
    mined_height: Option<BlockHeight>,
    /// Block hash and index within the block, if known.
    block_position: Option<TxBlockPosition>,
    /// Transaction fee in zatoshis, if known.
    fee: Option<Amount>,
    /// The expiry height for this transaction, if known.
    expiry_height: Option<BlockHeight>,
    attachments: Attachments,
}

bc_envelope::impl_attachable!(Transaction);

impl Transaction {
    pub fn new(txid: TxId) -> Self {
        Self {
            txid,
            tx_data: None,
            target_height: None,
            mined_height: None,
            block_position: None,
            fee: None,
            expiry_height: None,
            attachments: Attachments::new(),
        }
    }

    pub fn txid(&self) -> TxId {
        self.txid
    }

    pub fn tx_data(&self) -> Option<&TransactionData> {
        self.tx_data.as_ref()
    }

    pub fn set_tx_data(&mut self, tx_data: TransactionData) {
        self.tx_data = Some(tx_data);
    }

    pub fn target_height(&self) -> Option<BlockHeight> {
        self.target_height
    }

    pub fn set_target_height(&mut self, height: BlockHeight) {
        self.target_height = Some(height);
    }

    pub fn mined_height(&self) -> Option<BlockHeight> {
        self.mined_height
    }

    pub fn set_mined_height(&mut self, height: BlockHeight) {
        self.mined_height = Some(height);
    }

    pub fn block_position(&self) -> Option<&TxBlockPosition> {
        self.block_position.as_ref()
    }

    pub fn set_block_position(&mut self, block_position: TxBlockPosition) {
        self.block_position = Some(block_position);
    }

    pub fn fee(&self) -> Option<Amount> {
        self.fee
    }

    pub fn set_fee(&mut self, fee: Amount) {
        self.fee = Some(fee);
    }

    pub fn expiry_height(&self) -> Option<BlockHeight> {
        self.expiry_height
    }

    pub fn set_expiry_height(&mut self, height: BlockHeight) {
        self.expiry_height = Some(height);
    }
}

#[rustfmt::skip]
impl From<Transaction> for Envelope {
    fn from(value: Transaction) -> Self {
        let e = Envelope::new(value.txid)
            .add_type("Transaction")
            .add_optional_assertion("tx_data", value.tx_data)
            .add_optional_assertion("target_height", value.target_height)
            .add_optional_assertion("mined_height", value.mined_height)
            .add_optional_assertion("block_position", value.block_position)
            .add_optional_assertion("fee", value.fee)
            .add_optional_assertion("expiry_height", value.expiry_height);
        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for Transaction {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Transaction")?;
        let txid = envelope.extract_subject()?;
        let tx_data = envelope.try_optional_object_for_predicate("tx_data")?;
        let target_height = envelope.try_optional_object_for_predicate("target_height")?;
        let mined_height = envelope.try_optional_object_for_predicate("mined_height")?;
        let block_position = envelope.try_optional_object_for_predicate("block_position")?;
        let fee = envelope.try_optional_object_for_predicate("fee")?;
        let expiry_height = envelope.try_optional_object_for_predicate("expiry_height")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            txid,
            tx_data,
            target_height,
            mined_height,
            block_position,
            fee,
            expiry_height,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use super::Transaction;
    use crate::{
        Amount, BlockHeight, TransactionData, TxBlockPosition, TxId,
        test_envelope_roundtrip,
    };

    impl crate::RandomInstance for Transaction {
        fn random() -> Self {
            Self {
                txid: TxId::random(),
                tx_data: TransactionData::opt_random(),
                target_height: BlockHeight::opt_random(),
                mined_height: BlockHeight::opt_random(),
                block_position: TxBlockPosition::opt_random(),
                fee: Amount::opt_random(),
                expiry_height: BlockHeight::opt_random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Transaction);
}
