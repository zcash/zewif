//! Validates the CBOR emitted by this crate against the normative CDDL
//! schema in `docs/zewif.cddl` (kept in lockstep with the ZIP draft by
//! `tests/schema_sync.rs`).
//!
//! The document under test is a deterministically constructed [`Zewif`]
//! value exercising every schema production reachable through the crate's
//! public API: all four account viewing key variants, all four protocol
//! address kinds, all received/sent output pools, both transaction data
//! representations, both seed material forms, regtest network parameters,
//! chain-state frontiers, and extension maps at every level.
//!
//! The `incremental-witness` production is covered via the golden fixture
//! document (`tests/fixtures/v1/full.zewif`), whose payload is validated
//! against the schema below.

use std::collections::BTreeMap;
use std::path::Path;

use zewif::{
    Account, AccountPurpose, AccountViewingKey, Address, AddressBookEntry, Amount, Bip39Mnemonic,
    Blob, BlockHash, BlockHeight, ChainState, CommitmentTreeData, CompactTxData, Data,
    DerivationInfo, DerivedKeySource, Frontier, FrontierData, KeyScope, KeySource, LegacySeed,
    Memo, MnemonicLanguage, Network, OrchardOutputData, ProtocolAddress, RawTxData, ReceivedOutput,
    ReceivedOutputPool, RegtestParams, SaplingExtFvk, SaplingKeyEntry, SaplingOutputData,
    ScanRange, Script, SecretStore, Secrets, SeedEntry, SeedFingerprint, SeedMaterial, SentOutput,
    SproutKeyEntry, SproutOutputData, Transaction, TransactionData, TransparentKeyEntry,
    TransparentOutputData, TreePosition, TxBlockPosition, TxId, UnifiedAddress,
    UnifiedFullViewingKey, Zewif, ZewifWallet, orchard, sapling, sprout, transparent,
};

fn height(h: u32) -> BlockHeight {
    BlockHeight::from_u32(h)
}

fn zats(value: u64) -> Amount {
    Amount::from_u64(value).expect("value is within [0, MAX_MONEY]")
}

/// An account exercising the UFVK viewing key, HD-derived key source,
/// birthday metadata, all four protocol address kinds, all received output
/// pools, and all sent output pools.
fn ufvk_account(txid1: TxId, txid2: TxId) -> Account {
    let mut account = Account::new(AccountViewingKey::Ufvk(UnifiedFullViewingKey::new(
        "uview1conformance0testvector",
    )));
    account.set_name("primary");
    account.set_key_source(KeySource::Derived(DerivedKeySource::new(
        SeedFingerprint::new([0x5F; 32]),
        0,
        Some(3),
    )));
    account.set_birthday_height(height(1_687_104));
    account.set_birthday_block(BlockHash::from_bytes([0xBB; 32]));
    let mut chain_state = ChainState::new(height(1_687_103));
    chain_state.set_block_hash(BlockHash::from_bytes([0xBC; 32]));
    chain_state.set_sapling_tree(Frontier::NonEmpty(FrontierData::from_parts(
        1234,
        Blob::new([0xCC; 32]),
        vec![Blob::new([0xCD; 32]), Blob::new([0xCE; 32])],
    )));
    chain_state.set_orchard_tree(Frontier::Empty);
    account.set_birthday_chain_state(chain_state);
    account.set_recover_until_height(height(2_100_000));
    account.set_purpose(AccountPurpose::Spending);
    account.set_provenance("zcashd_mnemonic");
    account.add_scanned_range(ScanRange::new(height(1_687_104), height(1_900_000)));
    account.add_scanned_range(ScanRange::new(height(1_950_000), height(2_000_000)));

    // Transparent, HD-derived.
    let mut t_derived = transparent::Address::new("t1derivedconformanceaddress000000");
    t_derived.set_spend_authority(transparent::TransparentSpendAuthority::Derived(
        DerivationInfo::new(0u32.into(), 7u32.into()),
    ));
    let mut addr = Address::new(ProtocolAddress::Transparent(t_derived));
    addr.set_scope(KeyScope::External);
    addr.set_exposed_at_height(height(1_700_000));
    account.add_address(addr);

    // Transparent, imported with spend authority in the secret store.
    let mut t_imported = transparent::Address::new("t1importedconformanceaddress00000");
    t_imported.set_spend_authority(transparent::TransparentSpendAuthority::Imported);
    account.add_address(Address::new(ProtocolAddress::Transparent(t_imported)));

    // Transparent, watch-only P2SH imported by script.
    let mut t_watch = transparent::Address::new("t3watchonlyconformanceaddress0000");
    t_watch.set_redeem_script(Script::from(Data::from_slice(&[0x52, 0x21, 0x03, 0xAE])));
    let mut addr = Address::new(ProtocolAddress::Transparent(t_watch));
    addr.set_scope(KeyScope::Foreign);
    account.add_address(addr);

    // Sprout.
    account.add_address(Address::new(ProtocolAddress::Sprout(
        sprout::SproutAddress::new("zcconformancesproutaddress"),
    )));

    // Sapling, with diversifier index.
    let mut zs = sapling::Address::new("zsconformancesaplingaddress");
    zs.set_diversifier_index(Blob::new([0x0D; 11]));
    account.add_address(Address::new(ProtocolAddress::Sapling(Box::new(zs))));

    // Unified, with diversifier index.
    let mut ua = UnifiedAddress::new("u1conformanceunifiedaddress");
    ua.set_diversifier_index(Blob::new([0x0E; 11]));
    let mut addr = Address::new(ProtocolAddress::Unified(Box::new(ua)));
    addr.set_scope(KeyScope::Internal);
    addr.set_exposed_at_height(height(1_800_000));
    addr.extensions_mut()
        .add("land.nutty.zewif-test", "address-note", embedded_cbor());
    account.add_address(addr);

    // Received outputs covering all four pools.
    let mut transparent_received = ReceivedOutput::new(
        0,
        ReceivedOutputPool::Transparent(TransparentOutputData::new(
            Some(Script::from(Data::from_slice(&[0x76, 0xA9, 0x14, 0x00]))),
            Some(height(2_000_100)),
        )),
    );
    transparent_received.set_value(zats(50_000));
    transparent_received.set_is_change(false);
    transparent_received.set_spent_by(txid2);

    let mut sapling_received = ReceivedOutput::new(
        1,
        ReceivedOutputPool::Sapling(SaplingOutputData::new(
            Some(CommitmentTreeData::Position(TreePosition::new(987_654))),
            Some(Blob::new([0xAB; 32])),
        )),
    );
    sapling_received.set_value(zats(1_2500_0000));
    sapling_received.set_memo(Some(Memo::from_slice(b"conformance test memo")));
    sapling_received.set_is_change(false);

    let mut orchard_received = ReceivedOutput::new(
        0,
        ReceivedOutputPool::Orchard(OrchardOutputData::new(
            Some(CommitmentTreeData::Position(TreePosition::new(31_337))),
            Some(Blob::new([0xAC; 32])),
        )),
    );
    orchard_received.set_value(zats(7_0000_0000));
    orchard_received.set_is_change(true);

    account.add_relevant_transaction(
        txid1,
        vec![transparent_received, sapling_received, orchard_received],
    );

    let sprout_received = ReceivedOutput::new(
        3, // JoinSplit 1, output 1
        ReceivedOutputPool::Sprout(SproutOutputData::new(Some(Blob::new([0xAD; 32])))),
    );
    account.add_relevant_transaction(txid2, vec![sprout_received]);

    // Sent outputs covering all three pools.
    account.add_sent_outputs(
        txid2,
        vec![
            SentOutput::Transparent(transparent::TransparentSentOutput::from_parts(
                0,
                "t1recipientconformanceaddress0000".to_string(),
                zats(25_000),
            )),
            SentOutput::Sapling(sapling::SaplingSentOutput::from_parts(
                0,
                "zsrecipientconformanceaddress".to_string(),
                zats(5000_0000),
                Some(Memo::from_slice(b"sapling payment")),
            )),
            SentOutput::Orchard(orchard::OrchardSentOutput::from_parts(
                1,
                "u1recipientconformanceaddress".to_string(),
                zats(1_0000_0000),
                None,
            )),
        ],
    );

    account
        .extensions_mut()
        .add("land.nutty.zewif-test", "account-note", embedded_cbor());
    account
}

/// A view-only account with a standalone Sapling extended full viewing key.
fn sapling_account() -> Account {
    let mut account = Account::new(AccountViewingKey::SaplingExtFvk(SaplingExtFvk::new(
        sapling::SaplingExtendedFullViewingKey::new([0x73; 73]),
    )));
    account.set_name("sapling import");
    account.set_key_source(KeySource::Imported);
    account.set_purpose(AccountPurpose::ViewOnly);
    account.add_scanned_range(ScanRange::new(height(419_200), height(500_000)));
    account.add_address(Address::new(ProtocolAddress::Sapling(Box::new(
        sapling::Address::new("zsimportedconformanceaddress"),
    ))));
    account
}

/// A legacy Sprout account.
fn sprout_account() -> Account {
    let mut account = Account::new(AccountViewingKey::SproutViewingKey(
        sprout::SproutViewingKey::new(Data::from_slice(&[0x64; 64])),
    ));
    account.set_name("sprout legacy");
    account.add_address(Address::new(ProtocolAddress::Sprout(
        sprout::SproutAddress::new("zcregtestsproutaddress"),
    )));
    account
}

/// A legacy zcashd random-key account represented as a transparent
/// address set.
fn transparent_set_account() -> Account {
    let mut account = Account::new(AccountViewingKey::TransparentAddressSet);
    account.set_name("zcashd legacy keypool");
    let mut t_addr = transparent::Address::new("tmregtestconformanceaddress000000");
    t_addr.set_spend_authority(transparent::TransparentSpendAuthority::Imported);
    account.add_address(Address::new(ProtocolAddress::Transparent(t_addr)));
    // A watch-only P2PKH address imported by public key.
    let mut t_watch = transparent::Address::new("tmregtestwatchonlyaddress00000000");
    t_watch.set_pubkey(pubkey_bytes(0x03));
    account.add_address(Address::new(ProtocolAddress::Transparent(t_watch)));
    account
}

/// A byte string with the shape of a compressed secp256k1 public key.
fn pubkey_bytes(prefix: u8) -> Data {
    let mut pk = vec![prefix];
    pk.extend_from_slice(&[0x42; 32]);
    Data::from_vec(pk)
}

/// An extension value: a byte string containing a single embedded CBOR
/// data item.
fn embedded_cbor() -> Data {
    Data::from_vec(minicbor::to_vec("embedded item").expect("encoding a str cannot fail"))
}

/// The global transaction table: raw, compact, and txid-only entries.
fn transactions(txid1: TxId, txid2: TxId, txid3: TxId) -> BTreeMap<TxId, Transaction> {
    let mut tx1 = Transaction::new(txid1);
    tx1.set_tx_data(TransactionData::Raw(RawTxData::new(Data::from_slice(&[
        0x05, 0x00, 0x00, 0x80, 0xDE, 0xAD, 0xBE, 0xEF,
    ]))));
    tx1.set_target_height(height(2_000_000));
    tx1.set_mined_height(height(2_000_001));
    tx1.set_block_position(TxBlockPosition::new(BlockHash::from_bytes([0xAA; 32]), 3));
    tx1.set_fee(zats(10_000));
    tx1.set_expiry_height(height(2_000_040));
    tx1.set_created_time(1_700_000_000);
    tx1.set_trusted(true);
    tx1.extensions_mut()
        .add("land.nutty.zewif-test", "tx-note", embedded_cbor());

    let mut tx2 = Transaction::new(txid2);
    tx2.set_tx_data(TransactionData::Compact(CompactTxData::new(
        Data::from_slice(&[0x0A, 0x04, 0x01, 0x02, 0x03, 0x04]),
        "1.0.0",
    )));
    tx2.set_mined_height(height(2_000_010));

    // Only the txid is known.
    let tx3 = Transaction::new(txid3);

    [tx1, tx2, tx3]
        .into_iter()
        .map(|tx| (tx.txid(), tx))
        .collect()
}

/// The secret store: both seed material forms plus one key entry of each
/// protocol kind.
fn secret_store() -> SecretStore {
    let mut store = SecretStore::new();
    store.add_seed(SeedEntry::new(
        SeedFingerprint::new([0x5F; 32]),
        SeedMaterial::Bip39Mnemonic(Bip39Mnemonic::new(
            "abandon abandon abandon abandon abandon abandon abandon abandon \
             abandon abandon abandon about",
            Some(MnemonicLanguage::English),
        )),
    ));
    store.add_seed(SeedEntry::new(
        SeedFingerprint::new([0x60; 32]),
        SeedMaterial::LegacySeed(LegacySeed::new(Data::from_slice(&[0x99; 32]))),
    ));
    store.add_transparent_key(TransparentKeyEntry::new(
        pubkey_bytes(0x02),
        transparent::TransparentSpendingKey::new([0x77; 32]),
    ));
    store.add_sapling_key(SaplingKeyEntry::new(
        Data::from_slice(&[0x73; 73]),
        sapling::SaplingExtendedSpendingKey::new([0x69; 169]),
    ));
    store.add_sprout_key(SproutKeyEntry::new(
        "zcconformancesproutaddress",
        sprout::SproutSpendingKey::new(Data::from_slice(&[0x88; 32])),
    ));
    store
        .extensions_mut()
        .add("land.nutty.zewif-test", "secret-note", embedded_cbor());
    store
}

/// A deterministic ZeWIF document exercising every schema production
/// reachable through the crate's public API.
fn rich_zewif(schema: &str) -> Zewif {
    let txid1 = TxId::from_bytes([0x11; 32]);
    let txid2 = TxId::from_bytes([0x22; 32]);
    let txid3 = TxId::from_bytes([0x33; 32]);

    let mut mainnet_wallet = ZewifWallet::new(Network::Mainnet);
    mainnet_wallet.add_account(ufvk_account(txid1, txid2));
    mainnet_wallet.add_account(sapling_account());
    let mut entry = AddressBookEntry::new("u1recipientconformanceaddress");
    entry.set_label("counterparty");
    entry.set_purpose("send");
    entry
        .extensions_mut()
        .add("land.nutty.zewif-test", "book-note", embedded_cbor());
    mainnet_wallet.add_address_book_entry(entry);
    mainnet_wallet
        .extensions_mut()
        .add("land.nutty.zewif-test", "wallet-note", embedded_cbor());

    let mut regtest_wallet = ZewifWallet::new(Network::Regtest(RegtestParams::new(
        [(0xC2D6_D0B8u32, 1u32), (0xF5B9_230Bu32, 100u32)]
            .into_iter()
            .collect(),
    )));
    regtest_wallet.add_account(sprout_account());
    regtest_wallet.add_account(transparent_set_account());
    regtest_wallet
        .add_address_book_entry(AddressBookEntry::new("tmregtestconformanceaddress000000"));

    let mut zewif = Zewif::new(height(2_500_000), BlockHash::from_bytes([0xEE; 32]));
    zewif.add_wallet(mainnet_wallet);
    zewif.add_wallet(regtest_wallet);
    zewif.set_transactions(transactions(txid1, txid2, txid3));
    zewif.set_secrets(Secrets::Plain(secret_store()));
    zewif.set_export_id(Blob::new([0x1D; 32]));
    zewif.set_embedded_schema(schema);
    zewif
        .extensions_mut()
        .add("land.nutty.zewif-test", "export-note", embedded_cbor());
    zewif
}

fn load_schema() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(manifest_dir.join("docs/zewif.cddl"))
        .expect("machine-readable schema is readable")
}

#[test]
fn emitted_cbor_conforms_to_schema() {
    let schema = load_schema();
    let document = rich_zewif(&schema);
    let bytes = minicbor::to_vec(&document).expect("CBOR encoding succeeds");

    if let Err(diagnostics) = cddl_cat::validate_cbor_bytes("zewif", &schema, &bytes) {
        panic!(
            "the emitted CBOR does not conform to docs/zewif.cddl:\n{}",
            diagnostics
        );
    }
}

/// A minimal viewing-only export with an age-encrypted secret store also
/// conforms (the `[1, [encrypted-store]]` branch of `secrets`).
#[test]
fn encrypted_secrets_document_conforms_to_schema() {
    let schema = load_schema();
    let mut document = Zewif::new(height(2_500_000), BlockHash::from_bytes([0xEE; 32]));
    document.set_secrets(Secrets::Encrypted(zewif::EncryptedStore::new(
        Data::from_slice(b"age-encryption.org/v1 conformance ciphertext"),
    )));
    let bytes = minicbor::to_vec(&document).expect("CBOR encoding succeeds");

    if let Err(diagnostics) = cddl_cat::validate_cbor_bytes("zewif", &schema, &bytes) {
        panic!(
            "the emitted CBOR does not conform to docs/zewif.cddl:\n{}",
            diagnostics
        );
    }
}

/// The full golden fixture document's payload conforms to the schema. This
/// ties the byte-exact fixture suite to the schema validator and covers
/// productions the in-test document does not construct (notably
/// `incremental-witness`, present in the fixture in both shielded pools).
#[test]
fn golden_fixture_payload_conforms_to_schema() {
    let schema = load_schema();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let document = std::fs::read(manifest_dir.join("tests/fixtures/v1/full.zewif"))
        .expect("golden fixture is readable");
    // Strip the container header: 5 magic bytes + 4-byte LE version.
    let payload = &document[zewif::MAGIC_BYTES.len() + 4..];

    if let Err(diagnostics) = cddl_cat::validate_cbor_bytes("zewif", &schema, payload) {
        panic!(
            "the golden fixture payload does not conform to docs/zewif.cddl:\n{}",
            diagnostics
        );
    }
}

/// Guards against a vacuously accepting validator: a document violating the
/// schema (31-byte block hash where `bytes32` is required) must be rejected.
#[test]
fn validator_rejects_nonconformant_document() {
    let schema = load_schema();

    let mut bytes = Vec::new();
    let mut encoder = minicbor::Encoder::new(&mut bytes);
    encoder.map(4).expect("map header");
    encoder.u8(0).expect("key").array(0).expect("wallets");
    encoder.u8(1).expect("key").array(0).expect("transactions");
    encoder.u8(2).expect("key").u32(100).expect("export height");
    encoder
        .u8(3)
        .expect("key")
        .bytes(&[0u8; 31])
        .expect("truncated block hash");

    assert!(
        cddl_cat::validate_cbor_bytes("zewif", &schema, &bytes).is_err(),
        "a document with a 31-byte block hash must not validate against \
         docs/zewif.cddl"
    );
}
