use pqc_rust::number_theory::{ mod_exp::modular_exponentiation, extended_gcd::gcd_triple };

fn main() {
    let base = 2;
    let exp = 4;
    let modulus = 256;

    let result = modular_exponentiation(base, exp, modulus);
    println!("Result: {}", result);


    println!("{:?}",gcd_triple(3,0));
}
