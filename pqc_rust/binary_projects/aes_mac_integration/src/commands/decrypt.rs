use crate::{SecureMessage, CliError};
use std::fs;
use std::path::PathBuf;
use clap::arg;


#[derive(clap::Args)]
pub struct DecryptArgs {
    /// Input encrypted file
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file (optional). If not provided, prints plaintext to stdout.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 16-byte hex key
    #[arg(short, long)]
    pub key: String,

    /// Verbose
    #[arg(long)]
    pub verbose: bool,
}

pub fn handle_decrypt(args: DecryptArgs) -> Result<(), CliError> {
    if args.verbose {
        print_mode_summary_decrypt(&args.input, &args.output, &args.key);
    }

    let data = match fs::read(&args.input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Failed to read input file: {e}");
            return Err(CliError::Io(e));
        }
    };

    let key_bytes = match hex::decode(&args.key) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("❌ Invalid hex key format: {e}");
            return Err(CliError::Hex(e));
        }
    };

    let key: &[u8; 16] = match key_bytes.as_slice().try_into() {
        Ok(k) => k,
        Err(_) => {
            eprintln!("❌ AES key must be 16 bytes (32 hex characters)");
            return Err(CliError::InvalidKeyLength);
        }
    };

    let message: SecureMessage = match bincode::deserialize(&data) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Invalid ciphertext format: {e}");
            return Err(CliError::Bincode(e));
        }
    };

    let plaintext = match message.auth_decrypt(key) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Message failed to decrypt: {e}");
            return Err(CliError::Auth(e));
        }
    };

    if let Some(path) = &args.output {
        if let Err(e) = fs::write(path, &plaintext) {
            eprintln!("❌ Failed to write plaintext output: {e}");
            return Err(CliError::Io(e));
        }
    } else {
        match std::str::from_utf8(&plaintext) {
            Ok(text) => println!("{}", text),
            Err(e) => {
                println!("📦 Plaintext (hex): {:02x?}", plaintext);
                return Err(CliError::Utf8(e));
            }
        }
    }

    println!("✅ Decryption successful.");
    Ok(())
}

fn print_mode_summary_decrypt(input: &PathBuf, output: &Option<PathBuf>, key: &str) {
    println!("\n🔓 Decrypting with the following settings:");
    println!("  📄 Input File:  {}", input.display());
    match output {
        Some(path) => println!("  💾 Output File: {}", path.display()),
        None => println!("  📤 Output: printed plaintext to stdout"),
    }
    println!("  🔑 AES Key:     {} (16 bytes)", key);
    println!();
}
