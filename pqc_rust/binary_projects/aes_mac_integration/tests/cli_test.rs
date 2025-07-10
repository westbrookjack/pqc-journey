use assert_cmd::Command;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_encrypt_decrypt() {
    // Create a temp file with some test input
    let mut input_file = NamedTempFile::new().unwrap();
    writeln!(input_file, "This is a secret message").unwrap();
    let input_path = input_file.path();

    // Create a temp output file path for encrypted file
    let output_file = NamedTempFile::new().unwrap();
    let output_path = output_file.path();

    // Encryption command
    Command::cargo_bin("aes_mac_integration")
        .unwrap()
        .args([
            "encrypt",
            "--input", input_path.to_str().unwrap(),
            "--output", output_path.to_str().unwrap(),
            "--key", "00112233445566778899aabbccddeeff",
            "--mode", "ecb",             // <--- lowercase
            "--mac-mode", "prefix",      // <--- lowercase, if needed
        ])
        .assert()
        .success();

    // Create a new temp file to hold decrypted output
    let decrypted_file = NamedTempFile::new().unwrap();
    let decrypted_path = decrypted_file.path();

    // Decryption command
    Command::cargo_bin("aes_mac_integration")
        .unwrap()
        .args([
            "decrypt",
            "--input",
            output_path.to_str().unwrap(),
            "--output",
            decrypted_path.to_str().unwrap(),
            "--key",
            "00112233445566778899aabbccddeeff",
        ])
        .assert()
        .success();

    // Check the decrypted output
    let decrypted_contents = fs::read_to_string(decrypted_path).unwrap();
    assert!(decrypted_contents.contains("This is a secret message"));
}
