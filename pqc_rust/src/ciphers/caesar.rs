pub struct CaesarCipher {
    key: u8,
}

impl CaesarCipher {
    fn encrypt_char(input: char, shift: u8) -> char {
        let c = input as u8;
        if b'A' <= c && c <= b'Z' {
            ((c - b'A' + shift) % 26 + b'A') as char
        } else if b'a' <= c && c <= b'z' {
            ((c - b'a' + shift) % 26 + b'a') as char
        } else {
            input
        }
    }

    fn decrypt_char(input: char, shift: u8) -> char {
        let c = input as u8;
        if b'A' <= c && c <= b'Z' {
            ((26 + c - b'A' - shift) % 26 + b'A') as char
        } else if b'a' <= c && c <= b'z' {
            ((26 + c - b'a' - shift) % 26 + b'a') as char
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