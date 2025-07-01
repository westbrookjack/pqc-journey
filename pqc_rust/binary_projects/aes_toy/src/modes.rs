use crate::{key_schedule, AesState};



#[derive(Debug)]
pub enum DecryptionError {
    InvalidPadding,
    InvalidLength,
    // Add more variants as needed
}

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

pub fn encrypt_cbc(plaintext: &[u8], key: &[u8; 16], iv: [u8; 16]) -> Vec<u8>{
    let padded = pad16(plaintext);
    let round_keys = key_schedule(key);
    let mut previous = iv;
    let mut output = Vec::with_capacity(padded.len());
    for chunk in padded.chunks_exact(16) {
        let chunk: &[u8; 16] = chunk.try_into().unwrap();  // panics if not length 16
        let xor:[u8;16] = xor_blocks(chunk, &previous);
        let mut aes = AesState::new(&xor);
        aes.encrypt(&round_keys);
        let block = aes.output();
        output.extend_from_slice(&block);
        previous = block;
    }

    output

}


pub fn decrypt_cbc(ciphertext: &[u8], key: &[u8; 16], iv: [u8; 16]) -> Result<Vec<u8>, DecryptionError> {
    let round_keys = key_schedule(key);
    let mut previous = &iv;
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

pub fn pad16(text: &[u8])-> Vec<u8> {
    let k = 16 - (text.len() % 16);
    let mut out:Vec<u8> = text.to_vec();
    let padding = vec![k as u8; k];
    out.extend_from_slice(&padding);
    out
}

pub fn unpad16(text: &[u8])->Option<Vec<u8>> {
    if let Some(&k) = text.last() {
        if (1..=16).contains(&k) && text.len()>=k as usize {
                let slice = &text[text.len()-k as usize..];
                if slice.iter().all(|&x| x== k) {
                    let unpad = &text[..text.len()-k as usize];
                    return Some(unpad.to_vec());
                }
                
        }
    }
    None
}

pub fn xor_blocks(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out:[u8;16] = [0;16];
    for i in 0..16 {
        out[i] = a[i]^ b[i];
    }
    out
}

