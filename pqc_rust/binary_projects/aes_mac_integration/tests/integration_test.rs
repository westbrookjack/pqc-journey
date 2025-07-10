use aes_mac_integration::{auth_encrypt, auth_decrypt};
use aes_toy::{ECB, CBC};
use mac_toy::PrefixMac;

#[test]
fn test_ecb_roundtrip() {
    let key = [0u8; 16];
    let plaintext = b"Example message in ECB mode.";
    let mac = PrefixMac::new(&key);
    let ecb = ECB;

    let secure = auth_encrypt(plaintext, &key, None, mac.clone(), &ecb);
    let decrypted = auth_decrypt(&secure, &key, &mac, &ecb).expect("ECB decryption failed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_cbc_roundtrip() {
    let key = [1u8; 16];
    let iv = [42u8; 16];
    let plaintext = b"Example message in CBC mode.";
    let mac = PrefixMac::new(&key);
    let cbc = CBC;

    let secure = auth_encrypt(plaintext, &key, Some(&iv), mac.clone(), &cbc);
    let decrypted = auth_decrypt(&secure, &key, &mac, &cbc).expect("CBC decryption failed");

    assert_eq!(decrypted, plaintext);
}
