use minicbor::{Decode, Encode};
use std::collections::BTreeMap;

use crate::{Blob, BlockHash, BlockHeight, Extensions, Secrets};

use super::{Transaction, TxId, ZewifWallet};

/// Top-level ZeWIF container: wallets, global transaction history, and export
/// metadata.
///
/// Transactions are stored at the top level and referenced by account via
/// `TxId`, avoiding duplication across accounts and wallets.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Zewif {
    #[n(0)]
    wallets: Vec<ZewifWallet>,
    /// The global transaction table. On the wire this is an array of
    /// transactions sorted ascending by txid, unique by txid.
    #[cbor(n(1), with = "transaction_table")]
    transactions: BTreeMap<TxId, Transaction>,
    /// The chain tip at export time.
    #[n(2)]
    export_height: BlockHeight,
    /// Hash of the block at the export height; advisory — a chain
    /// reorganization may orphan this block after export.
    #[n(3)]
    export_height_block_hash: BlockHash,
    /// Sensitive key material; absent for a viewing-only export.
    #[n(4)]
    secrets: Option<Secrets>,
    /// Random identifier for this export.
    #[n(5)]
    export_id: Option<Blob<32>>,
    #[cbor(n(6), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
    /// Embedded copy of the CDDL schema for this document's container
    /// version, for archival self-description; informative only — the
    /// container version remains authoritative.
    #[n(7)]
    schema: Option<String>,
}

/// Field codec for the transaction table: the `BTreeMap<TxId, Transaction>`
/// is encoded as an array of transactions in ascending txid order (the map's
/// iteration order), and decoded back into the map keyed by each
/// transaction's txid. A duplicate txid is a decode error.
mod transaction_table {
    use minicbor::decode::Error as DecodeError;
    use minicbor::encode::{Error as EncodeError, Write};
    use minicbor::{Decoder, Encoder};
    use std::collections::BTreeMap;

    use super::{Transaction, TxId};

    pub fn encode<Ctx, W: Write>(
        v: &BTreeMap<TxId, Transaction>,
        e: &mut Encoder<W>,
        ctx: &mut Ctx,
    ) -> Result<(), EncodeError<W::Error>> {
        e.array(v.len() as u64)?;
        for tx in v.values() {
            minicbor::Encode::encode(tx, e, ctx)?;
        }
        Ok(())
    }

    pub fn decode<'b, Ctx>(
        d: &mut Decoder<'b>,
        _ctx: &mut Ctx,
    ) -> Result<BTreeMap<TxId, Transaction>, DecodeError> {
        let mut transactions = BTreeMap::new();
        for tx in d.array_iter::<Transaction>()? {
            let tx = tx?;
            if transactions.insert(tx.txid(), tx).is_some() {
                return Err(DecodeError::message("duplicate txid in transaction table"));
            }
        }
        Ok(transactions)
    }
}

impl Zewif {
    pub fn new(export_height: BlockHeight, export_height_block_hash: BlockHash) -> Self {
        Self {
            wallets: Vec::new(),
            transactions: BTreeMap::new(),
            export_height,
            export_height_block_hash,
            secrets: None,
            export_id: None,
            extensions: Extensions::new(),
            schema: None,
        }
    }

    pub fn wallets(&self) -> &Vec<ZewifWallet> {
        &self.wallets
    }

    pub fn wallets_len(&self) -> usize {
        self.wallets.len()
    }

    pub fn add_wallet(&mut self, wallet: ZewifWallet) {
        self.wallets.push(wallet);
    }

    pub fn transactions(&self) -> &BTreeMap<TxId, Transaction> {
        &self.transactions
    }

    pub fn add_transaction(&mut self, txid: TxId, transaction: Transaction) {
        self.transactions.insert(txid, transaction);
    }

    pub fn get_transaction(&self, txid: TxId) -> Option<&Transaction> {
        self.transactions.get(&txid)
    }

    pub fn set_transactions(&mut self, transactions: BTreeMap<TxId, Transaction>) {
        self.transactions = transactions;
    }

    pub fn export_height(&self) -> BlockHeight {
        self.export_height
    }

    pub fn export_height_block_hash(&self) -> BlockHash {
        self.export_height_block_hash
    }

    pub fn secrets(&self) -> Option<&Secrets> {
        self.secrets.as_ref()
    }

    pub fn set_secrets(&mut self, secrets: Secrets) {
        self.secrets = Some(secrets);
    }

    /// A random 32-byte identifier for this export, if one has been assigned.
    pub fn export_id(&self) -> Option<&Blob<32>> {
        self.export_id.as_ref()
    }

    pub fn set_export_id(&mut self, export_id: Blob<32>) {
        self.export_id = Some(export_id);
    }

    /// The embedded copy of the CDDL schema for this document's container
    /// version, if present. Informative only; the container version remains
    /// authoritative.
    pub fn embedded_schema(&self) -> Option<&str> {
        self.schema.as_deref()
    }

    pub fn set_embedded_schema(&mut self, schema: impl Into<String>) {
        self.schema = Some(schema.into());
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
    use crate::{
        Blob, BlockHash, BlockHeight, Extensions, RandomInstance, Secrets, Transaction, TxId,
        test_cbor_roundtrip,
    };

    use super::Zewif;

    impl RandomInstance for Zewif {
        fn random() -> Self {
            Self {
                wallets: Vec::random(),
                transactions: Vec::<Transaction>::random()
                    .iter()
                    .map(|tx| (tx.txid(), tx.clone()))
                    .collect(),
                export_height: BlockHeight::random(),
                export_height_block_hash: BlockHash::random(),
                secrets: Secrets::opt_random(),
                export_id: Blob::opt_random(),
                extensions: Extensions::random(),
                schema: String::opt_random(),
            }
        }
    }

    test_cbor_roundtrip!(Zewif);

    /// The transaction table must appear on the wire as an array of
    /// transactions in ascending txid order, and decode back into the
    /// txid-keyed map.
    #[test]
    fn transaction_table_encodes_as_sorted_array() {
        let mut zewif = Zewif::new(BlockHeight::from_u32(100), BlockHash::from_bytes([0u8; 32]));
        let txid_a = TxId::from_bytes([1u8; 32]);
        let txid_b = TxId::from_bytes([2u8; 32]);
        // Insert in descending txid order; the map re-sorts.
        zewif.add_transaction(txid_b, Transaction::new(txid_b));
        zewif.add_transaction(txid_a, Transaction::new(txid_a));

        let encoded = minicbor::to_vec(&zewif).unwrap();

        let mut d = minicbor::Decoder::new(&encoded);
        let _map_len = d.map().unwrap();
        assert_eq!(d.u32().unwrap(), 0); // wallets
        d.skip().unwrap();
        assert_eq!(d.u32().unwrap(), 1); // transaction table
        assert_eq!(d.datatype().unwrap(), minicbor::data::Type::Array);
        let transactions: Vec<Transaction> = d
            .array_iter::<Transaction>()
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            transactions.iter().map(|tx| tx.txid()).collect::<Vec<_>>(),
            vec![txid_a, txid_b]
        );

        let decoded: Zewif = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, zewif);
    }

    /// A transaction table containing two transactions with the same txid
    /// must be rejected.
    #[test]
    fn duplicate_txid_is_a_decode_error() {
        let txid = TxId::from_bytes([3u8; 32]);
        let tx_bytes = minicbor::to_vec(Transaction::new(txid)).unwrap();

        let mut buf = Vec::new();
        {
            let mut e = minicbor::Encoder::new(&mut buf);
            e.map(4).unwrap();
            e.u32(0).unwrap().array(0).unwrap(); // wallets
            e.u32(1).unwrap().array(2).unwrap(); // transaction table
        }
        buf.extend_from_slice(&tx_bytes);
        buf.extend_from_slice(&tx_bytes);
        {
            let mut e = minicbor::Encoder::new(&mut buf);
            e.u32(2).unwrap().u32(100).unwrap(); // export height
            e.u32(3).unwrap().bytes(&[0u8; 32]).unwrap(); // export block hash
        }

        assert!(minicbor::decode::<Zewif>(&buf).is_err());
    }
}
