use std::io;
use thiserror::Error;
use crate::{ModeSelectionError, AuthError};


/// A unified error type for the AES-MAC CLI.
#[derive(Error, Debug)]
pub enum CliError {

    /// Invalid UTF-8 in decrypted output
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    
    /// I/O error (e.g., reading/writing files)
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Error when decoding a hex string
    #[error("Hex decoding failed: {0}")]
    Hex(#[from] hex::FromHexError),

    /// Bincode serialization or deserialization failed
    #[error("Bincode error: {0}")]
    Bincode(#[from] bincode::Error),

    /// The key provided was not exactly 16 bytes
    #[error("Key must be 16 bytes (32 hex characters)")]
    InvalidKeyLength,

    /// Decryption failed or MAC did not verify
    #[error("Decryption failed or MAC is invalid")]
    Auth(#[from] AuthError),

    /// A requested mode was not implemented
    #[error("{0}")]
    UnsupportedMode(#[from] ModeSelectionError),

}
