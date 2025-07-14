pub trait MacModeImpl {
    fn new(key: &[u8; 16]) -> Self where Self: Sized;
    fn compute(&self, message: &[u8]) -> [u8; 16];
    fn verify(&self, message: &[u8], tag: &[u8; 16]) -> bool;
    fn name(&self) -> &'static str;
}
