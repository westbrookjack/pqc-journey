use crate::{auth_encrypt, CliError};
use clap::Args;
use clap::arg;



use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct EncryptArgs {
    /// Input file
        #[arg(short, long)]
        pub input: PathBuf,

        /// Output file (optional). If not provided, prints JSON to stdout.
        #[arg(short, long)]
        pub output: Option<PathBuf>,

        /// 16-byte hex key
        #[arg(short, long)]
        pub key: String,

        /// Cipher mode
        #[arg(long, value_enum)]
        pub cipher_mode: Option<String>,

        /// MAC algorithm
        #[arg(long, value_enum)]
        pub mac_mode: Option<String>,

        /// Verbose
        #[arg(long)]
        pub verbose: bool,
}



pub fn handle_encrypt(args: EncryptArgs) -> Result<(), CliError> {
    if args.verbose {
        print_mode_summary_encrypt(
            &args.input,
            &args.output,
            &args.key,
            args.cipher_mode.as_deref().unwrap_or("unknown"),
            args.mac_mode.as_deref().unwrap_or("unknown"),
        );

    }
    let data = match fs::read(&args.input) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("❌ Failed to read input file: {}", args.input.display());
            return Err(CliError::Io(e));
        }
    };

    let key_bytes = hex::decode(args.key).map_err(|e| {
        eprintln!("Invalid hex key");
        CliError::Hex(e)
    })?;

    
    let key: &[u8; 16] = match key_bytes.as_slice().try_into() {
        Ok(k) => k,
        Err(_) => {
            eprintln!("Key must be 16 bytes");
            return Err(CliError::InvalidKeyLength);
        }
    };


    let message = match auth_encrypt(&data, key, None, args.mac_mode.as_deref(), args.cipher_mode.as_deref()) {
        Ok(m)=>m,
        Err(e)=> {
            eprintln!("❌ Mode selection failed: {e}");
            return Err(CliError::UnsupportedMode(e));

        }
    };

    let serialized = match bincode::serialize(&message) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to serialize: {e}");
            return Err(CliError::Bincode(e.into()));
        }
    };

    if let Some(path) = args.output {
    if let Err(e) = fs::write(path, serialized) {
        eprintln!("Failed to write output file: {e}");
        return Err(CliError::Io(e));
    }
    } else {
        let json = match serde_json::to_string_pretty(&message) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("JSON serialization failed: {e}");
                return Err(CliError::SerdeJson(e));
            }
        };
        println!("{}", json);
    }
    println!("✅ Encryption successful.");
    Ok(())
}


fn print_mode_summary_encrypt(
    input: &PathBuf,
    output: &Option<PathBuf>,
    key: &str,
    cipher_mode: &str,
    mac_mode: &str,
) {
    println!("\n🔐 Encrypting with the following settings:");
    println!("  📄 Input File:  {}", input.display());
    match output {
        Some(path) => println!("  💾 Output File: {}", path.display()),
        None => println!("  📤 Output: printed as JSON to stdout"),
    }
    println!("  🔑 AES Key:     {} (16 bytes)", key);
    println!("  🔁 Cipher Mode: {}", cipher_mode);
    println!("  🧩 MAC Mode:    {}", mac_mode);
    println!();
}