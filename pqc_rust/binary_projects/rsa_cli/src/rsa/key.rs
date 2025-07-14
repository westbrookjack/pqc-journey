use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct RsaPublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[derive(Debug, Clone)]
pub struct RsaPrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}


pub fn keygen(bits: usize) -> (RsaPublicKey, RsaPrivateKey) {
    unimplemented!()
}
