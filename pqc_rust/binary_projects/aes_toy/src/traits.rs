use crate::{DecryptionError, Ecb, Cbc};

pub trait CipherMode {
    fn encrypt(&self, key: &[u8; 16], iv: Option<&[u8; 16]>, plaintext: &[u8]) -> (Vec<u8>, Option<[u8; 16]>);
    fn decrypt(&self, key: &[u8; 16], iv: Option<&[u8; 16]>, ciphertext: &[u8]) -> Result<Vec<u8>, DecryptionError>;
    fn name(&self) -> &'static str;
}

#[derive(Clone)]
pub enum CipherModeImpl {
    Ecb(Ecb),
    Cbc(Cbc),
}

impl CipherMode for CipherModeImpl {
    fn encrypt(&self, key: &[u8; 16], iv: Option<&[u8; 16]>, plaintext: &[u8]) -> (Vec<u8>, Option<[u8; 16]>) {
        match self {
            CipherModeImpl::Ecb(mode) => mode.encrypt(key, iv, plaintext),
            CipherModeImpl::Cbc(mode) => mode.encrypt(key, iv, plaintext),
        }
    }

    fn decrypt(&self, key: &[u8; 16], iv: Option<&[u8; 16]>, ciphertext: &[u8]) -> Result<Vec<u8>, DecryptionError> {
        match self {
            CipherModeImpl::Ecb(mode) => mode.decrypt(key, iv, ciphertext),
            CipherModeImpl::Cbc(mode) => mode.decrypt(key, iv, ciphertext),
        }
    }


    fn name(&self) -> &'static str {
        match self {
            CipherModeImpl::Ecb(mode) => mode.name(),
            CipherModeImpl::Cbc(mode) => mode.name(),
        }
    }
}

impl Default for CipherModeImpl {
    fn default()-> Self {
        CipherModeImpl::Cbc(Cbc::default())
    }
}