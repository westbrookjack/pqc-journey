use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

use aes_toy::{generate_random_iv, ECB, CBC, AesEncryptor, AesDecryptor};

#[derive(Parser)]
#[command(name = "AES Toy")]
#[command(about = "Encrypt or decrypt files using toy AES", long_about = None)]
struct Cli {
    /// Input file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// AES key (hex, 32 characters = 16 bytes)
    #[arg(short, long)]
    key: String,

    /// Mode: encrypt or decrypt
    #[arg(short, long)]
    mode: Mode,

    /// Cipher mode: ECB or CBC
    #[arg(short = 'c', long)]
    cipher_mode: CipherMode,

    /// IV (hex, 32 characters = 16 bytes), required for CBC encryption or decryption
    #[arg(long)]
    iv: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CipherMode {
    Ecb,
    Cbc,
}

fn main() {
    let cli = Cli::parse();

    // Parse key
    let key_bytes = hex::decode(&cli.key).expect("Invalid hex key");
    assert_eq!(key_bytes.len(), 16, "Key must be 16 bytes");
    let key: &[u8; 16] = key_bytes.as_slice().try_into().unwrap();

    // Read input file
    let input_data = fs::read(&cli.input).expect("Failed to read input file");

    // Parse IV if needed
    let iv: Option<[u8; 16]> = match cli.cipher_mode {
        CipherMode::Cbc => {
            let iv_str = cli.iv.as_ref().expect("IV is required for CBC mode");
            let iv_bytes = hex::decode(iv_str).expect("Invalid hex IV");
            assert_eq!(iv_bytes.len(), 16, "IV must be 16 bytes");
            Some(iv_bytes.as_slice().try_into().unwrap())
        }
        CipherMode::Ecb => None,
    };

    // Encrypt or decrypt based on mode and cipher
    let output_data = match (cli.mode, cli.cipher_mode) {
        (Mode::Encrypt, CipherMode::Ecb) => ECB.encrypt(&input_data, key, None),
        (Mode::Decrypt, CipherMode::Ecb) => ECB
            .decrypt(&input_data, key, None)
            .expect("Decryption failed"),

        (Mode::Encrypt, CipherMode::Cbc) => {
            let iv = iv.unwrap(); // already validated
            CBC.encrypt(&input_data, key, Some(&iv))
        }
        (Mode::Decrypt, CipherMode::Cbc) => {
            let iv = iv.unwrap(); // already validated
            CBC.decrypt(&input_data, key, Some(&iv))
                .expect("Decryption failed")
        }
    };

    // Write output
    fs::write(&cli.output, output_data).expect("Failed to write output file");
}
