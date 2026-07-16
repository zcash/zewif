//! The ZeWIF container format: standards-based CBOR self-identification
//! around the deterministic CBOR payload.
//!
//! A ZeWIF document is a single CBOR data item:
//!
//! ```text
//! 55799( ZEWIF_TAG( [ version, payload ] ) )
//! ```
//!
//! - Tag 55799 is "Self-Described CBOR" (RFC 8949 §3.4.6). Its three-byte
//!   head `D9 D9 F7` is the registered magic number that lets generic tooling
//!   recognize the byte stream as CBOR without this specification in hand.
//! - [`ZEWIF_TAG`] identifies the specific format, so a decoder that does know
//!   the tag can positively identify a document as ZeWIF.
//! - The tag content is a two-element array: an unsigned container `version`
//!   followed by the payload — the `zewif` map of the CDDL schema. The version
//!   is read before the payload is decoded, so a document whose version this
//!   crate does not implement is rejected without interpreting its payload.

use minicbor::data::Tag;

use crate::{Error, Result, Zewif};

/// RFC 8949 §3.4.6 "Self-Described CBOR". Encodes as the three bytes
/// `D9 D9 F7`, which serve as a magic number identifying the byte stream as
/// CBOR to generic tooling.
pub const SELF_DESCRIBED_CBOR_TAG: u64 = 55799;

/// The CBOR tag identifying a ZeWIF document.
///
/// The value echoes Zcash's SLIP-0044 coin type (133) by repetition. It lies
/// in the First Come First Served range of the IANA "CBOR Tags" registry;
/// registration has been requested (see `docs/cbor-tag-registration.md`).
///
/// PROVISIONAL: this value MUST be treated as unstable until IANA confirms the
/// assignment. If IANA assigns a different number, this constant — and the
/// golden test vectors — change with it.
pub const ZEWIF_TAG: u64 = 133_133;

/// The first version of the ZeWIF container format.
pub const ZEWIF_VERSION_1: u32 = 1;

impl Zewif {
    /// Serializes this document in the ZeWIF container format: the
    /// self-described-CBOR tag and the ZeWIF tag wrapping a `[version,
    /// payload]` array, where the payload is the deterministic CBOR encoding
    /// of this document.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut encoder = minicbor::Encoder::new(&mut bytes);
        encoder
            .encode(Tag::new(SELF_DESCRIBED_CBOR_TAG))?
            .encode(Tag::new(ZEWIF_TAG))?
            .array(2)?
            .u32(ZEWIF_VERSION_1)?
            .encode(self)?;
        Ok(bytes)
    }

    /// Parses a document in the ZeWIF container format.
    ///
    /// The payload is not interpreted unless the enclosing tags match and the
    /// document's declared container version is one this crate implements.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut decoder = minicbor::Decoder::new(bytes);

        // Outer self-described CBOR tag: the document's magic number.
        let self_described = decoder.decode::<Tag>()?;
        if self_described.as_u64() != SELF_DESCRIBED_CBOR_TAG {
            return Err(Error::UnexpectedTag {
                expected: SELF_DESCRIBED_CBOR_TAG,
                found: self_described.as_u64(),
            });
        }

        // Inner ZeWIF tag: positive identification of the format.
        let zewif_tag = decoder.decode::<Tag>()?;
        if zewif_tag.as_u64() != ZEWIF_TAG {
            return Err(Error::UnexpectedTag {
                expected: ZEWIF_TAG,
                found: zewif_tag.as_u64(),
            });
        }

        // The tag content is the definite-length two-element array
        // `[version, payload]`.
        match decoder.array()? {
            Some(2) => {}
            Some(_) => {
                return Err(Error::MalformedContainer(
                    "container array must have exactly two elements",
                ));
            }
            None => {
                return Err(Error::MalformedContainer(
                    "container array must be definite-length",
                ));
            }
        }

        // Read and dispatch on the version before touching the payload, so an
        // unknown version is rejected without interpreting the payload.
        let version = decoder.u32()?;
        match version {
            ZEWIF_VERSION_1 => {
                let zewif = decoder.decode()?;
                let remaining = bytes.len() - decoder.position();
                if remaining > 0 {
                    return Err(Error::TrailingData(remaining));
                }
                Ok(zewif)
            }
            unsupported => Err(Error::UnsupportedVersion(unsupported)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BlockHash, BlockHeight, Error, RandomInstance, Zewif};

    use super::{SELF_DESCRIBED_CBOR_TAG, ZEWIF_TAG};

    /// The fixed 10-byte container prefix preceding the payload:
    /// `D9 D9 F7` (tag 55799), `DA 00 02 08 0D` (tag 133133), `82` (array of
    /// two), `01` (version 1).
    const PREFIX: &[u8] = &[0xD9, 0xD9, 0xF7, 0xDA, 0x00, 0x02, 0x08, 0x0D, 0x82, 0x01];

    fn sample() -> Zewif {
        Zewif::new(
            BlockHeight::from_u32(1_000_000),
            BlockHash::from_bytes([9u8; 32]),
        )
    }

    #[test]
    fn container_roundtrip() {
        for _ in 0..8 {
            let zewif = Zewif::random();
            let bytes = zewif.to_bytes().unwrap();
            // The document begins with the self-described-CBOR magic and the
            // ZeWIF tag, then the `[version, ...]` array.
            assert_eq!(&bytes[..PREFIX.len()], PREFIX);
            assert_eq!(Zewif::from_bytes(&bytes).unwrap(), zewif);
        }
    }

    /// A well-formed CBOR document whose outer tag is not the self-described
    /// CBOR tag is rejected as an unexpected tag.
    #[test]
    fn wrong_outer_tag_is_rejected() {
        let mut bytes = sample().to_bytes().unwrap();
        // Corrupt the outer tag 55799 (`D9 D9 F7`) to a different tag value.
        bytes[2] = 0xF6;
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::UnexpectedTag {
                expected,
                ..
            }) if expected == SELF_DESCRIBED_CBOR_TAG
        ));
    }

    /// A document carrying the self-described CBOR tag but not the ZeWIF tag
    /// is rejected as an unexpected tag.
    #[test]
    fn wrong_inner_tag_is_rejected() {
        let mut bytes = sample().to_bytes().unwrap();
        // Corrupt a byte of the inner ZeWIF tag 133133 (`DA 00 02 08 0D`).
        bytes[7] = 0x0E;
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::UnexpectedTag { expected, .. }) if expected == ZEWIF_TAG
        ));
    }

    /// Input that is not even a CBOR tag is a decode error.
    #[test]
    fn non_cbor_input_is_a_decode_error() {
        assert!(matches!(
            Zewif::from_bytes(b"nope"),
            Err(Error::CborDecode(_))
        ));
    }

    /// A document with an unknown version must be rejected without attempting
    /// to interpret the payload: even a payload that is not well-formed CBOR
    /// reports the version, not a decode error.
    #[test]
    fn unknown_version_is_rejected_without_payload_interpretation() {
        let mut bytes = Vec::from(PREFIX);
        // Overwrite the version byte (last byte of PREFIX) with 2 and follow
        // it with a payload that is not well-formed CBOR.
        *bytes.last_mut().unwrap() = 0x02;
        bytes.extend_from_slice(&[0xff, 0xff, 0xff]);
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::UnsupportedVersion(2))
        ));
    }

    /// Truncation anywhere within the container is a decode error (an
    /// unexpected end of input), not a panic.
    #[test]
    fn truncated_container_is_a_decode_error() {
        let bytes = sample().to_bytes().unwrap();
        for len in 0..PREFIX.len() {
            assert!(matches!(
                Zewif::from_bytes(&bytes[..len]),
                Err(Error::CborDecode(_))
            ));
        }
    }

    /// The payload must be a single CBOR data item: a valid document with
    /// extra bytes appended after the payload is rejected as trailing data,
    /// reporting the number of unconsumed bytes.
    #[test]
    fn trailing_data_after_payload_is_rejected() {
        let mut bytes = sample().to_bytes().unwrap();
        let junk = [0xDE, 0xAD, 0xBE, 0xEF];
        bytes.extend_from_slice(&junk);
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::TrailingData(n)) if n == junk.len()
        ));
    }

    /// A well-framed document whose payload is not a valid `zewif` CBOR item
    /// is a decode error, not a panic.
    #[test]
    fn malformed_payload_is_a_decode_error() {
        let mut bytes = Vec::from(PREFIX);
        bytes.extend_from_slice(&[0xff, 0xff, 0xff]);
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::CborDecode(_))
        ));
        // An empty payload is likewise rejected.
        assert!(matches!(
            Zewif::from_bytes(PREFIX),
            Err(Error::CborDecode(_))
        ));
    }

    /// A container array with the wrong number of elements is rejected as a
    /// malformed container, distinct from a payload decode error.
    #[test]
    fn wrong_container_arity_is_rejected() {
        // The two tags, then `81` (array of one) `01` (version): a
        // one-element container array is rejected before the version or
        // payload is read.
        let bytes = vec![0xD9, 0xD9, 0xF7, 0xDA, 0x00, 0x02, 0x08, 0x0D, 0x81, 0x01];
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::MalformedContainer(_))
        ));
    }
}
