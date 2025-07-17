use crate::{AesState, key_schedule, DecryptionError};
use crate::utils::{pad16, unpad16};
use crate::traits::CipherMode;

#[derive(Clone)]
pub struct Ecb;

impl CipherMode for Ecb {
    fn encrypt(&self, key: &[u8; 16],  _iv: Option<&[u8; 16]>, plaintext: &[u8]) -> (Vec<u8>, Option<[u8; 16]>) {
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

    fn decrypt(&self, key: &[u8; 16], _iv: Option<&[u8; 16]>,ciphertext: &[u8]) -> Result<Vec<u8>, DecryptionError> {
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

    fn name (&self) -> &'static str {
        "ecb"
    }
}

impl Default for Ecb {
    fn default()->Self {
        Ecb
    }
}