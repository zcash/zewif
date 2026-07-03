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
}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;
