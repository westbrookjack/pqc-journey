use crate::{AesState, key_schedule, DecryptionError};
use crate::utils::{pad16, unpad16, xor_blocks, generate_random_iv};
use crate::traits::{AesEncryptor, AesDecryptor};

pub struct CBC;

impl AesEncryptor for CBC {
    fn encrypt(&self, plaintext: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> (Vec<u8>, Option<[u8; 16]>) {
        let actual_iv = iv.copied().unwrap_or_else(generate_random_iv);
        let round_keys = key_schedule(key);
        let padded = pad16(plaintext);
        let mut output = Vec::with_capacity(padded.len());
        let mut previous = actual_iv;

        for chunk in padded.chunks_exact(16) {
            let chunk: &[u8; 16] = chunk.try_into().unwrap();
            let xor = xor_blocks(chunk, &previous);
            let mut aes = AesState::new(&xor);
            aes.encrypt(&round_keys);
            let block = aes.output();
            output.extend_from_slice(&block);
            previous = block;
        }

        (output, Some(actual_iv))
    }
}


impl AesDecryptor for CBC {
    fn decrypt(&self, ciphertext: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> Result<Vec<u8>, DecryptionError> {
        let iv = iv.ok_or(DecryptionError::InvalidLength)?;
        let round_keys = key_schedule(key);
        let mut output = Vec::with_capacity(ciphertext.len());
        let mut previous = iv;

        if ciphertext.len() % 16 != 0 {
            return Err(DecryptionError::InvalidLength);
        }

        for chunk in ciphertext.chunks_exact(16) {
            let chunk: &[u8; 16] = chunk.try_into().unwrap();
            let mut aes = AesState::new(chunk);
            aes.decrypt(&round_keys);
            let block = aes.output();
            let xor = xor_blocks(&block, previous);
            output.extend_from_slice(&xor);
            previous = chunk;
        }

        unpad16(&output).ok_or(DecryptionError::InvalidPadding)
    }
}
