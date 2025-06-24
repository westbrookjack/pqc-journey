pub struct VigenereCipher<'a> {
    key: &'a str,
}

impl<'a> VigenereCipher<'a> {
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

    fn shift_value(&self, i: usize) -> u8 {
        let c = self.key.chars().nth(i % self.key.len())
            .expect("Key must not be empty and must be long enough");
        c as u8 - b'a'
    }
    //constructs new instance of the cypher with each characters in the key having to be ascii lowercase only
    pub fn new(key: &'a str) -> Self {
        assert!(key.chars().all(|c| c.is_ascii_lowercase()), "Key must be lowercase letters only");
        Self { key }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut s = String::new();
        let mut i = 0;
        for c in plaintext.chars() {
            s.push(Self::encrypt_char(c, self.shift_value(i)));
            if c.is_ascii_alphabetic() {
                i += 1;
            }
        }
        s
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut s = String::new();
        let mut i = 0;
        for c in ciphertext.chars() {
            s.push(Self::decrypt_char(c, self.shift_value(i)));
            if c.is_ascii_alphabetic() {
                i += 1;
            }
        }
        s
    }

    pub fn key(&self) -> &str {
        self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_basic() {
        let cipher = VigenereCipher::new("lemon");
        let plaintext = "attackatdawn";
        let expected = "lxfopvefrnhr";
        let result = cipher.encrypt(plaintext);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decrypt_basic() {
        let cipher = VigenereCipher::new("lemon");
        let ciphertext = "lxfopvefrnhr";
        let expected = "attackatdawn";
        let result = cipher.decrypt(ciphertext);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_encrypt_with_spaces_and_punctuation() {
        let cipher = VigenereCipher::new("key");
        let plaintext = "Hello, World!";
        let result = cipher.encrypt(plaintext);
        // Just make sure it preserves non-alphabetic characters
        assert_eq!(result.len(), plaintext.len());
        assert!(result.chars().any(|c| !c.is_ascii_alphabetic())); // punctuation preserved
    }

    #[test]
    #[should_panic(expected = "Key must be lowercase letters only")]
    fn test_invalid_key_panics() {
        VigenereCipher::new("ABC123");
    }

    #[test]
    fn test_shift_value_logic() {
        let cipher = VigenereCipher::new("abc");
        // 'a' => shift 0, 'b' => 1, 'c' => 2
        assert_eq!(cipher.shift_value(0), 0);
        assert_eq!(cipher.shift_value(1), 1);
        assert_eq!(cipher.shift_value(2), 2);
        assert_eq!(cipher.shift_value(3), 0); // wraps
    }
}
