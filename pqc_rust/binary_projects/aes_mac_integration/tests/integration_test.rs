use aes_mac_integration::{auth_encrypt, SecureMessage};
use aes_toy::{CBC, ECB};
use mac_toy::PrefixMac;

#[test]
fn test_ecb_mac_encrypt_decrypt_roundtrip() {
    let plaintext = b"Attack at dawn!";
    let key = *b"YELLOW SUBMARINE";
    let mac = PrefixMac::new(&key);
    let ecb = ECB;

    let secure: SecureMessage<_> = auth_encrypt(plaintext, &key, None, mac, &ecb);
    let recovered = secure.decrypt_with(&key, &ecb).expect("Decryption failed");

    assert_eq!(plaintext.to_vec(), recovered);
}

#[test]
fn test_cbc_mac_encrypt_decrypt_roundtrip() {
    let plaintext = b"Attack at dusk!";
    let key = *b"YELLOW SUBMARINE";
    let iv = [0u8; 16];
    let mac = PrefixMac::new(&key);
    let cbc = CBC;

    let secure: SecureMessage<_> = auth_encrypt(plaintext, &key, Some(&iv), mac, &cbc);
    let recovered = secure.decrypt_with(&key, &cbc).expect("Decryption failed");

    assert_eq!(plaintext.to_vec(), recovered);
}

#[test]
fn test_mac_verification_fails_on_modified_ciphertext() {
    let plaintext = b"Meet at the bridge.";
    let key = *b"YELLOW SUBMARINE";
    let iv = [0u8; 16];
    let mac = PrefixMac::new(&key);
    let cbc = CBC;

    let mut secure: SecureMessage<_> = auth_encrypt(plaintext, &key, Some(&iv), mac, &cbc);
    // Tamper with the ciphertext
    secure.ciphertext[0] ^= 0xFF;

    assert!(!secure.mac_is_valid());
    assert!(secure.decrypt_with(&key, &cbc).is_err());
}
