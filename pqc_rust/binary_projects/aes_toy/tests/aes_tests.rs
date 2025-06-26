use aes_toy::{AesState, key_schedule};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_does_not_panic() {
        let input = [0u8; 16];
        let key = [0x01; 16];
        let round_keys = key_schedule(&key, 3);

        let mut aes = AesState::new(input, key);
        aes.encrypt(3, &round_keys);
        let _ = aes.output();
    }
}
