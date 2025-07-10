use crate::DecryptionError;

pub trait AesMode {
    fn encrypt(&self, plaintext: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> (Vec<u8>, Option<[u8; 16]>);
    fn decrypt(&self, ciphertext: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> Result<Vec<u8>, DecryptionError>;
}
