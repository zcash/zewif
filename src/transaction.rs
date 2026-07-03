use crate::{Amount, BlockHeight, Extensions, TransactionData, TxBlockPosition, TxId};
use minicbor::{Decode, Encode};

/// A Zcash transaction's metadata as tracked by a wallet.
///
/// Stores the transaction identifier along with optional blockchain context
/// (mining height, block position, fee) and the transaction data itself
/// (either full raw bytes or compact light-wallet representation).
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Transaction {
    #[n(0)]
    txid: TxId,
    /// Full or compact transaction data, if available.
    #[n(1)]
    tx_data: Option<TransactionData>,
    /// The consensus branch height this transaction targets, if known.
    #[n(2)]
    target_height: Option<BlockHeight>,
    /// The height at which this transaction was mined, if known.
    /// May become invalid after a rollback near the export height.
    #[n(3)]
    mined_height: Option<BlockHeight>,
    /// Block hash and index within the block, if known.
    #[n(4)]
    block_position: Option<TxBlockPosition>,
    /// Transaction fee in zatoshis, if known.
    #[n(5)]
    fee: Option<Amount>,
    /// The expiry height for this transaction, if known.
    #[n(6)]
    expiry_height: Option<BlockHeight>,
    /// The wallet-local timestamp associated with this transaction, in seconds
    /// since the Unix epoch: its creation time for wallet-authored transactions,
    /// or the time it was first received or observed (zcashd's nTimeReceived).
    #[n(7)]
    created_time: Option<i64>,
    /// Whether the user has explicitly marked this transaction as trusted,
    /// making its outputs spendable under the trusted confirmations policy
    /// of ZIP 315. Omitted from the encoding when false.
    #[cbor(n(8), with = "trusted_flag", has_nil)]
    trusted: bool,
    #[cbor(n(9), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
}

/// Field codec for the `trusted` flag: the map entry is omitted when false,
/// and an absent (or null) entry decodes as false.
mod trusted_flag {
    use minicbor::decode::Error as DecodeError;
    use minicbor::encode::{Error as EncodeError, Write};
    use minicbor::{Decoder, Encoder};

    pub fn encode<Ctx, W: Write>(
        v: &bool,
        e: &mut Encoder<W>,
        _ctx: &mut Ctx,
    ) -> Result<(), EncodeError<W::Error>> {
        e.bool(*v)?;
        Ok(())
    }

    pub fn decode<'b, Ctx>(d: &mut Decoder<'b>, _ctx: &mut Ctx) -> Result<bool, DecodeError> {
        if d.datatype()? == minicbor::data::Type::Null {
            d.skip()?;
            return Ok(false);
        }
        d.bool()
    }

    pub fn nil() -> Option<bool> {
        Some(false)
    }

    pub fn is_nil(v: &bool) -> bool {
        !*v
    }
}

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
            created_time: None,
            trusted: false,
            extensions: Extensions::new(),
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

    pub fn created_time(&self) -> Option<i64> {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: i64) {
        self.created_time = Some(created_time);
    }

    pub fn is_trusted(&self) -> bool {
        self.trusted
    }

    pub fn set_trusted(&mut self, trusted: bool) {
        self.trusted = trusted;
    }

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

#[cfg(test)]
mod tests {
    use super::Transaction;
    use crate::{
        Amount, BlockHeight, Extensions, TransactionData, TxBlockPosition, TxId,
        test_cbor_roundtrip,
    };

    impl crate::RandomInstance for Transaction {
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::rng();
            Self {
                txid: TxId::random(),
                tx_data: TransactionData::opt_random(),
                target_height: BlockHeight::opt_random(),
                mined_height: BlockHeight::opt_random(),
                block_position: TxBlockPosition::opt_random(),
                fee: Amount::opt_random(),
                expiry_height: BlockHeight::opt_random(),
                created_time: rng
                    .random_bool(0.5)
                    .then(|| rng.random_range(0..=2_000_000_000i64)),
                trusted: rng.random_bool(0.3),
                extensions: Extensions::random(),
            }
        }
    }

    test_cbor_roundtrip!(Transaction);

    /// A default (untrusted) flag and empty extensions must be omitted from
    /// the encoding, so that each state has exactly one encoding.
    #[test]
    fn default_fields_are_omitted() {
        let tx = Transaction::new(TxId::from_bytes([7u8; 32]));
        let encoded = minicbor::to_vec(&tx).unwrap();
        // A single-entry map containing only the txid.
        let mut expected = vec![0xa1, 0x00, 0x58, 0x20];
        expected.extend_from_slice(&[7u8; 32]);
        assert_eq!(encoded, expected);
    }

    /// Readers must ignore map keys not defined in the schema version they
    /// implement.
    #[test]
    fn unknown_fields_are_ignored() {
        let mut buf = Vec::new();
        let mut e = minicbor::Encoder::new(&mut buf);
        e.map(2).unwrap();
        e.u32(0).unwrap().bytes(&[7u8; 32]).unwrap();
        e.u32(99).unwrap().str("data from the future").unwrap();
        let tx: Transaction = minicbor::decode(&buf).unwrap();
        assert_eq!(tx.txid(), TxId::from_bytes([7u8; 32]));
    }

    /// Readers should treat a null value in an optional field position as if
    /// the entry were absent.
    #[test]
    fn null_optional_fields_decode_as_absent() {
        let mut buf = Vec::new();
        let mut e = minicbor::Encoder::new(&mut buf);
        e.map(3).unwrap();
        e.u32(0).unwrap().bytes(&[7u8; 32]).unwrap();
        e.u32(3).unwrap().null().unwrap();
        e.u32(8).unwrap().null().unwrap();
        let tx: Transaction = minicbor::decode(&buf).unwrap();
        assert_eq!(tx.mined_height(), None);
        assert!(!tx.is_trusted());
    }
}
