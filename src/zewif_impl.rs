use bc_components::ARID;
use bc_envelope::prelude::*;
use std::collections::BTreeMap;

use crate::{envelope_indexed_objects_for_predicate, BlockHash, BlockHeight, Indexed};

use super::{Transaction, TxId, ZewifWallet};

/// Top-level ZeWIF container: wallets, global transaction history, and export metadata.
///
/// Transactions are stored at the top level and referenced by account via `TxId`,
/// avoiding duplication across accounts and wallets.
#[derive(Debug, Clone, PartialEq)]
pub struct Zewif {
    id: ARID,
    wallets: Vec<ZewifWallet>,
    transactions: BTreeMap<TxId, Transaction>,
    export_height: BlockHeight,
    export_height_block_hash: BlockHash,
    attachments: Attachments,
}

bc_envelope::impl_attachable!(Zewif);

impl Zewif {
    pub fn new(export_height: BlockHeight, export_height_block_hash: BlockHash) -> Self {
        Self {
            id: ARID::new(),
            wallets: Vec::new(),
            transactions: BTreeMap::new(),
            export_height,
            export_height_block_hash,
            attachments: Attachments::new(),
        }
    }

    pub fn id(&self) -> ARID {
        self.id
    }

    pub fn wallets(&self) -> &Vec<ZewifWallet> {
        &self.wallets
    }

    pub fn wallets_len(&self) -> usize {
        self.wallets.len()
    }

    pub fn add_wallet(&mut self, mut wallet: ZewifWallet) {
        wallet.set_index(self.wallets_len());
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
}

#[rustfmt::skip]
impl From<Zewif> for Envelope {
    fn from(value: Zewif) -> Self {
        let mut e = Envelope::new(value.id)
            .add_type("Zewif");
        e = value.wallets.iter().fold(e, |e, wallet| e.add_assertion("wallet", wallet.clone()));
        e = value.transactions.iter().fold(e, |e, (_, transaction)| e.add_assertion("transaction", transaction.clone()));
        e = e.add_assertion("export_height", value.export_height);
        e = e.add_assertion("export_height_block_hash", value.export_height_block_hash);
        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for Zewif {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Zewif")?;
        let id = envelope.extract_subject()?;

        let wallets = envelope_indexed_objects_for_predicate(&envelope, "wallet")
            .map_err(|e| bc_envelope::Error::General(format!("wallets: {}", e)))?;

        let transactions = envelope
            .try_objects_for_predicate::<Transaction>("transaction")?
            .into_iter().map(|tx| (tx.txid(), tx)).collect();

        let export_height = envelope.extract_object_for_predicate("export_height")?;
        let export_height_block_hash = envelope.extract_object_for_predicate("export_height_block_hash")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            id,
            wallets,
            transactions,
            export_height,
            export_height_block_hash,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_components::ARID;
    use bc_envelope::Attachments;

    use crate::{BlockHash, BlockHeight, Transaction, test_envelope_roundtrip};

    use super::Zewif;

    impl crate::RandomInstance for Zewif {
        fn random() -> Self {
            use crate::SetIndexes;

            Self {
                id: ARID::new(),
                wallets: Vec::random().set_indexes(),
                transactions: Vec::<Transaction>::random()
                    .iter()
                    .map(|tx| (tx.txid(), tx.clone()))
                    .collect(),
                export_height: BlockHeight::random(),
                export_height_block_hash: BlockHash::random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Zewif);
}
