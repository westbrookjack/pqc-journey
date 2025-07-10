use sha2::{Sha256, Digest};
use crate::traits::MacModeImpl;


#[derive(Clone)]
pub struct PrefixMac {
    key: [u8; 16],
}


impl PrefixMac {
    pub fn new(key: &[u8; 16]) -> Self {
        Self { key: *key }
    }

    pub fn compute(&self, m: &[u8]) -> [u8; 16] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(m);
        let result = hasher.finalize();
        let mut tag = [0u8; 16];
        tag.copy_from_slice(&result[..16]);
        tag
    }

    pub fn verify(&self, m: &[u8], t: &[u8; 16]) -> bool {
        self.compute(m) == *t
    }
}

impl MacModeImpl for PrefixMac {
    fn new(key: &[u8; 16]) -> Self {
        PrefixMac::new(key)
    }

    fn compute(&self, message: &[u8]) -> [u8; 16] {
        self.compute(message)
    }

    fn verify(&self, message: &[u8], tag: &[u8; 16]) -> bool {
        self.verify(message, tag)
    }
}
