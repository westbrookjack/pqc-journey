// tests/cipher.rs

use pqc_rust::ciphers::caesar::CaesarCipher;


#[test]
    fn test_encrypt_and_decrypt() {
        let cipher = CaesarCipher::new_rand();
        let plaintext = "Hello, World!";
        let encrypted = cipher.encrypt(plaintext);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, plaintext);
    }

#[test]
    fn test_shift_wrapping() {
        let cipher = CaesarCipher::new_set(25);
        assert_eq!(cipher.encrypt("a"), "z");
        assert_eq!(cipher.encrypt("z"), "y");
    }
