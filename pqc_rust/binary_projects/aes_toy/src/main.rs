use aes_toy::{AesState, key_schedule};

fn main() {
    let key: [u8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15
    ];

    let input: [u8; 16] = [0x00; 16];

    let round_keys = key_schedule(&key, 3);
    let mut aes = AesState::new(input, key);
    aes.encrypt(3, &round_keys);

    println!("Encrypted: {:?}", aes.output());
}
