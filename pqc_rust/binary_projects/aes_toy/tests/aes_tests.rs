use aes_toy::{AesState, key_schedule};


// 128-bit AES test key
const AES_KEY: [u8; 16] = [
    0x2b, 0x7e, 0x15, 0x16,
    0x28, 0xae, 0xd2, 0xa6,
    0xab, 0xf7, 0x15, 0x88,
    0x09, 0xcf, 0x4f, 0x3c,
];

// Plaintext block
const PLAINTEXT: [u8; 16] = [
    0x32, 0x43, 0xf6, 0xa8,
    0x88, 0x5a, 0x30, 0x8d,
    0x31, 0x31, 0x98, 0xa2,
    0xe0, 0x37, 0x07, 0x34,
];

// Expected ciphertext (AES-128 ECB, 1 block)
const CIPHERTEXT: [u8; 16] = [
    0x39, 0x25, 0x84, 0x1d,
    0x02, 0xdc, 0x09, 0xfb,
    0xdc, 0x11, 0x85, 0x97,
    0x19, 0x6a, 0x0b, 0x32,
];

#[test]
fn aes_encrypts_correctly() {
    let round_keys = key_schedule(&AES_KEY);
    let mut state = AesState::new(&PLAINTEXT);
    state.encrypt(&round_keys);
    assert_eq!(state.output(), CIPHERTEXT);
}

#[test]
fn aes_decrypts_correctly() {
    let round_keys = key_schedule(&AES_KEY);
    let mut state = AesState::new(&CIPHERTEXT);
    state.decrypt(&round_keys);
    assert_eq!(state.output(), PLAINTEXT);
}

#[test]
fn aes_roundtrip() {
    let round_keys = key_schedule(&AES_KEY);
    let mut state = AesState::new(&PLAINTEXT);
    state.encrypt(&round_keys);
    state.decrypt(&round_keys);
    assert_eq!(state.output(), PLAINTEXT);
}
