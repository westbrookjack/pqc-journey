use hkdf::Hkdf;
use sha2::Sha256;

pub fn derive_key(master_key: &[u8], salt: Option<&[u8]>, info: &[u8], out_len: usize) -> Vec<u8> {
    
    let hk = Hkdf::<Sha256>::new(salt,master_key);
    
    let mut okm = vec![0u8; out_len];
    hk.expand(info, &mut okm).expect("HKDF expand failed");

    okm
}

pub fn derive_keys_for_secure_message(master_key: &[u8], salt: Option<&[u8]>, info: &[u8], out_len: usize) -> (Vec<u8>, Vec<u8>){
    
}