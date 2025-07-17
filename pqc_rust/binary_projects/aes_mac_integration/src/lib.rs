use aes_toy::traits::{CipherMode, CipherModeImpl};
use mac_toy::traits::{MacMode, MacModeImpl};
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use mac_toy::PrefixMac;
use aes_toy::Cbc;
use aes_toy::Ecb;
use std::fmt;


pub mod commands;
pub mod errors;

pub use commands::{EncryptArgs, DecryptArgs, handle_encrypt, handle_decrypt};
pub use errors::CliError;


pub fn mac_mode(name: Option<&str>) -> Result<MacModeImpl, ModeSelectionError> {
    if let Some(n) = name {
        match n.to_lowercase().as_str() {
            "prefix" => Ok(MacModeImpl::Prefix(PrefixMac)),
            _ => Err(ModeSelectionError::UnknownMacMode(n.to_string())),
        }
    }
    else {
        Ok(MacModeImpl::default())
    }
    
}

pub fn cipher_mode(name: Option<&str>) -> Result<CipherModeImpl, ModeSelectionError> {
    if let Some(n) = name {
        match n.to_lowercase().as_str() {
            "cbc" => Ok(CipherModeImpl::Cbc(Cbc)),
            "ecb" => Ok(CipherModeImpl::Ecb(Ecb)),
            _ => Err(ModeSelectionError::UnknownCipherMode(n.to_string())),
        }
    }
    else {
        Ok(CipherModeImpl::default())
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct SecureMessage {
    pub ciphertext: Vec<u8>,
    pub iv: Option<[u8; 16]>,

    pub mac_mode_name: Option<String>, // uses MacModeImpl::default() for mac mode if None
    pub cipher_mode_name: Option<String>, // uses CipherModeImpl::default() for cipher mode if None

    pub inner_mac_tag: [u8;16], // authenticates just the ciphertext using the chosen mac
    pub outer_mac_tag: [u8;16], // authenticates all public fields (except itself) using default mac
}

pub fn serialize_mac_input(ciphertext: &[u8], iv: Option<&[u8; 16]>, mac_name: Option<&str>, cipher_name: Option<&str>, tag:&[u8;16]) -> Vec<u8> {
    let mut data = Vec::new();

    // MAC the ciphertext
    data.extend_from_slice(ciphertext);

    // MAC the IV, or 16 zero bytes if None (preserves structure)
    data.extend_from_slice(iv.unwrap_or(&[0u8; 16]));

    // MAC the mac algorithm name, as UTF-8 bytes
    if let Some(name) = mac_name {
        data.extend_from_slice(name.as_bytes());
    }
    else {
        data.extend_from_slice(MacModeImpl::default().name().as_bytes());
    }
    
    if let Some(name) = cipher_name {
        data.extend_from_slice(name.as_bytes());
    }
    else {
        data.extend_from_slice(CipherModeImpl::default().name().as_bytes());
    }

    data.extend_from_slice(tag);

    data
}

impl SecureMessage {
    pub fn mac_mode(&self) -> Result<MacModeImpl, ModeSelectionError> {
        mac_mode(self.mac_mode_name.as_ref().map(|s| s.as_str()))
    }

    pub fn cipher_mode(&self)-> Result<CipherModeImpl, ModeSelectionError> {
        cipher_mode(self.cipher_mode_name.as_ref().map(|s| s.as_str()))
    }

    pub fn outer_mac_is_valid(&self, key: &[u8;16]) -> bool {
        let mac_mode = MacModeImpl::default();
        if let Ok(tag_array) = self.outer_mac_tag.as_slice().try_into() {
            let data = serialize_mac_input(
                &self.ciphertext,
                self.iv.as_ref(),
                self.mac_mode_name.as_ref().map(|s| s.as_str()),
                self.cipher_mode_name.as_ref().map(|s| s.as_str()),
                &self.inner_mac_tag,
            );


            mac_mode.verify(key, &data, tag_array)
        } else {
            false
        }
    }

    pub fn inner_mac_is_valid(&self, key:&[u8;16]) -> bool {
        let Ok(mac_mode) = self.mac_mode() else {return false;};
            if let Ok(tag_array) = self.inner_mac_tag.as_slice().try_into() {
                mac_mode.verify(key, &self.ciphertext, tag_array)
            } else {
                false
            }
        
    }

    pub fn auth_decrypt(
        &self,
        key: &[u8; 16]
    ) -> Result<Vec<u8>, AuthError> {
            if !(self.outer_mac_is_valid(key) && self.inner_mac_is_valid(key)) {
                return Err(AuthError::MacVerificationFailed);
            }
            let cipher_mode = match self.cipher_mode(){
                Ok(c) => c,
                Err(e) => {return Err(AuthError::InvalidCipherMode(e));}
            };
            cipher_mode
                .decrypt( key, self.iv.as_ref(), &self.ciphertext,)
                .map_err(AuthError::DecryptionFailed)
        }
}

impl std::fmt::Debug for SecureMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SecureMessage {{ \
                ciphertext: {:02x?}, \
                outer_mac_tag: {:02x?}, \
                inner_mac_tag: {:02x?}, \
                iv: {:?}, \
                mac_mode_name: {}, \
                cipher_mode_name: {} \
            }}",
            self.ciphertext,
            self.outer_mac_tag,
            self.inner_mac_tag,
            self.iv,
            self.mac_mode_name.as_deref().unwrap_or("None"),
            self.cipher_mode_name.as_deref().unwrap_or("None"),
        )
    }
}


#[derive(Debug)]
pub enum AuthError {
    MacVerificationFailed,
    DecryptionFailed(aes_toy::DecryptionError),
    InvalidCipherMode(ModeSelectionError),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // Or provide custom messages
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeSelectionError {
    UnknownMacMode(String),
    UnknownCipherMode(String),
}

impl fmt::Display for ModeSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeSelectionError::UnknownMacMode(name) => write!(f, "Unknown MAC mode: {name}"),
            ModeSelectionError::UnknownCipherMode(name) => write!(f, "Unknown cipher mode: {name}"),
        }
    }
}

impl std::error::Error for ModeSelectionError {}

/// Encrypts a message using the provided AES mode and computes a MAC.
pub fn auth_encrypt(
    plaintext: &[u8],
    key: &[u8; 16],
    iv: Option<&[u8; 16]>,
    mac_mode_name: Option<&str>,
    cipher_mode_name: Option<&str>,
) -> Result<SecureMessage, ModeSelectionError> {
    let cipher = cipher_mode(cipher_mode_name)?;
    let mac = mac_mode(mac_mode_name)?;

    let (ciphertext, actual_iv) = cipher.encrypt( key, iv, plaintext);
    let inner_mac_tag = mac.compute(key, &ciphertext);
    let outer_mac_tag = MacModeImpl::default().compute(key, &serialize_mac_input(&ciphertext, actual_iv.as_ref(), mac_mode_name, cipher_mode_name, &inner_mac_tag));

    Ok(SecureMessage {
        ciphertext,
        iv: actual_iv,
        mac_mode_name: mac_mode_name.map(|s| s.to_string()),
        cipher_mode_name: cipher_mode_name.map(|s| s.to_string()),
        inner_mac_tag,
        outer_mac_tag,
    })
}

