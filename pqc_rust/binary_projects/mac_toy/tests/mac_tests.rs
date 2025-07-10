use mac_toy::mac::{mac_sha256, verify_mac};

#[test]
fn test_mac_correctness() {
    let key = b"secretkey1234567";
    let msg = b"hello world!";
    let tag = mac_sha256(key, msg);
    assert!(verify_mac(key, msg, &tag));
}

#[test]
fn test_mac_tamper_detected() {
    let key = b"secretkey1234567";
    let msg = b"hello world!";
    let tampered = b"hello WORLD!";
    let tag = mac_sha256(key, msg);
    assert!(!verify_mac(key, tampered, &tag));
}
