#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecb_roundtrip_success() {
        let key = [0x0Fu8; 16];
        let plaintext = b"this is a test msg";
        let msg = auth_encrypt(plaintext, &key, None, AesMode::ECB);
        let decrypted = auth_decrypt(msg, &key, AesMode::ECB).expect("decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_cbc_roundtrip_success() {
        let key = [0xABu8; 16];
        let plaintext = b"another plaintext test";
        let msg = auth_encrypt(plaintext, &key, None, AesMode::CBC);
        let decrypted = auth_decrypt(msg, &key, AesMode::CBC).expect("decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_cbc_with_given_iv() {
        let key = [0xCDu8; 16];
        let iv = [0x11u8; 16];
        let plaintext = b"known iv test";
        let msg = auth_encrypt(plaintext, &key, Some(&iv), AesMode::CBC);
        assert_eq!(msg.iv.unwrap(), iv);
        let decrypted = auth_decrypt(msg, &key, AesMode::CBC).expect("decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_mac_verification_fail() {
        let key = [0xFFu8; 16];
        let plaintext = b"verify fail test";
        let mut msg = auth_encrypt(plaintext, &key, None, AesMode::ECB);
        msg.mac_tag[0] ^= 0xFF; // Corrupt the tag

        let result = auth_decrypt(msg, &key, AesMode::ECB);
        assert!(matches!(result, Err(AuthDecryptError::MacVerificationFailed)));
    }

    #[test]
    fn test_missing_iv_for_cbc() {
        let key = [0x12u8; 16];
        let plaintext = b"missing iv test";
        let mut msg = auth_encrypt(plaintext, &key, None, AesMode::CBC);
        msg.iv = None; // Simulate missing IV

        let result = auth_decrypt(msg, &key, AesMode::CBC);
        assert!(matches!(result, Err(AuthDecryptError::MissingIV)));
    }

    #[test]
    fn test_decryption_error_propagation() {
        let key = [0x01u8; 16];
        let plaintext = b"bad decrypt";
        let mut msg = auth_encrypt(plaintext, &key, None, AesMode::ECB);
        msg.ciphertext[0] ^= 0xAA; // corrupt ciphertext

        let result = auth_decrypt(msg, &key, AesMode::ECB);
        assert!(matches!(result, Err(AuthDecryptError::DecryptionFailed(_))));
    }
}
