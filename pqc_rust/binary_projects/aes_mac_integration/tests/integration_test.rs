use aes_mac_integration::{auth_encrypt, SecureMessage};
use aes_toy::{ECB, CBC};
use mac_toy::PrefixMac;

#[test]
fn test_ecb_prefix_mac() {
    let key = [0x01; 16];
    let plaintext = b"Secret message!";

    let mac = PrefixMac::new(&key);
    let ecb = ECB;

    let secure: SecureMessage<_> = auth_encrypt(plaintext, &key, None, mac, &ecb);
    assert!(secure.mac_is_valid());

    let decrypted = secure.decrypt_with(&key, &ecb).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_cbc_prefix_mac() {
    let key = [0x02; 16];
    let iv = [0xAA; 16];
    let plaintext = b"Another message!";

    let mac = PrefixMac::new(&key);
    let cbc = CBC;

    let secure: SecureMessage<_> = auth_encrypt(plaintext, &key, Some(&iv), mac, &cbc);
    assert!(secure.mac_is_valid());

    let decrypted = secure.decrypt_with(&key, &cbc).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}
