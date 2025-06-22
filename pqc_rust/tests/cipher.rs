// tests/cipher.rs

use pqc_rust::ciphers::caesar::CaesarCipher;


#[test]
fn encrypt_decrypt_roundtrip() {
    let cipher = CaesarCipher::new(3);
    let plaintext = "hello world";
    let ciphertext = cipher.encrypt(plaintext);
    let decrypted = cipher.decrypt(&ciphertext);
    assert_eq!(plaintext, decrypted);
}
