//! Golden fixture conformance tests (spec: Reference implementation).
//!
//! The committed files under `tests/fixtures/v1/` are byte-exact golden
//! vectors for container version 1, built by the deterministic constructor
//! functions in this file (hand-written constants only — no randomness, no
//! timestamps). The tests assert byte exactness in both directions, so any
//! behavioral change in the codec dependency — or in this crate's encoders —
//! fails the build instead of silently shipping spec-divergent bytes.

use std::collections::BTreeMap;

use zewif::{
    Account, AccountPurpose, AccountViewingKey, Address, AddressBookEntry, Amount, Bip39Mnemonic,
    Blob, BlockHash, BlockHeight, ChainState, CommitmentTreeData, CompactTxData, Data,
    DerivationInfo, DerivedKeySource, Frontier, FrontierData, IncrementalWitness, KeyScope,
    KeySource, LegacySeed, Memo, MnemonicLanguage, Network, NonHardenedChildIndex,
    OrchardOutputData, ProtocolAddress, RawTxData, ReceivedOutput, ReceivedOutputPool,
    RegtestParams, SaplingExtFvk, SaplingKeyEntry, SaplingOutputData, ScanRange, Script,
    SecretStore, Secrets, SeedEntry, SeedFingerprint, SeedMaterial, SentOutput, SproutKeyEntry,
    SproutOutputData, Transaction, TransactionData, TransparentKeyEntry, TransparentOutputData,
    TreePosition, TxBlockPosition, TxId, UnifiedAddress, UnifiedFullViewingKey, Zewif, ZewifWallet,
    orchard, sapling, sprout, transparent,
};

const MINIMAL_GOLDEN: &[u8] = include_bytes!("fixtures/v1/minimal.zewif");
const FULL_GOLDEN: &[u8] = include_bytes!("fixtures/v1/full.zewif");

/// The transaction carrying every optional field.
const TXID_FULL: [u8; 32] = [0xA1; 32];
/// The transaction carrying almost no optional fields.
const TXID_BARE: [u8; 32] = [0xB2; 32];
/// The transaction carrying compact light-wallet data.
const TXID_COMPACT: [u8; 32] = [0xC3; 32];

/// The fingerprint of the mnemonic seed shared by the derived accounts.
const SEED_FINGERPRINT: [u8; 32] = [0x0F; 32];

/// An empty-ish document: no wallets, transactions, secrets, or metadata.
fn minimal_fixture() -> Zewif {
    Zewif::new(
        BlockHeight::from_u32(2_500_000),
        BlockHash::from_bytes([0x11; 32]),
    )
}

fn sapling_witness() -> sapling::SaplingWitness {
    IncrementalWitness::from_parts(
        sapling::MerkleHashSapling::new([0x51; 32]),
        7,
        vec![
            sapling::MerkleHashSapling::new([0x52; 32]),
            sapling::MerkleHashSapling::new([0x53; 32]),
        ],
        sapling::MerkleHashSapling::new([0x54; 32]),
        12,
        vec![sapling::MerkleHashSapling::new([0x55; 32])],
    )
    .into()
}

fn orchard_witness() -> orchard::OrchardWitness {
    IncrementalWitness::from_parts(
        orchard::MerkleHashOrchard::new([0x61; 32]),
        21,
        vec![orchard::MerkleHashOrchard::new([0x62; 32])],
        orchard::MerkleHashOrchard::new([0x64; 32]),
        40,
        vec![
            orchard::MerkleHashOrchard::new([0x65; 32]),
            orchard::MerkleHashOrchard::new([0x66; 32]),
        ],
    )
    .into()
}

/// A modern UFVK account exercising the birthday fields (including a chain
/// state with one empty and one non-empty frontier), scan ranges, a unified
/// address, witness-form tree data, and sent outputs in all three pools.
fn ufvk_account() -> Account {
    let mut account = Account::new(AccountViewingKey::Ufvk(UnifiedFullViewingKey::new(
        "uview1fixture0account0viewing0key0000000000",
    )));
    account.set_name("Shielded savings");
    account.set_key_source(KeySource::Derived(DerivedKeySource::new(
        SeedFingerprint::new(SEED_FINGERPRINT),
        0,
        None,
    )));
    account.set_birthday_height(BlockHeight::from_u32(1_800_000));
    account.set_birthday_block(BlockHash::from_bytes([0x1B; 32]));
    let mut chain_state = ChainState::new(BlockHeight::from_u32(1_799_999));
    chain_state.set_block_hash(BlockHash::from_bytes([0x1C; 32]));
    chain_state.set_sapling_tree(Frontier::Empty);
    chain_state.set_orchard_tree(Frontier::NonEmpty(FrontierData::from_parts(
        1234,
        Blob::new([0x1D; 32]),
        vec![Blob::new([0x1E; 32]), Blob::new([0x1F; 32])],
    )));
    account.set_birthday_chain_state(chain_state);
    account.set_recover_until_height(BlockHeight::from_u32(2_400_000));
    account.set_purpose(AccountPurpose::Spending);
    account.set_provenance("zcashd_mnemonic");
    account.add_scanned_range(ScanRange::new(
        BlockHeight::from_u32(1_800_000),
        BlockHeight::from_u32(2_000_000),
    ));
    account.add_scanned_range(ScanRange::new(
        BlockHeight::from_u32(2_100_000),
        BlockHeight::from_u32(2_500_000),
    ));

    let mut unified = UnifiedAddress::new("u1fixtureunifiedaddress00000000000000000000");
    unified.set_diversifier_index(Blob::new([0x02; 11]));
    let mut address = Address::new(ProtocolAddress::Unified(Box::new(unified)));
    address.set_scope(KeyScope::External);
    address.set_exposed_at_height(BlockHeight::from_u32(1_850_000));
    // CBOR "red"
    address.extensions_mut().add(
        "org.example.wallet",
        "color",
        Data::from_hex("63726564").unwrap(),
    );
    account.add_address(address);

    // Witness-form tree data with all enrichment fields present.
    let mut sapling_received = ReceivedOutput::new(
        0,
        ReceivedOutputPool::Sapling(SaplingOutputData::new(
            Some(CommitmentTreeData::Witness(sapling_witness())),
            Some(Blob::new([0x5A; 32])),
        )),
    );
    sapling_received.set_value(Amount::const_from_u64(150_000_000));
    sapling_received.set_memo(Some(Memo::new(b"Thanks for lunch!".to_vec())));
    sapling_received.set_is_change(false);
    sapling_received.set_spent_by(TxId::from_bytes(TXID_COMPACT));

    // Witness-form tree data with all enrichment fields absent.
    let orchard_received = ReceivedOutput::new(
        1,
        ReceivedOutputPool::Orchard(OrchardOutputData::new(
            Some(CommitmentTreeData::Witness(orchard_witness())),
            Some(Blob::new([0x6A; 32])),
        )),
    );
    account.add_relevant_transaction(
        TxId::from_bytes(TXID_FULL),
        vec![sapling_received, orchard_received],
    );

    // Position-form tree data with partial enrichment.
    let mut orchard_change = ReceivedOutput::new(
        0,
        ReceivedOutputPool::Orchard(OrchardOutputData::new(
            Some(CommitmentTreeData::Position(TreePosition::new(4_242))),
            None,
        )),
    );
    orchard_change.set_value(Amount::const_from_u64(90_000_000));
    orchard_change.set_is_change(true);
    account.add_relevant_transaction(TxId::from_bytes(TXID_COMPACT), vec![orchard_change]);

    account.add_sent_outputs(
        TxId::from_bytes(TXID_FULL),
        vec![
            SentOutput::Transparent(transparent::TransparentSentOutput::from_parts(
                0,
                "t1fixtureRecipientBob0000000000000".to_string(),
                Amount::const_from_u64(12_500_000),
            )),
            SentOutput::Sapling(sapling::SaplingSentOutput::from_parts(
                1,
                "zs1fixturerecipient0000000000000000000000".to_string(),
                Amount::const_from_u64(25_000_000),
                Some(Memo::new(b"Rent for June".to_vec())),
            )),
            SentOutput::Orchard(orchard::OrchardSentOutput::from_parts(
                2,
                "u1fixturerecipientalice000000000000000000000".to_string(),
                Amount::const_from_u64(50_000_000),
                None,
            )),
        ],
    );

    // CBOR unsigned 42
    account.extensions_mut().add(
        "org.example.wallet",
        "icon",
        Data::from_hex("182a").unwrap(),
    );
    account
}

/// A standalone Sapling account derived via zcashd's legacy post-v4.7.0
/// path, exercising `legacy_address_index` and position-form tree data
/// without enrichment.
fn sapling_account() -> Account {
    let mut account = Account::new(AccountViewingKey::SaplingExtFvk(SaplingExtFvk::new(
        sapling::SaplingExtendedFullViewingKey::new([0x2A; 73]),
    )));
    account.set_name("Legacy Sapling");
    account.set_key_source(KeySource::Derived(DerivedKeySource::new(
        SeedFingerprint::new(SEED_FINGERPRINT),
        0x7FFF_FFFF,
        Some(3),
    )));
    account.set_birthday_height(BlockHeight::from_u32(500_000));
    account.add_scanned_range(ScanRange::new(
        BlockHeight::from_u32(500_000),
        BlockHeight::from_u32(2_500_000),
    ));

    let mut zaddr = sapling::Address::new("zs1fixturesaplingaddress000000000000000000");
    zaddr.set_diversifier_index(Blob::new([0x00; 11]));
    let mut address = Address::new(ProtocolAddress::Sapling(Box::new(zaddr)));
    address.set_scope(KeyScope::Internal);
    address.set_exposed_at_height(BlockHeight::from_u32(600_000));
    account.add_address(address);

    // Position-form tree data with no enrichment at all.
    let sapling_received = ReceivedOutput::new(
        2,
        ReceivedOutputPool::Sapling(SaplingOutputData::new(
            Some(CommitmentTreeData::Position(TreePosition::new(98_765))),
            None,
        )),
    );
    account.add_relevant_transaction(TxId::from_bytes(TXID_FULL), vec![sapling_received]);
    account
}

/// An imported, view-only Sprout account.
fn sprout_account() -> Account {
    let mut account = Account::new(AccountViewingKey::SproutViewingKey(
        sprout::SproutViewingKey::new(Data::from_slice(&[0x3C; 64])),
    ));
    account.set_name("Sprout relic");
    account.set_key_source(KeySource::Imported);
    account.set_purpose(AccountPurpose::ViewOnly);
    account.set_provenance("zcashd_imported");

    let mut address = Address::new(ProtocolAddress::Sprout(sprout::SproutAddress::new(
        "zcfixturesproutaddress00000000000000000000000000000000000000000000000000000000000000000000000",
    )));
    address.set_scope(KeyScope::External);
    address.set_exposed_at_height(BlockHeight::from_u32(120_000));
    account.add_address(address);

    let mut sprout_received = ReceivedOutput::new(
        3,
        ReceivedOutputPool::Sprout(SproutOutputData::new(Some(Blob::new([0x3D; 32])))),
    );
    sprout_received.set_value(Amount::const_from_u64(300_000_000));
    account.add_relevant_transaction(TxId::from_bytes(TXID_BARE), vec![sprout_received]);
    account
}

/// A legacy zcashd hybrid account: a transparent address set backed by the
/// mnemonic at the legacy account index, with derived, imported, and
/// watch-only addresses.
fn transparent_account() -> Account {
    let mut account = Account::new(AccountViewingKey::TransparentAddressSet);
    account.set_name("Legacy transparent");
    account.set_key_source(KeySource::Derived(DerivedKeySource::new(
        SeedFingerprint::new(SEED_FINGERPRINT),
        0x7FFF_FFFF,
        None,
    )));

    // An HD-derived transparent address.
    let mut derived = transparent::Address::new("t1fixtureDerivedAddr00000000000000");
    derived.set_spend_authority(transparent::TransparentSpendAuthority::Derived(
        DerivationInfo::new(
            NonHardenedChildIndex::from(0u32),
            NonHardenedChildIndex::from(5u32),
        ),
    ));
    let mut address = Address::new(ProtocolAddress::Transparent(derived));
    address.set_scope(KeyScope::External);
    address.set_exposed_at_height(BlockHeight::from_u32(2_000_000));
    account.add_address(address);

    // An address whose independently-generated private key lives in the
    // secret store.
    let mut imported = transparent::Address::new("t1fixtureImportedAddr0000000000000");
    imported.set_spend_authority(transparent::TransparentSpendAuthority::Imported);
    let mut address = Address::new(ProtocolAddress::Transparent(imported));
    address.set_scope(KeyScope::Foreign);
    address.set_exposed_at_height(BlockHeight::from_u32(1_500_000));
    account.add_address(address);

    // A watch-only address imported by public key (zcashd importpubkey).
    let mut watch_pubkey = transparent::Address::new("t1fixtureWatchOnlyPubkey0000000000");
    watch_pubkey.set_pubkey(Data::from_slice(&[0x02; 33]));
    let mut address = Address::new(ProtocolAddress::Transparent(watch_pubkey));
    address.set_scope(KeyScope::Foreign);
    account.add_address(address);

    // A watch-only P2SH address imported by script (zcashd importaddress).
    let mut watch_script = transparent::Address::new("t3fixtureWatchOnlyScript0000000000");
    watch_script.set_redeem_script(Script::from(Data::from_hex("52210300000051ae").unwrap()));
    let mut address = Address::new(ProtocolAddress::Transparent(watch_script));
    address.set_scope(KeyScope::Foreign);
    account.add_address(address);

    // A transparent UTXO with the enrichment fields present...
    let mut enriched = ReceivedOutput::new(
        0,
        ReceivedOutputPool::Transparent(TransparentOutputData::new(
            Some(Script::from(
                Data::from_hex("76a91400000000000088ac").unwrap(),
            )),
            Some(BlockHeight::from_u32(2_450_000)),
        )),
    );
    enriched.set_value(Amount::const_from_u64(75_000_000));
    enriched.set_is_change(true);
    enriched.set_spent_by(TxId::from_bytes(TXID_COMPACT));
    // ...and one with them all absent.
    let bare = ReceivedOutput::new(
        1,
        ReceivedOutputPool::Transparent(TransparentOutputData::new(None, None)),
    );
    account.add_relevant_transaction(TxId::from_bytes(TXID_FULL), vec![enriched, bare]);
    account
}

fn full_wallet() -> ZewifWallet {
    // Consensus branch IDs per ZIP 200: Sapling and NU5.
    let mut wallet = ZewifWallet::new(Network::Regtest(RegtestParams::new(BTreeMap::from([
        (0x76B8_09BB, 1),
        (0xC2D6_D0B4, 10),
    ]))));
    wallet.add_account(ufvk_account());
    wallet.add_account(sapling_account());
    wallet.add_account(sprout_account());
    wallet.add_account(transparent_account());

    let mut alice = AddressBookEntry::new("u1fixturerecipientalice000000000000000000000");
    alice.set_label("Alice");
    alice.set_purpose("send");
    // CBOR true
    alice.extensions_mut().add(
        "org.example.wallet",
        "verified",
        Data::from_hex("f5").unwrap(),
    );
    wallet.add_address_book_entry(alice);

    let mut bob = AddressBookEntry::new("t1fixtureRecipientBob0000000000000");
    bob.set_label("Bob");
    wallet.add_address_book_entry(bob);

    // CBOR "dark"
    wallet.extensions_mut().add(
        "org.example.wallet",
        "theme",
        Data::from_hex("656461726b").unwrap(),
    );
    wallet
}

/// A transaction with every optional field present, including
/// `trusted = true`.
fn full_transaction() -> Transaction {
    let mut tx = Transaction::new(TxId::from_bytes(TXID_FULL));
    tx.set_tx_data(TransactionData::Raw(RawTxData::new(
        Data::from_hex("050000800a27a726b4d0d6c200000000").unwrap(),
    )));
    tx.set_target_height(BlockHeight::from_u32(2_100_010));
    tx.set_mined_height(BlockHeight::from_u32(2_100_012));
    tx.set_block_position(TxBlockPosition::new(BlockHash::from_bytes([0xBB; 32]), 3));
    tx.set_fee(Amount::const_from_u64(10_000));
    tx.set_expiry_height(BlockHeight::from_u32(2_100_050));
    // 2025-01-01T00:00:00Z, a hand-written constant (never the current time).
    tx.set_created_time(1_735_689_600);
    tx.set_trusted(true);
    // CBOR "gift"
    tx.extensions_mut().add(
        "org.example.wallet",
        "note",
        Data::from_hex("6467696674").unwrap(),
    );
    tx
}

/// A transaction with almost no optional fields: only the txid.
fn bare_transaction() -> Transaction {
    Transaction::new(TxId::from_bytes(TXID_BARE))
}

/// A transaction whose data is the compact light-wallet representation.
fn compact_transaction() -> Transaction {
    let mut tx = Transaction::new(TxId::from_bytes(TXID_COMPACT));
    tx.set_tx_data(TransactionData::Compact(CompactTxData::new(
        Data::from_hex("0a0408011201ff").unwrap(),
        "1.0.0",
    )));
    tx.set_mined_height(BlockHeight::from_u32(2_200_000));
    tx
}

/// A plain secret store containing at least one of every entry kind, with
/// both seed-material variants represented.
fn secret_store() -> SecretStore {
    let mut store = SecretStore::new();
    store.add_seed(SeedEntry::new(
        SeedFingerprint::new(SEED_FINGERPRINT),
        SeedMaterial::Bip39Mnemonic(Bip39Mnemonic::new(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            Some(MnemonicLanguage::English),
        )),
    ));
    store.add_seed(SeedEntry::new(
        SeedFingerprint::new([0x9E; 32]),
        SeedMaterial::LegacySeed(LegacySeed::new(Data::from_slice(&[0x9F; 32]))),
    ));
    store.add_transparent_key(TransparentKeyEntry::new(
        Data::from_slice(&[0x02; 33]),
        transparent::TransparentSpendingKey::new([0x4B; 32]),
    ));
    store.add_sapling_key(SaplingKeyEntry::new(
        Data::from_slice(&[0x2A; 73]),
        sapling::SaplingExtendedSpendingKey::new([0x5B; 169]),
    ));
    store.add_sprout_key(SproutKeyEntry::new(
        "zcfixturesproutaddress00000000000000000000000000000000000000000000000000000000000000000000000",
        sprout::SproutSpendingKey::new(Data::from_slice(&[0x6B; 32])),
    ));
    // CBOR 100
    store.extensions_mut().add(
        "org.example.wallet",
        "kdf-hint",
        Data::from_hex("1864").unwrap(),
    );
    store
}

/// A comprehensive document exercising every record type and union variant
/// of the version-1 schema.
fn full_fixture() -> Zewif {
    let mut zewif = Zewif::new(
        BlockHeight::from_u32(2_500_000),
        BlockHash::from_bytes([0xEC; 32]),
    );
    zewif.add_wallet(full_wallet());
    for tx in [
        full_transaction(),
        bare_transaction(),
        compact_transaction(),
    ] {
        zewif.add_transaction(tx.txid(), tx);
    }
    zewif.set_secrets(Secrets::Plain(secret_store()));
    zewif.set_export_id(Blob::new(*b"ZEWIF-fixture-export-id-32bytes!"));
    zewif.set_embedded_schema("zewif = {0: [* wallet], 1: [* transaction]} ; abridged");
    // CBOR "1.2.3"
    zewif.extensions_mut().add(
        "org.example.exporter",
        "version",
        Data::from_hex("65312e322e33").unwrap(),
    );
    zewif
}

#[test]
fn minimal_to_bytes_matches_golden() {
    assert_eq!(minimal_fixture().to_bytes().unwrap(), MINIMAL_GOLDEN);
}

#[test]
fn minimal_from_bytes_matches_fixture() {
    assert_eq!(
        Zewif::from_bytes(MINIMAL_GOLDEN).unwrap(),
        minimal_fixture()
    );
}

#[test]
fn full_to_bytes_matches_golden() {
    assert_eq!(full_fixture().to_bytes().unwrap(), FULL_GOLDEN);
}

#[test]
fn full_from_bytes_matches_fixture() {
    assert_eq!(Zewif::from_bytes(FULL_GOLDEN).unwrap(), full_fixture());
}

/// Rewrites the committed fixture files from the constructors in this file:
/// `cargo test --test golden regenerate_fixtures -- --ignored`.
///
/// Regenerating these fixtures is only legitimate alongside a deliberate,
/// spec-versioned change to the ZeWIF format. The committed bytes are the
/// normative reference for container version 1; if this test's output ever
/// differs from them without an accompanying specification change, the
/// encoder (or a codec dependency) has diverged from the spec, and the
/// divergence is the bug — not the fixtures.
#[test]
#[ignore = "writes the golden fixture files; see the doc comment"]
fn regenerate_fixtures() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/v1");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("minimal.zewif"),
        minimal_fixture().to_bytes().unwrap(),
    )
    .unwrap();
    std::fs::write(dir.join("full.zewif"), full_fixture().to_bytes().unwrap()).unwrap();
}
