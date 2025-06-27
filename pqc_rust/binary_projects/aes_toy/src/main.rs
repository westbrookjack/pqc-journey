use aes_toy::{AesState, key_schedule};

fn main() {
    let plaintext: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8,
        0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2,
        0xe0, 0x37, 0x07, 0x34
    ];
    let key = [0x2b; 16]; // Simple key
    
    let round_keys = key_schedule(&key, 4);

    let mut aes = AesState::new(plaintext);
    aes.encrypt(4, &round_keys);
    println!("Ciphertext: {:02x?}", aes.output());

    aes.decrypt(4, &round_keys);
    println!("Decrypted: {:02x?}", aes.output());

    assert_eq!(aes.output(), plaintext);
}
