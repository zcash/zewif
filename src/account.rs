use minicbor::{Decode, Encode};
use std::collections::BTreeMap;

use crate::{
    AccountViewingKey, Address, BlockHash, BlockHeight, ChainState, Extensions, KeySource,
    ReceivedOutput, ScanRange, SentOutput, TxId,
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
/// a derived `KeySource` having account index 0x7FFFFFFF. This means
/// the account contains both pre-mnemonic randomly-derived addresses
/// (marked `TransparentSpendAuthority::Imported`, with their private keys,
/// if exported, in the secret store) and HD-derived addresses (via
/// `TransparentSpendAuthority::Derived`). The account-level derived
/// `KeySource` indicates that additional addresses *can* be derived from
/// the seed at that account index.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cbor(map)]
pub struct Account {
    /// The account name (may be empty; no uniqueness semantics).
    #[n(0)]
    name: String,

    /// The viewing capability for this account.
    #[n(1)]
    viewing_key: AccountViewingKey,

    /// How the account's keys were obtained.
    #[n(2)]
    key_source: Option<KeySource>,

    /// Minimum block height at which to scan for this account.
    #[n(3)]
    birthday_height: Option<BlockHeight>,

    /// Hash of the birthday block, for chain verification.
    #[n(4)]
    birthday_block: Option<BlockHash>,

    /// Tree state at the end of a block strictly before `birthday_height`
    /// — canonically the block immediately preceding it. Scanning begins at
    /// the following block, so a state at `birthday_height` itself would
    /// cause the birthday block to be skipped (maps to librustzcash
    /// `AccountBirthday` prior_chain_state). Exporters without chain access
    /// omit it.
    #[n(5)]
    birthday_chain_state: Option<ChainState>,

    /// Height (exclusive) up to which scanning of this account's history
    /// counts as recovery rather than regular scanning; typically the chain
    /// tip at the time recovery was initiated (maps to zcash_client_sqlite
    /// accounts.recover_until_height).
    #[n(6)]
    recover_until_height: Option<BlockHeight>,

    /// The capability of the account in the source wallet; `None` = unknown.
    #[n(7)]
    purpose: Option<AccountPurpose>,

    /// Free-form tag identifying the origin of the account's key material,
    /// e.g. "zcashd_mnemonic" (maps to zcash_client_sqlite
    /// accounts.key_source; named provenance here because zewif uses
    /// `KeySource` for the structured enum).
    #[n(8)]
    provenance: Option<String>,

    /// Block ranges that have been fully scanned for this account.
    #[n(9)]
    scanned_ranges: Vec<ScanRange>,

    #[n(10)]
    addresses: Vec<Address>,

    /// Maps transaction IDs to the received outputs relevant to this account.
    #[n(11)]
    relevant_transactions: BTreeMap<TxId, Vec<ReceivedOutput>>,

    /// Sent output metadata not recoverable from the chain, grouped by
    /// the transaction that created them.
    #[n(12)]
    sent_outputs: BTreeMap<TxId, Vec<SentOutput>>,

    #[cbor(n(13), with = "crate::extensions_field", has_nil)]
    extensions: Extensions,
}

/// The capability of an account in the source wallet (maps to
/// zcash_client_backend `AccountPurpose`).
///
/// `ViewOnly` indicates the account was imported without spend authority;
/// a `None` value of a containing `Option` means the purpose is unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum AccountPurpose {
    #[n(0)]
    Spending,
    #[n(1)]
    ViewOnly,
}

impl Account {
    pub fn new(viewing_key: AccountViewingKey) -> Self {
        Self {
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
            relevant_transactions: BTreeMap::new(),
            sent_outputs: BTreeMap::new(),
            extensions: Extensions::new(),
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

    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn relevant_transactions(&self) -> &BTreeMap<TxId, Vec<ReceivedOutput>> {
        &self.relevant_transactions
    }

    pub fn add_relevant_transaction(&mut self, txid: TxId, outputs: Vec<ReceivedOutput>) {
        self.relevant_transactions.insert(txid, outputs);
    }

    pub fn sent_outputs(&self) -> &BTreeMap<TxId, Vec<SentOutput>> {
        &self.sent_outputs
    }

    pub fn add_sent_output(&mut self, txid: TxId, output: SentOutput) {
        self.sent_outputs.entry(txid).or_default().push(output);
    }

    pub fn add_sent_outputs(&mut self, txid: TxId, outputs: Vec<SentOutput>) {
        self.sent_outputs.entry(txid).or_default().extend(outputs);
    }

    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(AccountViewingKey::TransparentAddressSet)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        AccountViewingKey, BlockHash, BlockHeight, ChainState, Extensions, KeySource,
        ReceivedOutput, ScanRange, SentOutput, TransparentOutputData, TxId, test_cbor_roundtrip,
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
            use rand::Rng;

            let mut rng = rand::rng();
            let num_txs = rng.random_range(0..3usize);
            let mut relevant_transactions = BTreeMap::new();
            for _ in 0..num_txs {
                let txid = TxId::random();
                let num_outputs = rng.random_range(1..4usize);
                let outputs: Vec<ReceivedOutput> = (0..num_outputs)
                    .map(|i| {
                        ReceivedOutput::new(
                            i as u32,
                            crate::ReceivedOutputPool::Transparent(TransparentOutputData::new(
                                None, None,
                            )),
                        )
                    })
                    .collect();
                relevant_transactions.insert(txid, outputs);
            }

            let num_sent_txs = rng.random_range(0..3usize);
            let mut sent_outputs = BTreeMap::new();
            for _ in 0..num_sent_txs {
                let txid = TxId::random();
                let num_outputs = rng.random_range(1..3usize);
                let outputs: Vec<SentOutput> =
                    (0..num_outputs).map(|_| SentOutput::random()).collect();
                sent_outputs.insert(txid, outputs);
            }

            let num_ranges = rng.random_range(0..3usize);
            let mut scanned_ranges: Vec<ScanRange> =
                (0..num_ranges).map(|_| ScanRange::random()).collect();
            scanned_ranges.sort_by_key(|r| r.start());

            Self {
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
                addresses: Vec::random(),
                relevant_transactions,
                sent_outputs,
                extensions: Extensions::random(),
            }
        }
    }

    test_cbor_roundtrip!(Account);
}
