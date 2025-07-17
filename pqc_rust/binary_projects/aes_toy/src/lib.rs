pub mod aes_state;
pub mod key_schedule;
pub mod traits;
pub mod cbc;
pub mod ecb;
pub mod utils;

pub use aes_state::AesState;
pub use cbc::Cbc;
pub use ecb::Ecb;
pub use key_schedule::key_schedule;
use std::fmt;
pub use traits::{CipherMode, CipherModeImpl};
pub use utils::generate_random_iv;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecryptionError {
    InvalidPadding,
    MissingIV,
    InvalidLength,
}

impl fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPadding => write!(f, "Invalid padding"),
            Self::MissingIV => write!(f, "Missing IV"),
            Self::InvalidLength => write!(f, "Invalid length"),
        }
    }
}

impl std::error::Error for DecryptionError {}
