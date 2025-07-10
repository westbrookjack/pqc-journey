pub mod aes_state;
pub mod key_schedule;
pub mod traits;
pub mod cbc;
pub mod ecb;
pub mod utils;

pub use aes_state::AesState;
pub use key_schedule::key_schedule;

pub use traits::{AesEncryptor, AesDecryptor};
pub use utils::generate_random_iv;
pub use cbc::CBC;
pub use ecb::ECB;

#[derive(Debug)]
pub enum DecryptionError {
    InvalidPadding,
    MissingIV,
    InvalidLength,
}
