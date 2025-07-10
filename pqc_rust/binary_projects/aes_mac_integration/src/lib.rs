use aes_toy::traits::{AesEncryptor, AesDecryptor};
use mac_toy::traits::MacModeImpl;
use std::convert::TryInto;

pub struct SecureMessage<M> {
    pub ciphertext: Vec<u8>,
    pub mac_tag: Vec<u8>,
    pub iv: Option<[u8;16]>,
    pub mac_mode: M,
}

#[derive(Debug)]
pub enum AuthError {
    MacVerificationFailed,
    DecryptionFailed(aes_toy::DecryptionError),
    MissingIV,
}

/// Encrypts a message using the provided AES mode and computes a MAC.
/// 
/// - `plaintext`: the data to encrypt
/// - `key`: the 16-byte encryption key
/// - `iv`: optional initialization vector (for CBC mode)
/// - `mac`: an instance of a MAC algorithm
/// - `encryptor`: an AES encryptor (e.g., ECB or CBC)
///
/// Returns a `SecureMessage` containing the ciphertext, MAC tag, IV, and MAC.

pub fn auth_encrypt<M, E>(
    plaintext: &[u8],
    key: &[u8; 16],
    iv: Option<&[u8; 16]>,
    mac: M,
    encryptor: &E
) -> SecureMessage<M>
where
    M: MacModeImpl + Clone,
    E: AesEncryptor,
{
    let (ciphertext, actual_iv) = encryptor.encrypt(plaintext, key, iv);
    let tag = mac.compute(&ciphertext);

    SecureMessage {
        ciphertext,
        mac_tag: tag.to_vec(),
        iv: actual_iv,
        mac_mode: mac,
    }
}


pub fn auth_decrypt<M, D>(
    message: &SecureMessage<M>,
    key: &[u8; 16],
    mac: &M,
    decryptor: &D,
) -> Result<Vec<u8>, AuthError>
where
    M: MacModeImpl,
    D: AesDecryptor,
{
    let tag_array: &[u8; 16] = message.mac_tag.as_slice()
        .try_into()
        .map_err(|_| AuthError::MacVerificationFailed)?;

    if !mac.verify(&message.ciphertext, tag_array) {
        return Err(AuthError::MacVerificationFailed);
    }

    decryptor
        .decrypt(&message.ciphertext, key, message.iv.as_ref())
        .map_err(AuthError::DecryptionFailed)
}
