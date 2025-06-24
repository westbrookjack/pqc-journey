use pqc_rust::number_theory::{ mod_exp::modular_exponentiation, extended_gcd::gcd_triple };
use pqc_rust::ciphers::vigenere::{ VigenereCipher};

fn main() {
    let base = 2;
    let exp = 4;
    let modulus = 256;

    let result = modular_exponentiation(base, exp, modulus);
    println!("Result: {}", result);


    println!("{:?}",gcd_triple(3,0));

    let key: &str = "hat";

    let cipher = VigenereCipher::new(key);

    let plaintext = "The key is \"hat\".";

    println!("{} after encryption is: {}\n The resulting ciphertext after decryption is : {}", plaintext, cipher.encrypt(plaintext), cipher.decrypt(&cipher.encrypt(plaintext)));

}
