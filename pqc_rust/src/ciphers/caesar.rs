use rand::Rng;

//This struct will have a private key of type u8 as it's only value. One can construct an instance with_key by setting
// the key to a u8 value, or with the default key 3 by the without_key function.
pub struct CaesarCipher {
    key: u8,
}


// takes the key, shifts letters down so a, A = 0, b, B =1, ... , z,Z = 25, then adds the key modulo 26 and shifts back up. Leaves other characters unchanged.
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

    // takes the key, shifts letters down so a, A = 0, b, B =1, ... , z,Z = 25, then subtracts the key modulo 26 and shifts back up. Leaves other characters unchanged.

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

    //constructs new insance of CeasarCipher with key value set to the input, which must be of type u8
    pub fn new_set(key: u8) -> Self {
        Self { key: key % 26 }
    }
    //constructs new insance of CeasarCipher with randomly set key value
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(0..=25);
        Self { key: n }
    }

    // uses the private function encrypt_char to encrypt each character of the plaintext, and returns the cyphertext
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

}

// #[cfg(test)]
// mod tests {
//     use super::*;

    
// }