use aes_mac_integration::{auth_encrypt, SecureMessage};
use aes_toy::{ECB, CBC};
use mac_toy::PrefixMac;
use aes_toy::traits::AesMode;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use bincode;

/// CLI for AES encryption with MAC
#[derive(Parser)]
#[command(name = "aes_mac_integration")]
#[command(about = "Encrypt/decrypt with AES + MAC", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt {
        /// Input file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file (optional). If not provided, prints JSON to stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// 16-byte hex key
        #[arg(short, long)]
        key: String,

        /// Cipher mode
        #[arg(long, value_enum)]
        mode: CipherMode,

        /// MAC algorithm
        #[arg(long, value_enum)]
        mac_mode: MacMode,
    },

    /// Decrypt a file
    Decrypt {
        /// Input encrypted file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file (optional). If not provided, prints plaintext to stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,


        /// 16-byte hex key
        #[arg(short, long)]
        key: String,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum CipherMode {
    Ecb,
    Cbc,
}

#[derive(Copy, Clone, ValueEnum)]
enum MacMode {
    Prefix,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt {
            input,
            output,
            key,
            mode,
            mac_mode,
        } => {
            let data = fs::read(input).expect("Failed to read input");
            let key_bytes = hex::decode(key).expect("Invalid hex key");
            let key: &[u8; 16] = key_bytes.as_slice().try_into().expect("Key must be 16 bytes");

            let cipher: Box<dyn AesMode> = match mode {
                CipherMode::Ecb => Box::new(ECB),
                CipherMode::Cbc => Box::new(CBC),
            };

            let mac = match mac_mode {
                MacMode::Prefix => PrefixMac::new(key),
            };

            let message = auth_encrypt(&data, key, None, mac, &*cipher);

            let serialized = bincode::serialize(&message).expect("Failed to serialize");

            if let Some(path) = output {
                fs::write(path, serialized).expect("Failed to write output file");
            } else {
                let json = serde_json::to_string_pretty(&message)
                    .expect("JSON serialization failed");
                println!("{}", json);
            }
        }

        Commands::Decrypt {
            input,
            output,
            key,
        } => {
            let input_data = fs::read(input).expect("Failed to read encrypted file");
            let key_bytes = hex::decode(key).expect("Invalid hex key");
            let key: &[u8; 16] = key_bytes.as_slice().try_into().expect("Key must be 16 bytes");

            let mut message: SecureMessage<PrefixMac> =
                bincode::deserialize(&input_data).expect("Invalid ciphertext format");

            // Re-insert the MAC object using the known name
            message.mac_mode = PrefixMac::new(key);



            let cipher: Box<dyn AesMode> = match message.iv {
                Some(_) => Box::new(CBC),
                None => Box::new(ECB),
            };

            let plaintext = message
                .decrypt_with(key, cipher.as_ref())
                .expect("Decryption failed or MAC invalid");

            if let Some(path) = output {
                fs::write(path, plaintext).expect("Failed to write plaintext output");
            } else {
                match std::str::from_utf8(&plaintext) {
                    Ok(text) => println!("{}", text),
                    Err(_) => println!("Plaintext (hex): {:02x?}", plaintext),
                }
            }

        }
    }
}
