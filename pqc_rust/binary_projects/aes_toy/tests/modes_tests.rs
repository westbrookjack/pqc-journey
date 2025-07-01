use aes_toy::{encrypt_cbc, decrypt_cbc, encrypt_ecb, decrypt_ecb};
use aes_toy::modes::DecryptionError;

const KEY: [u8; 16] = [0x00; 16];  // simple test key
const IV: [u8; 16]  = [0x00; 16];  // simple test IV

#[test]
fn test_cbc_encrypt_decrypt_roundtrip() {
    let plaintext = b"Cryptography is awesome!";
    let ciphertext = encrypt_cbc(plaintext, &KEY, IV);
    let decrypted = decrypt_cbc(&ciphertext, &KEY, IV).expect("Decryption failed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_cbc_decrypt_invalid_padding() {
    // create valid ciphertext then tamper with padding
    let plaintext = b"Valid padding test!";
    let mut ciphertext = encrypt_cbc(plaintext, &KEY, IV);

    // Corrupt the last byte of padding
    if let Some(last) = ciphertext.last_mut() {
        *last ^= 0x01;
    }

    let result = decrypt_cbc(&ciphertext, &KEY, IV);
    assert!(matches!(result, Err(DecryptionError::InvalidPadding)));
}

#[test]
fn test_cbc_decrypt_invalid_length() {
    let ciphertext = vec![0u8; 31]; // not a multiple of 16
    let result = decrypt_cbc(&ciphertext, &KEY, IV);
    assert!(matches!(result, Err(DecryptionError::InvalidLength)));
}

#[test]
fn test_encrypt_output_differs_from_plaintext() {
    let plaintext = b"Confidential data!";
    let ciphertext = encrypt_cbc(plaintext, &KEY, IV);
    assert_ne!(&ciphertext[..plaintext.len()], plaintext); // encrypted form should differ
}

#[test]
fn test_different_iv_yields_different_ciphertext() {
    let plaintext = b"Same plaintext test!";
    let iv2 = [0xFF; 16];

    let ct1 = encrypt_cbc(plaintext, &KEY, IV);
    let ct2 = encrypt_cbc(plaintext, &KEY, iv2);

    assert_ne!(ct1, ct2); // IV should change output
}

#[test]
fn test_ecb_encrypt_decrypt_roundtrip() {
    let key = [0u8; 16];
    let plaintext = b"The quick brown fox jumps";

    let ciphertext = encrypt_ecb(plaintext, &key);
    let decrypted = decrypt_ecb(&ciphertext, &key).expect("Decryption failed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_ecb_decrypt_invalid_padding() {
    let key = [0u8; 16];
    let plaintext = b"Hello ECB!";
    let mut ciphertext = encrypt_ecb(plaintext, &key);

    // Corrupt the last padding byte
    if let Some(last) = ciphertext.last_mut() {
        *last ^= 0x01;
    }

    let result = decrypt_ecb(&ciphertext, &key);
    assert!(matches!(result, Err(DecryptionError::InvalidPadding)));
}

#[test]
fn test_ecb_decrypt_invalid_length() {
    let key = [0u8; 16];
    let ciphertext = vec![0u8; 30]; // not block-aligned

    let result = decrypt_ecb(&ciphertext, &key);
    assert!(matches!(result, Err(DecryptionError::InvalidLength)));
}

#[test]
fn test_ecb_output_differs_from_plaintext() {
    let key = [0u8; 16];
    let plaintext = b"ECB will change me";

    let ciphertext = encrypt_ecb(plaintext, &key);
    assert_ne!(&ciphertext[..plaintext.len()], plaintext);
}

#[test]
fn test_ecb_deterministic_output() {
    let key = [0u8; 16];
    let plaintext = b"Repeat plaintext block";
    
    let ct1 = encrypt_ecb(plaintext, &key);
    let ct2 = encrypt_ecb(plaintext, &key);

    assert_eq!(ct1, ct2); // ECB should produce same output for same input
}
