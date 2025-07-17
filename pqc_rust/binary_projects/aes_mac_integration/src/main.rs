use aes_mac_integration::{EncryptArgs, DecryptArgs, handle_encrypt, handle_decrypt, CliError};
use clap::{Parser, Subcommand};

/// CLI for AES encryption with MAC
#[derive(Parser)]
#[command(name = "aes_mac_integration")]
#[command(about = "Encrypt/decrypt with AES + MAC")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt(EncryptArgs),

    /// Decrypt a file
    Decrypt(DecryptArgs),
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(args) => handle_encrypt(args),
        Commands::Decrypt(args) => handle_decrypt(args),
    }
}
