# ZeWIF — Zcash Wallet Interchange Format

ZeWIF is a standard data format for migrating wallet state between Zcash wallet
implementations. It captures the complete set of information a wallet needs to
resume operation — accounts, viewing keys, addresses, transaction history, and
note commitment witnesses — in a single serializable container. ZeWIF is
designed to be produced and consumed by any Zcash wallet, regardless of its
internal storage format.

## Type Hierarchy

A ZeWIF export is structured as follows:

- **`Zewif`** — the root container. Holds one or more wallets, a global
  transaction store (keyed by `TxId`), and export metadata (the block height
  and hash at the time of export, used by importers for chain verification).

- **`ZewifWallet`** — a wallet within the export. Carries the network
  (mainnet/testnet), optional seed material (BIP-39 mnemonic or legacy raw
  seed), and a list of accounts.

- **`Account`** — a logical grouping of funds. Centered on an
  `AccountViewingKey` that determines what the account can observe on-chain,
  with an optional `KeySource` recording how the keys were obtained. Contains
  derived addresses, references to relevant transactions, received output
  metadata, sent output metadata, and scan-progress tracking.

- **`Address`** — a wallet address wrapping a `ProtocolAddress` (transparent,
  Sprout, Sapling, or unified). Carries no user-facing metadata; labels and
  contact information belong in attachments.

## Design Principles

### Canonical encodings everywhere

Wherever a standard encoding exists for a value, ZeWIF stores that encoding
directly rather than decomposing it into parsed components. Unified full viewing
keys are stored as their ZIP-316 string encoding. Transactions are stored as
raw serialized bytes (or compact protobuf bytes from a light wallet server).
Sapling extended full viewing keys and Sprout spending keys use their respective
canonical wire formats. This keeps ZeWIF from needing to understand protocol
internals, avoids inconsistency between parsed fields, and ensures that any
wallet capable of using a value can simply parse the canonical encoding it
already understands.

### Minimal per-note detail

For received outputs, ZeWIF stores only the output index within the
transaction, a pool tag (transparent/Sapling/Orchard), and an optional note
commitment tree witness. Value, nullifier, memo, and other per-note data are
recoverable from the raw transaction combined with the account's viewing key.
This avoids duplicating information that can be derived, eliminating a class of
inconsistency bugs in export data.

### Spending key separability

Spending key material is architecturally separate from viewing key material. For
HD-derived accounts, spending keys are recoverable from the wallet's seed
material plus the derivation metadata in `KeySource` — no spending keys are
stored in the account. For legacy zcashd transparent addresses with
independently-generated keys, each address carries its own private key via its
`TransparentSpendAuthority`. This separation enables different security
treatment of spending and viewing data (e.g., separate encryption layers or
differential access control).

### Address types for cryptographic data only

Address types contain only protocol-level cryptographic and derivation data. No
user-facing metadata — names, labels, contact information, purpose tags — lives
on address types. That information belongs in wallet-level attachments, keeping
the core data model focused on what is needed for cryptographic operations and
key derivation.

### Extensibility via attachments

Every major type in ZeWIF (`Zewif`, `ZewifWallet`, `Account`, `Address`,
`Transaction`) supports an attachments map for wallet-specific metadata. This
is the extension point for data like address books, private notes on
transactions, or any other wallet-specific information that doesn't belong in
the core interchange format. Attachment types will be standardized as specific
wallet export implementations are built.

## Account Model

An account is fundamentally defined by its viewing capability, represented by
the `AccountViewingKey` enum:

- **`Ufvk`** — a Unified Full Viewing Key (ZIP-316 encoding), which may contain
  Orchard, Sapling, and/or transparent receiver components.
- **`SaplingExtFvk`** — a standalone Sapling extended full viewing key.
- **`SproutViewingKey`** — a Sprout viewing key (64 bytes: paying key `a_pk` +
  receiving key `sk_enc`), sufficient to detect incoming Sprout notes.
- **`TransparentAddressSet`** — a set of transparent addresses with no unified
  key structure, used for legacy zcashd random-key wallets.

The `KeySource` enum records how the account's keys were obtained:

- **`Derived`** — derived from an HD seed via ZIP-32, identified by a seed
  fingerprint and ZIP-32 account index.
- **`Imported`** — imported directly as a standalone viewing key.

### Legacy zcashd hybrid accounts

When zcashd introduced mnemonic seeds in v4.7.0, it created a hybrid situation:
the legacy account contains both pre-mnemonic randomly-generated addresses and
HD-derived addresses under account index `0x7FFFFFFF`. ZeWIF models this as an
account with `AccountViewingKey::TransparentAddressSet` combined with
`KeySource::Derived { account_index: 0x7FFFFFFF, .. }`. The account-level
derivation metadata indicates that additional addresses *can* be derived from
the seed at that index, while each individual transparent address carries its
own `TransparentSpendAuthority` distinguishing whether it was HD-derived or
independently generated.

## Transaction Model

Transactions are stored in a global map at the `Zewif` root level, keyed by
`TxId`. Accounts reference transactions by ID rather than embedding them,
avoiding duplication when multiple accounts observe the same transaction.

Each `Transaction` carries:

- **`tx_data`** — optional transaction data, either full raw bytes
  (`TransactionData::Raw`) or compact light-wallet protobuf bytes
  (`TransactionData::Compact`, tagged with a protocol version string since the
  protobuf schema is not self-describing).
- **`mined_height`** and **`block_position`** — blockchain location, if the
  transaction has been mined.
- **`fee`** and **`expiry_height`** — consensus-relevant metadata tracked by
  wallets.
- **`target_height`** — the consensus branch height the transaction targets.

### Scan ranges

Each account tracks which block ranges have been fully scanned via a list of
`ScanRange` values (inclusive start and end heights). This allows an importing
wallet to identify gaps in scanning history and avoid re-scanning blocks that
have already been processed. Scan ranges are per-account because different
accounts may have different birthday heights and scanning histories.

## Serialization Format

ZeWIF uses [Gordian Envelope](https://github.com/BlockchainCommons/BCSwiftSecureComponents)
over CBOR as its serialization format. An Envelope is a recursive structure
where a subject (the primary value) is annotated with assertions (predicate-
object pairs). Each ZeWIF type maps to an Envelope: the type's identity or
primary key becomes the subject, and its fields become assertions keyed by
string predicates.

This structure provides:

- **Self-describing data** — each envelope carries type tags and named
  predicates, making the format inspectable without a schema.
- **Unordered assertions** — fields are assertions on a subject, so field order
  does not affect semantics (ordered collections use explicit index values).
- **Selective disclosure** — the envelope model supports elision and encryption
  of individual assertions, enabling spending key separability at the
  serialization level.

## Protocol Support

ZeWIF supports all Zcash address protocols:

- **Transparent** (t-addresses) — with optional derivation info or imported
  private keys for legacy wallets.
- **Sprout** (zc-addresses) — minimal support for legacy zcashd migration.
  Viewing keys (64 bytes) and spending keys (32 bytes) are stored separately
  as opaque canonical encodings.
- **Sapling** (zs-addresses) — with extended full viewing keys and optional
  note commitment witnesses for received outputs.
- **Orchard** — via unified full viewing keys and unified addresses, with
  optional note commitment witnesses.
- **Unified** (u-addresses) — first-class support via ZIP-316 encoded UFVKs
  and unified addresses containing multiple receiver types.
