use aes_toy::modes::{encrypt_cbc, decrypt_cbc, DecryptionError};
use rand::rngs::OsRng;
use rand::RngCore;




fn main() {
    let mut iv = [0u8; 16];
    let key = [0u8; 16];
    OsRng.fill_bytes(&mut iv);
    //OsRng.fill_bytes(&mut key);
    println!("Key (hex) is: {}", hex::encode(key));
    println!("IV (hex) is: {}", hex::encode(iv));
    let message = b"Attack at dawn! Confidential--do not share.";

    let ciphertext = encrypt_cbc(message, &key, iv);
    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

    match decrypt_cbc(&ciphertext, &key, iv) {
        Ok(decrypted) => {
            println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
        }
        Err(DecryptionError::InvalidPadding) => {
            println!("Invalid padding during decryption!");
        }
        Err(DecryptionError::InvalidLength) => {
            println!("Ciphertext had invalid length!");
        }
    }
}
