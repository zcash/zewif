use std::{array::TryFromSliceError, convert::Infallible};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // Validation errors
    #[error("Amount underflow: {0}")]
    AmountUnderflow(u64),

    #[error("Amount overflow: {0}")]
    AmountOverflow(u64),

    #[error("Invalid language value: {0}")]
    InvalidLanguage(String),

    #[error("Invalid MnemonicLanguage string: {0}")]
    InvalidMnemonicLanguage(String),

    #[error("Hex parsing error: expected {expected} bytes, got {actual}")]
    HexLengthMismatch { expected: usize, actual: usize },

    #[error("Invalid hex string: {0}")]
    InvalidHexString(#[from] hex::FromHexError),

    #[error("Slice conversion error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),

    // Container format errors
    /// The document does not begin with the ZeWIF magic bytes.
    #[error("Not a ZeWIF document: bad magic bytes")]
    BadMagic,

    /// The document ends before the end of the fixed-size container header;
    /// carries the number of bytes present.
    #[error("Truncated ZeWIF container header: {0} bytes")]
    TruncatedHeader(usize),

    /// The document declares a container version that this crate does not
    /// implement; carries the declared version.
    #[error("Unsupported ZeWIF container version: {0}")]
    UnsupportedVersion(u32),

    /// The CBOR payload could not be decoded.
    #[error("CBOR decode error: {0}")]
    CborDecode(#[from] minicbor::decode::Error),

    /// The CBOR payload could not be encoded.
    #[error("CBOR encode error: {0}")]
    CborEncode(#[from] minicbor::encode::Error<Infallible>),
    /// Encryption of a secret store to the requested age recipients failed.
    #[cfg(feature = "encryption")]
    #[error("Secret store encryption failed: {0}")]
    SecretStoreEncryption(#[from] age::EncryptError),

    /// Decryption of an encrypted secret store failed.
    #[cfg(feature = "encryption")]
    #[error("Secret store decryption failed: {0}")]
    SecretStoreDecryption(#[from] age::DecryptError),

    /// The decrypted plaintext was not the CBOR encoding of a secret store.
    #[cfg(feature = "encryption")]
    #[error("Secret store decoding failed: {0}")]
    SecretStoreDecode(#[source] minicbor::decode::Error),
}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;
