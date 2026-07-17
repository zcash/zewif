# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). 

## [Unreleased]

### Changed
- The container framing is now standards-based CBOR self-identification
  instead of an ASCII magic and a little-endian version header. A ZeWIF
  document is a single CBOR data item: tag 55799 ("Self-Described CBOR",
  RFC 8949 §3.4.6, whose `D9 D9 F7` head is a magic number marking the byte
  stream as CBOR) enclosing the ZeWIF tag enclosing a `[version, payload]`
  array. The whole document is now valid, sniffable CBOR. The container
  version remains 1 while the format is in release-candidate status;
  documents produced by 1.0.0-rc.2 do not decode with this revision.
- `Zewif::to_bytes` / `from_bytes` emit and parse the new framing. The
  `Error::BadMagic` and `Error::TruncatedHeader` variants are replaced by
  `Error::UnexpectedTag { expected, found }` and
  `Error::MalformedContainer(_)`; the `MAGIC_BYTES` constant is removed and
  `SELF_DESCRIBED_CBOR_TAG` and `ZEWIF_TAG` are added.

### Added
- The ZeWIF CBOR tag (provisional value 133133, requested in the IANA "CBOR
  Tags" First Come First Served range; see `docs/cbor-tag-registration.md`).

## [1.0.0-rc.2] 2026-07-11

### Changed
- MSRV is now 1.88, and the CBOR codec dependency is now minicbor 2.
- The wire encoding of tagged unions is flattened from `[variant-id, [body?]]`
  to `[variant-id, body?]`: payload-free variants encode as the one-element
  array `[variant-id]`, and data-bearing variants carry their single body
  item directly. The container version remains 1 while the format is in
  release-candidate status; documents produced by 1.0.0-rc.1 do not decode
  with this revision.

## [1.0.0-rc.1] 2026-07-09

Initial release candidate for the Zcash Wallet Interchange Format. The 
serialized format defined herein should not be considered stable until the
final 1.0 release has been published.


