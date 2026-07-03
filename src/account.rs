use bc_envelope::prelude::*;
use std::collections::HashMap;

use crate::{
    AccountViewingKey, Address, BlockHash, BlockHeight, ChainState, Indexed, KeySource,
    ReceivedOutput, ScanRange, SentOutput, TxId, envelope_indexed_objects_for_predicate,
};

/// A logical grouping of funds, addresses, and transaction history.
///
/// An account represents an undifferentiated pool of funds within a wallet.
/// It is centered on a viewing capability (`AccountViewingKey`) that determines
/// what the account can observe on-chain, and optionally a `KeySource` that
/// records how the keys were obtained (HD derivation or import).
///
/// For HD-derived accounts, the viewing key and spending capability are
/// derivable from the wallet's seed material plus the `KeySource` metadata.
/// For imported accounts, the viewing key is stored directly.
///
/// ## Legacy zcashd hybrid accounts
///
/// Due to how mnemonic seeds were introduced in zcashd v4.7.0, a legacy
/// account may combine `AccountViewingKey::TransparentAddressSet` with
/// `KeySource::Derived { account_index: 0x7FFFFFFF, .. }`. This means
/// the account contains both pre-mnemonic randomly-derived addresses
/// (each carrying its own private key via `TransparentSpendAuthority::Imported`)
/// and HD-derived addresses (via `TransparentSpendAuthority::Derived`). The
/// account-level `KeySource::Derived` indicates that additional addresses
/// *can* be derived from the seed at that account index.
#[derive(Clone, PartialEq)]
pub struct Account {
    index: usize,

    name: String,

    /// The viewing capability for this account.
    viewing_key: AccountViewingKey,

    /// How the account's keys were obtained.
    key_source: Option<KeySource>,

    /// Minimum block height at which to scan for this account.
    birthday_height: Option<BlockHeight>,

    /// Hash of the birthday block, for chain verification.
    birthday_block: Option<BlockHash>,

    /// Tree state at a block at or before `birthday_height` from which
    /// scanning may begin (maps to librustzcash `AccountBirthday`
    /// prior_chain_state). Exporters without chain access omit it.
    birthday_chain_state: Option<ChainState>,

    /// Height below which recovery of this account's history is considered
    /// complete (maps to zcash_client_sqlite accounts.recover_until_height).
    recover_until_height: Option<BlockHeight>,

    /// The capability of the account in the source wallet; `None` = unknown.
    purpose: Option<AccountPurpose>,

    /// Free-form tag identifying the origin of the account's key material,
    /// e.g. "zcashd_mnemonic" (maps to zcash_client_sqlite
    /// accounts.key_source; named provenance here because zewif uses
    /// `KeySource` for the structured enum).
    provenance: Option<String>,

    /// Block ranges that have been fully scanned for this account.
    scanned_ranges: Vec<ScanRange>,

    addresses: Vec<Address>,

    /// Maps transaction IDs to the received outputs relevant to this account.
    relevant_transactions: HashMap<TxId, Vec<ReceivedOutput>>,

    /// Sent output metadata not recoverable from the chain, grouped by
    /// the transaction that created them.
    sent_outputs: HashMap<TxId, Vec<SentOutput>>,

    attachments: Attachments,
}

/// The capability of an account in the source wallet (maps to
/// zcash_client_backend `AccountPurpose`).
///
/// `ViewOnly` indicates the account was imported without spend authority;
/// a `None` value of a containing `Option` means the purpose is unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountPurpose {
    Spending,
    ViewOnly,
}

#[rustfmt::skip]
impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("index", &self.index)
            .field("name", &self.name)
            .field("viewing_key", &self.viewing_key)
            .field("key_source", &self.key_source)
            .field("birthday_height", &self.birthday_height)
            .field("birthday_block", &self.birthday_block)
            .field("birthday_chain_state", &self.birthday_chain_state)
            .field("recover_until_height", &self.recover_until_height)
            .field("purpose", &self.purpose)
            .field("provenance", &self.provenance)
            .field("scanned_ranges", &self.scanned_ranges)
            .field("addresses", &self.addresses)
            .field("relevant_transactions", &self.relevant_transactions)
            .field("sent_outputs", &self.sent_outputs)
            .field("attachments", &self.attachments)
            .finish()
    }
}

bc_envelope::impl_attachable!(Account);

impl Indexed for Account {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl Account {
    pub fn new(viewing_key: AccountViewingKey) -> Self {
        Self {
            index: 0,
            name: String::default(),
            viewing_key,
            key_source: None,
            birthday_height: None,
            birthday_block: None,
            birthday_chain_state: None,
            recover_until_height: None,
            purpose: None,
            provenance: None,
            scanned_ranges: Vec::new(),
            addresses: Vec::new(),
            relevant_transactions: HashMap::new(),
            sent_outputs: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn viewing_key(&self) -> &AccountViewingKey {
        &self.viewing_key
    }

    pub fn key_source(&self) -> Option<&KeySource> {
        self.key_source.as_ref()
    }

    pub fn set_key_source(&mut self, source: KeySource) {
        self.key_source = Some(source);
    }

    pub fn birthday_height(&self) -> Option<BlockHeight> {
        self.birthday_height
    }

    pub fn set_birthday_height(&mut self, height: BlockHeight) {
        self.birthday_height = Some(height);
    }

    pub fn birthday_block(&self) -> Option<BlockHash> {
        self.birthday_block
    }

    pub fn set_birthday_block(&mut self, hash: BlockHash) {
        self.birthday_block = Some(hash);
    }

    pub fn birthday_chain_state(&self) -> Option<&ChainState> {
        self.birthday_chain_state.as_ref()
    }

    pub fn set_birthday_chain_state(&mut self, chain_state: ChainState) {
        self.birthday_chain_state = Some(chain_state);
    }

    pub fn recover_until_height(&self) -> Option<BlockHeight> {
        self.recover_until_height
    }

    pub fn set_recover_until_height(&mut self, height: BlockHeight) {
        self.recover_until_height = Some(height);
    }

    pub fn purpose(&self) -> Option<AccountPurpose> {
        self.purpose
    }

    pub fn set_purpose(&mut self, purpose: AccountPurpose) {
        self.purpose = Some(purpose);
    }

    pub fn provenance(&self) -> Option<&str> {
        self.provenance.as_deref()
    }

    pub fn set_provenance(&mut self, provenance: impl Into<String>) {
        self.provenance = Some(provenance.into());
    }

    pub fn scanned_ranges(&self) -> &[ScanRange] {
        &self.scanned_ranges
    }

    pub fn add_scanned_range(&mut self, range: ScanRange) {
        self.scanned_ranges.push(range);
    }

    pub fn addresses(&self) -> &[Address] {
        &self.addresses
    }

    pub fn add_address(&mut self, mut address: Address) {
        address.set_index(self.addresses.len());
        self.addresses.push(address);
    }

    pub fn relevant_transactions(&self) -> &HashMap<TxId, Vec<ReceivedOutput>> {
        &self.relevant_transactions
    }

    pub fn add_relevant_transaction(&mut self, txid: TxId, outputs: Vec<ReceivedOutput>) {
        self.relevant_transactions.insert(txid, outputs);
    }

    pub fn sent_outputs(&self) -> &HashMap<TxId, Vec<SentOutput>> {
        &self.sent_outputs
    }

    pub fn add_sent_output(&mut self, txid: TxId, mut output: SentOutput) {
        let outputs = self.sent_outputs.entry(txid).or_default();
        output.set_index(outputs.len());
        outputs.push(output);
    }

    pub fn add_sent_outputs(&mut self, txid: TxId, outputs: Vec<SentOutput>) {
        for output in outputs {
            self.add_sent_output(txid, output);
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(AccountViewingKey::TransparentAddressSet)
    }
}

#[rustfmt::skip]
impl From<Account> for Envelope {
    fn from(value: Account) -> Self {
        let mut e = Envelope::new(value.index)
            .add_type("Account")
            .add_assertion("name", value.name)
            .add_assertion("viewing_key", value.viewing_key)
            .add_optional_assertion("key_source", value.key_source)
            .add_optional_assertion("birthday_height", value.birthday_height)
            .add_optional_assertion("birthday_block", value.birthday_block)
            .add_optional_assertion("birthday_chain_state", value.birthday_chain_state)
            .add_optional_assertion("recover_until_height", value.recover_until_height)
            .add_optional_assertion("purpose", value.purpose.map(|purpose| match purpose {
                AccountPurpose::Spending => "spending",
                AccountPurpose::ViewOnly => "view_only",
            }))
            .add_optional_assertion("provenance", value.provenance);

        e = value.scanned_ranges.iter().fold(e, |e, range| e.add_assertion("scanned_range", *range));
        e = value.addresses.iter().fold(e, |e, address| e.add_assertion("address", address.clone()));

        // Serialize relevant_transactions as a list of (txid, outputs) pairs
        for (txid, outputs) in &value.relevant_transactions {
            let tx_envelope = Envelope::new(*txid)
                .add_type("RelevantTransaction");
            let tx_envelope = outputs.iter().fold(tx_envelope, |e, output| {
                e.add_assertion("output", output.clone())
            });
            e = e.add_assertion("relevant_transaction", tx_envelope);
        }

        // Serialize sent_outputs as a list of (txid, outputs) pairs
        for (txid, outputs) in &value.sent_outputs {
            let tx_envelope = Envelope::new(*txid)
                .add_type("SentTransaction");
            let tx_envelope = outputs.iter().fold(tx_envelope, |e, output| {
                e.add_assertion("sent_output", output.clone())
            });
            e = e.add_assertion("sent_transaction", tx_envelope);
        }

        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for Account {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Account")?;
        let index = envelope.extract_subject()?;
        let name = envelope.extract_object_for_predicate("name")?;
        let viewing_key = envelope.try_object_for_predicate("viewing_key")?;
        let key_source = envelope.try_optional_object_for_predicate("key_source")?;
        let birthday_height = envelope.extract_optional_object_for_predicate("birthday_height")?;
        let birthday_block = envelope.extract_optional_object_for_predicate("birthday_block")?;
        let birthday_chain_state =
            envelope.try_optional_object_for_predicate("birthday_chain_state")?;
        let recover_until_height =
            envelope.extract_optional_object_for_predicate("recover_until_height")?;
        let purpose = match envelope.extract_optional_object_for_predicate::<String>("purpose")? {
            None => None,
            Some(s) => match s.as_str() {
                "spending" => Some(AccountPurpose::Spending),
                "view_only" => Some(AccountPurpose::ViewOnly),
                other => {
                    return Err(bc_envelope::Error::General(format!(
                        "unknown AccountPurpose: {}",
                        other
                    )));
                }
            },
        };
        let provenance = envelope.extract_optional_object_for_predicate("provenance")?;

        let mut scanned_ranges: Vec<ScanRange> =
            envelope.try_objects_for_predicate("scanned_range")?;
        scanned_ranges.sort_by_key(|r| r.start());

        let addresses = envelope_indexed_objects_for_predicate(&envelope, "address")
            .map_err(|e| bc_envelope::Error::General(format!("addresses: {}", e)))?;

        // Deserialize relevant_transactions
        let mut relevant_transactions = HashMap::new();
        for tx_env in envelope.objects_for_predicate("relevant_transaction") {
            tx_env.check_type("RelevantTransaction")?;
            let txid: TxId = tx_env.extract_subject()?;
            let mut outputs: Vec<ReceivedOutput> = tx_env.try_objects_for_predicate("output")?;
            outputs.sort_by_key(|o| o.output_index());
            relevant_transactions.insert(txid, outputs);
        }

        // Deserialize sent_outputs
        let mut sent_outputs = HashMap::new();
        for tx_env in envelope.objects_for_predicate("sent_transaction") {
            tx_env.check_type("SentTransaction")?;
            let txid: TxId = tx_env.extract_subject()?;
            let mut outputs: Vec<SentOutput> = tx_env.try_objects_for_predicate("sent_output")?;
            outputs.sort_by_key(|o| o.index());
            sent_outputs.insert(txid, outputs);
        }

        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            index,
            name,
            viewing_key,
            key_source,
            birthday_height,
            birthday_block,
            birthday_chain_state,
            recover_until_height,
            purpose,
            provenance,
            scanned_ranges,
            addresses,
            relevant_transactions,
            sent_outputs,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bc_envelope::Attachments;

    use crate::{
        AccountViewingKey, BlockHash, BlockHeight, ChainState, KeySource, ReceivedOutput,
        ScanRange, SentOutput, TxId, test_envelope_roundtrip,
    };

    use super::{Account, AccountPurpose};

    impl crate::RandomInstance for AccountPurpose {
        fn random() -> Self {
            let mut rng = rand::rng();
            if rand::Rng::random_bool(&mut rng, 0.5) {
                AccountPurpose::Spending
            } else {
                AccountPurpose::ViewOnly
            }
        }
    }

    impl crate::RandomInstance for Account {
        fn random() -> Self {
            use crate::SetIndexes;
            use rand::Rng;

            let mut rng = rand::rng();
            let num_txs = rng.random_range(0..3usize);
            let mut relevant_transactions = HashMap::new();
            for _ in 0..num_txs {
                let txid = TxId::random();
                let num_outputs = rng.random_range(1..4usize);
                let outputs: Vec<ReceivedOutput> = (0..num_outputs)
                    .map(|i| {
                        ReceivedOutput::new(
                            i as u32,
                            crate::ReceivedOutputPool::Transparent {
                                script: None,
                                max_observed_unspent_height: None,
                            },
                            crate::Amount::random(),
                        )
                    })
                    .collect();
                relevant_transactions.insert(txid, outputs);
            }

            let num_sent_txs = rng.random_range(0..3usize);
            let mut sent_outputs = HashMap::new();
            for _ in 0..num_sent_txs {
                let txid = TxId::random();
                let num_outputs = rng.random_range(1..3usize);
                let outputs: Vec<SentOutput> = (0..num_outputs)
                    .enumerate()
                    .map(|(i, _)| {
                        let mut o = SentOutput::random();
                        o.set_index(i);
                        o
                    })
                    .collect();
                sent_outputs.insert(txid, outputs);
            }

            let num_ranges = rng.random_range(0..3usize);
            let mut scanned_ranges: Vec<ScanRange> =
                (0..num_ranges).map(|_| ScanRange::random()).collect();
            scanned_ranges.sort_by_key(|r| r.start());

            Self {
                index: 0,
                name: String::random(),
                viewing_key: AccountViewingKey::random(),
                key_source: KeySource::opt_random(),
                birthday_height: BlockHeight::opt_random(),
                birthday_block: BlockHash::opt_random(),
                birthday_chain_state: ChainState::opt_random(),
                recover_until_height: BlockHeight::opt_random(),
                purpose: AccountPurpose::opt_random(),
                provenance: String::opt_random(),
                scanned_ranges,
                addresses: Vec::random().set_indexes(),
                relevant_transactions,
                sent_outputs,
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Account);
}
