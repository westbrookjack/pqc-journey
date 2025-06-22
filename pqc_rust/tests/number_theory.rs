use pqc_rust::number_theory::{mod_exp::modular_exponentiation, extended_gcd::gcd_triple};

//modular exponentiation tests

#[test]
fn test_mod_exp_basic() {
    // 2^10 = 1024, 1024 mod 1000 = 24
    assert_eq!(modular_exponentiation(2, 10, 1000), 24);
}

#[test]
fn test_mod_exp_zero_exponent() {
    // x^0 mod m == 1 for any x, m > 1
    assert_eq!(modular_exponentiation(5, 0, 13), 1);
    assert_eq!(modular_exponentiation(123456789, 0, 999), 1);
}

#[test]
fn test_mod_exp_zero_base() {
    // 0^b mod m == 0 for any b > 0
    assert_eq!(modular_exponentiation(0, 5, 7), 0);
    assert_eq!(modular_exponentiation(0, 123456, 2), 0);
}


#[test]
fn test_mod_exp_modulus_one() {
    // Anything mod 1 is 0
    assert_eq!(modular_exponentiation(10, 20, 1), 0);
    assert_eq!(modular_exponentiation(123456, 789, 1), 0);
}


#[test]
fn test_mod_exp_edge_bit_boundary() {
    assert_eq!(modular_exponentiation(2, 63, 1_000_000_007), 291172004);
    assert_eq!(modular_exponentiation(2, 62, 1_000_000_007), 145586002);
}

#[test]
fn test_mod_exp_large_exponent() {
    assert_eq!(modular_exponentiation(3, 64, 1_000_000_007), 767713261);
}



// gcd tests

#[test]
fn test_basic_gcds() {
    let (gcd, x, y) = gcd_triple(240, 46);
    assert_eq!(gcd, 2);
    assert_eq!(240u64 as i128 * x + 46u64 as i128 * y, gcd as i128);

    let (gcd, x, y) = gcd_triple(56, 15);
    assert_eq!(gcd, 1);
    assert_eq!(56u64 as i128 * x + 15u64 as i128 * y, gcd as i128);

    let (gcd, x, y) = gcd_triple(99, 78);
    assert_eq!(gcd, 3);
    assert_eq!(99u64 as i128 * x + 78u64 as i128 * y, gcd as i128);
}

#[test]
fn test_zero_cases() {
    let (gcd, x, y) = gcd_triple(0, 0);
    assert_eq!(gcd, 0);
    assert_eq!(0, 0 * x + 0 * y);

    let (gcd, x, y) = gcd_triple(0, 5);
    assert_eq!(gcd, 5);
    assert_eq!(0u64 as i128 * x + 5u64 as i128 * y, 5);

    let (gcd, x, y) = gcd_triple(7, 0);
    assert_eq!(gcd, 7);
    assert_eq!(7u64 as i128 * x + 0u64 as i128 * y, 7);
}

#[test]
fn test_large_inputs() {
    let a = 1_000_000_000_000_000_003;
    let b = 1_000_000_000_000_000_000;
    let (gcd, x, y) = gcd_triple(a, b);
    assert_eq!(a as i128 * x + b as i128 * y, gcd as i128);
}