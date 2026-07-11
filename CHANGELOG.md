# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). 

## [Unreleased]

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


