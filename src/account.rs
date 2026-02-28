use bc_envelope::prelude::*;
use std::collections::HashMap;

use crate::{
    envelope_indexed_objects_for_predicate,
    orchard::OrchardSentOutput,
    sapling::SaplingSentOutput,
    AccountViewingKey, Address, BlockHash, BlockHeight,
    Indexed, KeySource, ReceivedOutput,
    ScanRange, TxId,
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

    /// Block ranges that have been fully scanned for this account.
    scanned_ranges: Vec<ScanRange>,

    addresses: Vec<Address>,

    /// Maps transaction IDs to the received outputs relevant to this account.
    relevant_transactions: HashMap<TxId, Vec<ReceivedOutput>>,

    /// Sent output metadata not recoverable from the chain.
    sapling_sent_outputs: Vec<SaplingSentOutput>,
    orchard_sent_outputs: Vec<OrchardSentOutput>,
    attachments: Attachments,
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
            .field("scanned_ranges", &self.scanned_ranges)
            .field("addresses", &self.addresses)
            .field("relevant_transactions", &self.relevant_transactions)
            .field("sapling_sent_outputs", &self.sapling_sent_outputs)
            .field("orchard_sent_outputs", &self.orchard_sent_outputs)
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
            scanned_ranges: Vec::new(),
            addresses: Vec::new(),
            relevant_transactions: HashMap::new(),
            sapling_sent_outputs: Vec::new(),
            orchard_sent_outputs: Vec::new(),
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

    pub fn sapling_sent_outputs(&self) -> &[SaplingSentOutput] {
        &self.sapling_sent_outputs
    }

    pub fn add_sapling_sent_output(&mut self, mut output: SaplingSentOutput) {
        output.set_index(self.sapling_sent_outputs.len());
        self.sapling_sent_outputs.push(output);
    }

    pub fn orchard_sent_outputs(&self) -> &[OrchardSentOutput] {
        &self.orchard_sent_outputs
    }

    pub fn add_orchard_sent_output(&mut self, mut output: OrchardSentOutput) {
        output.set_index(self.orchard_sent_outputs.len());
        self.orchard_sent_outputs.push(output);
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
            .add_optional_assertion("birthday_block", value.birthday_block);

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

        e = value.sapling_sent_outputs.iter().fold(e, |e, output| e.add_assertion("sapling_sent_output", output.clone()));
        e = value.orchard_sent_outputs.iter().fold(e, |e, output| e.add_assertion("orchard_sent_output", output.clone()));

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

        let mut scanned_ranges: Vec<ScanRange> = envelope
            .try_objects_for_predicate("scanned_range")?;
        scanned_ranges.sort_by_key(|r| r.start());

        let addresses = envelope_indexed_objects_for_predicate(&envelope, "address")
            .map_err(|e| bc_envelope::Error::General(format!("addresses: {}", e)))?;

        // Deserialize relevant_transactions
        let mut relevant_transactions = HashMap::new();
        for tx_env in envelope.objects_for_predicate("relevant_transaction") {
            tx_env.check_type("RelevantTransaction")?;
            let txid: TxId = tx_env.extract_subject()?;
            let mut outputs: Vec<ReceivedOutput> = tx_env
                .try_objects_for_predicate("output")?;
            outputs.sort_by_key(|o| o.output_index());
            relevant_transactions.insert(txid, outputs);
        }

        let sapling_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "sapling_sent_output")
            .map_err(|e| bc_envelope::Error::General(format!("sapling_sent_outputs: {}", e)))?;
        let orchard_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "orchard_sent_output")
            .map_err(|e| bc_envelope::Error::General(format!("orchard_sent_outputs: {}", e)))?;

        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            index,
            name,
            viewing_key,
            key_source,
            birthday_height,
            birthday_block,
            scanned_ranges,
            addresses,
            relevant_transactions,
            sapling_sent_outputs,
            orchard_sent_outputs,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bc_envelope::Attachments;

    use crate::{
        test_envelope_roundtrip, AccountViewingKey, BlockHash, BlockHeight,
        KeySource, ReceivedOutput, ScanRange, TxId,
    };

    use super::Account;

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
                    .map(|i| ReceivedOutput::new(i as u32, crate::ReceivedOutputPool::Transparent))
                    .collect();
                relevant_transactions.insert(txid, outputs);
            }

            let num_ranges = rng.random_range(0..3usize);
            let mut scanned_ranges: Vec<ScanRange> = (0..num_ranges)
                .map(|_| ScanRange::random())
                .collect();
            scanned_ranges.sort_by_key(|r| r.start());

            Self {
                index: 0,
                name: String::random(),
                viewing_key: AccountViewingKey::random(),
                key_source: KeySource::opt_random(),
                birthday_height: BlockHeight::opt_random(),
                birthday_block: BlockHash::opt_random(),
                scanned_ranges,
                addresses: Vec::random().set_indexes(),
                relevant_transactions,
                sapling_sent_outputs: Vec::random().set_indexes(),
                orchard_sent_outputs: Vec::random().set_indexes(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Account);
}
