use aes_toy::traits::AesMode;
use mac_toy::traits::MacModeImpl;
use std::convert::TryInto;

pub struct SecureMessage<M> {
    pub ciphertext: Vec<u8>,
    pub mac_tag: Vec<u8>,
    pub iv: Option<[u8; 16]>,
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

    pub fn decrypt_with<C: AesMode>(
        &self,
        key: &[u8; 16],
        cipher: &C,
    ) -> Result<Vec<u8>, AuthError> {
        if !self.mac_is_valid() {
            return Err(AuthError::MacVerificationFailed);
        }

        cipher
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
pub fn auth_encrypt<M, C>(
    plaintext: &[u8],
    key: &[u8; 16],
    iv: Option<&[u8; 16]>,
    mac: M,
    cipher: &C,
) -> SecureMessage<M>
where
    M: MacModeImpl + Clone,
    C: AesMode,
{
    let (ciphertext, actual_iv) = cipher.encrypt(plaintext, key, iv);
    let tag = mac.compute(&ciphertext);

    SecureMessage {
        ciphertext,
        mac_tag: tag.to_vec(),
        iv: actual_iv,
        mac_mode: mac,
    }
}
