use aes_toy::{AesState, key_schedule};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_encrypt_decrypt_cycle() {
    let plaintext: [u8; 16] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
                               0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key: [u8; 16] = [0x0f; 16]; // constant key

    let round_keys = key_schedule(&key, 10);

    let mut state = AesState::new(plaintext);
    state.encrypt(10, &round_keys);
    let ciphertext = state.output();

    let mut state2 = AesState::new(ciphertext);
    state2.decrypt(10, &round_keys);
    let decrypted = state2.output();

    assert_eq!(decrypted, plaintext, "Decryption failed: {:?} != {:?}", decrypted, plaintext);
}

    #[test]
fn test_encrypt_decrypt_case_2() {
    let plaintext: [u8; 16] = [
        0xde, 0xad, 0xbe, 0xef,
        0xba, 0xad, 0xf0, 0x0d,
        0xca, 0xfe, 0xba, 0xbe,
        0x00, 0x11, 0x22, 0x33,
    ];
    let key: [u8; 16] = [
        0x0f, 0x15, 0x71, 0xc9,
        0x47, 0xd9, 0xe8, 0x59,
        0x0c, 0xb7, 0xad, 0xd6,
        0xaf, 0x7f, 0x67, 0x98,
    ];

    let round_keys = crate::key_schedule(&key, 10);
    let mut state = crate::AesState::new(plaintext);
    state.encrypt(10, &round_keys);
    let ciphertext = state.output();

    let mut decrypt_state = crate::AesState::new(ciphertext);
    decrypt_state.decrypt(10, &round_keys);
    let decrypted = decrypt_state.output();

    assert_eq!(decrypted, plaintext, "Decryption did not yield original plaintext");
}

}
