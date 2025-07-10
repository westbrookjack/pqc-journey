use crate::{AesState, key_schedule, DecryptionError};
use crate::utils::{pad16, unpad16};
use crate::traits::{AesEncryptor, AesDecryptor};

pub struct ECB;

impl AesEncryptor for ECB {
    fn encrypt(&self, plaintext: &[u8], key: &[u8; 16], _iv: Option<&[u8; 16]>) -> (Vec<u8>, Option<[u8; 16]>) {
        let round_keys = key_schedule(key);
        let padded = pad16(plaintext);
        let mut output = Vec::with_capacity(padded.len());

        for chunk in padded.chunks_exact(16) {
            let chunk: &[u8; 16] = chunk.try_into().unwrap();
            let mut aes = AesState::new(chunk);
            aes.encrypt(&round_keys);
            output.extend_from_slice(&aes.output());
        }

        (output, None)
    }
}


impl AesDecryptor for ECB {
    fn decrypt(&self, ciphertext: &[u8], key: &[u8; 16], _iv: Option<&[u8; 16]>) -> Result<Vec<u8>, DecryptionError> {
        let round_keys = key_schedule(key);
        let mut output = Vec::with_capacity(ciphertext.len());

        if ciphertext.len() % 16 != 0 {
            return Err(DecryptionError::InvalidLength);
        }

        for chunk in ciphertext.chunks_exact(16) {
            let chunk: &[u8; 16] = chunk.try_into().unwrap();
            let mut aes = AesState::new(chunk);
            aes.decrypt(&round_keys);
            output.extend_from_slice(&aes.output());
        }

        unpad16(&output).ok_or(DecryptionError::InvalidPadding)
    }
}
