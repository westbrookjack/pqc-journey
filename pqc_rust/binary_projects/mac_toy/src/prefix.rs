use sha2::{Sha256, Digest};
use crate::traits:: MacMode;
use serde::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct PrefixMac;


impl PrefixMac {
    pub fn compute(&self, k:&[u8;16], m: &[u8]) -> [u8; 16] {
        let mut hasher = Sha256::new();
        hasher.update(k);
        hasher.update(m);
        let result = hasher.finalize();
        let mut tag = [0u8; 16];
        tag.copy_from_slice(&result[..16]);
        tag
    }

    pub fn verify(&self,k:&[u8;16], m: &[u8], t: &[u8; 16]) -> bool {
        self.compute(k, m) == *t
    }
}

impl MacMode for PrefixMac {
    fn compute(&self, key:&[u8;16], message: &[u8] ) -> [u8; 16] {
        self.compute(key, message)
    }

    fn verify(&self,key:&[u8;16], message: &[u8], tag: &[u8; 16]) -> bool {
        self.verify(key, message, tag)
    }

    fn name(&self) -> &'static str {
        "prefix"
    }
}

impl Default for PrefixMac {
    fn default() -> Self {
        PrefixMac
    }
}