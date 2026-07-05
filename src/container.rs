//! The ZeWIF container format: magic bytes and format version framing
//! around the deterministic CBOR payload.

use crate::{Error, Result, Zewif};

/// The magic bytes identifying a ZeWIF document: the ASCII string `ZEWIF`.
pub const MAGIC_BYTES: &[u8] = b"ZEWIF";

/// The first version of the ZeWIF container format.
pub const ZEWIF_VERSION_1: u32 = 1;

/// The length of the container header: the magic bytes followed by the
/// unsigned 32-bit little-endian format version.
const HEADER_LEN: usize = MAGIC_BYTES.len() + 4;

impl Zewif {
    /// Serializes this document in the ZeWIF container format: the magic
    /// bytes and format version followed by the deterministic CBOR payload.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC_BYTES);
        bytes.extend_from_slice(&ZEWIF_VERSION_1.to_le_bytes());
        minicbor::encode(self, &mut bytes)?;
        Ok(bytes)
    }

    /// Parses a document in the ZeWIF container format.
    ///
    /// The payload is not interpreted unless the magic bytes match and the
    /// document's declared container version is one this crate implements.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let matched = bytes.len().min(MAGIC_BYTES.len());
        if bytes[..matched] != MAGIC_BYTES[..matched] {
            return Err(Error::BadMagic);
        }
        if bytes.len() < HEADER_LEN {
            return Err(Error::TruncatedHeader(bytes.len()));
        }
        let version = u32::from_le_bytes(bytes[MAGIC_BYTES.len()..HEADER_LEN].try_into()?);
        match version {
            ZEWIF_VERSION_1 => {
                // The payload must be a single CBOR data item: decode one
                // item and reject any bytes left over after it.
                let payload = &bytes[HEADER_LEN..];
                let mut decoder = minicbor::Decoder::new(payload);
                let zewif = decoder.decode()?;
                let remaining = payload.len() - decoder.position();
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

    use super::{HEADER_LEN, MAGIC_BYTES, ZEWIF_VERSION_1};

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
            assert_eq!(&bytes[..MAGIC_BYTES.len()], MAGIC_BYTES);
            assert_eq!(
                bytes[MAGIC_BYTES.len()..HEADER_LEN],
                ZEWIF_VERSION_1.to_le_bytes()
            );
            assert_eq!(Zewif::from_bytes(&bytes).unwrap(), zewif);
        }
    }

    #[test]
    fn wrong_magic_is_rejected() {
        let mut bytes = sample().to_bytes().unwrap();
        bytes[0] = b'X';
        assert!(matches!(Zewif::from_bytes(&bytes), Err(Error::BadMagic)));
        assert!(matches!(Zewif::from_bytes(b"nope"), Err(Error::BadMagic)));
    }

    /// A document with an unknown version must be rejected without
    /// attempting to interpret the payload: even a payload that is not
    /// well-formed CBOR reports the version, not a decode error.
    #[test]
    fn unknown_version_is_rejected_without_payload_interpretation() {
        let mut bytes = Vec::from(MAGIC_BYTES);
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&[0xff, 0xff, 0xff]);
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::UnsupportedVersion(2))
        ));
    }

    /// An input that matches the magic as far as it goes but ends before
    /// the end of the version field is reported as a truncated header; an
    /// empty input is the degenerate case.
    #[test]
    fn truncated_header_is_rejected() {
        let bytes = sample().to_bytes().unwrap();
        for len in 0..HEADER_LEN {
            assert!(matches!(
                Zewif::from_bytes(&bytes[..len]),
                Err(Error::TruncatedHeader(n)) if n == len
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

    /// A well-framed document whose payload is not a valid `zewif` CBOR
    /// item is a decode error, not a panic.
    #[test]
    fn malformed_payload_is_a_decode_error() {
        let mut bytes = Vec::from(MAGIC_BYTES);
        bytes.extend_from_slice(&ZEWIF_VERSION_1.to_le_bytes());
        bytes.extend_from_slice(&[0xff, 0xff, 0xff]);
        assert!(matches!(
            Zewif::from_bytes(&bytes),
            Err(Error::CborDecode(_))
        ));
        // An empty payload is likewise rejected.
        assert!(matches!(
            Zewif::from_bytes(&bytes[..HEADER_LEN]),
            Err(Error::CborDecode(_))
        ));
    }
}
