use crate::PrefixMac;

pub trait MacMode {
    fn compute(&self, key:&[u8;16], message: &[u8]) -> [u8; 16];
    fn verify(&self, key: &[u8;16], message: &[u8], tag: &[u8; 16]) -> bool;
    fn name(&self) -> &'static str;
}

pub enum MacModeImpl {
    Prefix(PrefixMac),
}

impl MacMode for MacModeImpl {
    fn compute(&self, key: &[u8;16], msg:&[u8]) -> [u8;16] {
        match self {
            MacModeImpl::Prefix(p) => p.compute(key, msg),
        }
    }

    fn verify(&self, key: &[u8;16], msg: &[u8], tag: &[u8;16]) -> bool {
        match self {
            MacModeImpl::Prefix(p) => p.verify(key,msg,tag),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            MacModeImpl::Prefix(p) => p.name(),
        }
    }
}

//Once I get more secure macs then Prefix will be replaced
impl Default for MacModeImpl {
    fn default() -> Self {
        MacModeImpl::Prefix(PrefixMac::default())
    }
}