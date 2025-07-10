use aes_toy::traits::{AesEncryptor, AesDecryptor};
use mac_toy::traits::MacModeImpl;
use std::convert::TryInto;

pub struct SecureMessage<M> {
    pub ciphertext: Vec<u8>,
    pub mac_tag: Vec<u8>,
    pub iv: Option<[u8;16]>,
    pub mac_mode: M,
}

impl<M: MacModeImpl> SecureMessage<M> {
    pub fn mac_is_valid(&self) -> bool {
    if let Ok(tag_array) = self.mac_tag.as_slice().try_into() {
        self.mac_mode.verify(&self.ciphertext, tag_array)
    } else {
        false
    }
}


    pub fn decrypt_with<D: AesDecryptor>(
        &self,
        key: &[u8; 16],
        decryptor: &D,
    ) -> Result<Vec<u8>, AuthError> {
        if !self.mac_is_valid() {
            return Err(AuthError::MacVerificationFailed);
        }

        decryptor
            .decrypt(&self.ciphertext, key, self.iv.as_ref())
            .map_err(AuthError::DecryptionFailed)
    }

    
}

impl<M: MacModeImpl> std::fmt::Debug for SecureMessage<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SecureMessage {{ ciphertext: {:02x?}, mac_tag: {:02x?}, iv: {:?} }}",
            self.ciphertext,
            self.mac_tag,
            self.iv
        )
    }
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

pub fn encrypt_authenticated<M, E>(
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

