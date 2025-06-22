pub fn modular_exponentiation(a: u64, b: u64, m: u64) -> u64 {
    // Exponentiation by squaring
    let mut alpha: u128 = a as u128;
    let mu: u128 = m as u128;
    alpha%=mu;
    let mut result:u128 = 1;

    //the case division here makes it so that we can save one computation of updating a by squaring it
    if b==0 {
        result.try_into().unwrap()
    }
    else {
        for i in 0..63-b.leading_zeros() {
            let bit = (b >> i) & 1;
        
            if bit == 1 {
                result = (result * alpha) % mu;
            }
        
            alpha = ( alpha*alpha )% mu;
        
        }
        ((result*alpha)%mu).try_into().unwrap()
    }
    
    
}
