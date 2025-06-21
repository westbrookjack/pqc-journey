pub struct CaesarCipher {
    key: u8,
}

impl CaesarCipher {
    fn encrypt_char(input: char, shift: u8) -> char {
        let c = input as u8;
        if 65 <= c && c <= 90 {
            ((c - 65 + shift) % 26 + 65) as char
        } else if 97 <= c && c <= 122 {
            ((c - 97 + shift) % 26 + 97) as char
        } else {
            input
        }
    }

    fn decrypt_char(input: char, shift: u8) -> char {
        let c = input as u8;
        if 65 <= c && c <= 90 {
            ((26 + c - 65 - shift) % 26 + 65) as char
        } else if 97 <= c && c <= 122 {
            ((26 + c - 97 - shift) % 26 + 97) as char
        } else {
            input
        }
    }

    pub fn new(key: u8) -> Self {
        Self { key: key % 26 }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut s = String::new();
        for c in plaintext.chars() {
            s.push(Self::encrypt_char(c, self.key));
        }
        s
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut s = String::new();
        for c in ciphertext.chars() {
            s.push(Self::decrypt_char(c, self.key));
        }
        s
    }

    pub fn set_key(&mut self, new_key: u8) {
        self.key = new_key % 26;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        let cipher = CaesarCipher::new(3);
        let plaintext = "Hello, World!";
        let encrypted = cipher.encrypt(plaintext);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_shift_wrapping() {
        let cipher = CaesarCipher::new(25);
        assert_eq!(cipher.encrypt("a"), "z");
        assert_eq!(cipher.encrypt("z"), "y");
    }
}