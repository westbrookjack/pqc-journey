mod caesar_cipher;
use caesar_cipher::CaesarCipher;


fn main() {
    let mut cipher = CaesarCipher::new(3);
    let message = "Attack at dawn!";
    let encrypted = cipher.encrypt(message);
    let decrypted = cipher.decrypt(&encrypted);

    println!("Original: {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);

    cipher.set_key(1);

    let message = "Copy that.";
    let encrypted = cipher.encrypt(message);
    let decrypted = cipher.decrypt(&encrypted);

    println!("Original: {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);

    
}
