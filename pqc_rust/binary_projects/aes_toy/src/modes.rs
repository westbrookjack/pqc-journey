use crate::{AesState, key_schedule};
use crate::utils::{pad16, unpad16, xor_blocks, DecryptionError};


pub fn encrypt_ecb(plaintext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let round_keys = key_schedule(key);
    let padded = pad16(plaintext);
    let mut output = Vec::with_capacity(padded.len());


    for chunk in padded.chunks_exact(16) {
        let chunk: &[u8; 16] = chunk.try_into().unwrap();  // panics if not length 16
        let mut aes = AesState::new(chunk);
        aes.encrypt(&round_keys);

        output.extend_from_slice(&aes.output());
    }

    output

}


pub fn decrypt_ecb(ciphertext: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, DecryptionError> {
    let round_keys = key_schedule(key);
    let mut output = Vec::with_capacity(ciphertext.len());


    if ciphertext.len() % 16 != 0 {
        return Err(DecryptionError::InvalidLength);
    }


    for chunk in ciphertext.chunks_exact(16) {
        let chunk: &[u8; 16] = chunk.try_into().unwrap();  // panics if not length 16
        let mut aes = AesState::new(chunk);
        aes.decrypt(&round_keys);
        output.extend_from_slice(&aes.output());
    }

    unpad16(&output).ok_or(DecryptionError::InvalidPadding)

}

pub fn encrypt_cbc(plaintext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let padded = pad16(plaintext);
    let round_keys = key_schedule(key);
    let mut previous = *iv; // Copies the IV — okay since it's 16 bytes
    let mut output = Vec::with_capacity(padded.len());

    for chunk in padded.chunks_exact(16) {
        let chunk: &[u8; 16] = chunk.try_into().unwrap();  // Panics if not length 16
        let xor: [u8; 16] = xor_blocks(chunk, &previous);  // XOR with previous block
        let mut aes = AesState::new(&xor);                // Create AES state
        aes.encrypt(&round_keys);                         // Encrypt the block
        let block = aes.output();                         // Get output block
        output.extend_from_slice(&block);                 // Append to output
        previous = block;                                 // Update previous
    }

    output
}



pub fn decrypt_cbc(ciphertext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>, DecryptionError> {
    let round_keys = key_schedule(key);
    let mut previous = iv;
    let mut output = Vec::with_capacity(ciphertext.len());


    if ciphertext.len() % 16 != 0 {
        return Err(DecryptionError::InvalidLength);
    }

    for chunk in ciphertext.chunks_exact(16) {
        let chunk: &[u8;16] = chunk.try_into().unwrap();
        
        let mut aes = AesState::new(chunk);

        aes.decrypt(&round_keys);

        let block = aes.output();
        let xor = xor_blocks(&block, previous);
        output.extend_from_slice(&xor);

        previous = chunk;
    }
    unpad16(&output).ok_or(DecryptionError::InvalidPadding)


}

