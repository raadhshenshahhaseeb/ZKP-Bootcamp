extern crate num_rational;
extern crate num_bigint;
extern crate num_traits;

use num_rational::BigRational;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

fn main() {
    let field: u32 = 71;
    
    q1(field);
    q2(field);
    q3(field);
    q4(field);
    q5(field);
}

/// Print congruent values for a set of integers in a given field
fn q1(field: u32) {
    let mut values = vec![
        BigRational::from_integer(BigInt::from(-1)),
        BigRational::from_integer(BigInt::from(-4)),
        BigRational::from_integer(BigInt::from(-160)),
        BigRational::from_integer(BigInt::from(500)),
    ];
    
    congruent(&mut values, field);
}

/// Verify additive properties in a field
fn q2(field: u32) {
    /* 
    steps:
    a = 5 * pow(6,-1,p) % p

    b = 11 * pow(12,-1,p)

    c = 21 * pow(12,-1,p)

    then (a + b) % p == c % p

    */

    let values = vec![
        BigRational::new(BigInt::from(5), BigInt::from(6)),
        BigRational::new(BigInt::from(11), BigInt::from(12)),
        BigRational::new(BigInt::from(21), BigInt::from(12)),
    ];

    // Verify that a + b = c in GF(field)
    let a = &values[0];
    let b = &values[1];
    let c = &values[2];

    let a_congruent = congruent_single(a.clone(), field);
    let b_congruent = congruent_single(b.clone(), field);
    let c_congruent = congruent_single(c.clone(), field);

    let sum_ab = (a_congruent.clone() + b_congruent.clone()).numer() % BigInt::from(field);

    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("a + b = {} in GF({})", sum_ab, field);
    println!("Is a + b == c? {}", sum_ab == *c_congruent.numer());
}

/// Verify multiplicative properties in a field
fn q3(field: u32) {
    let values = vec![
        BigRational::new(BigInt::from(2), BigInt::from(3)),
        BigRational::new(BigInt::from(1), BigInt::from(2)),
        BigRational::new(BigInt::from(1), BigInt::from(3)),
    ];

    // Verify that a * b = c in GF(field)
    let a = &values[0];
    let b = &values[1];
    let c = &values[2];

    let a_congruent = congruent_single(a.clone(), field);
    let b_congruent = congruent_single(b.clone(), field);
    let c_congruent = congruent_single(c.clone(), field);

    let mul_ab = (a_congruent.clone() * b_congruent.clone()).numer() % BigInt::from(field);

    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("a * b = {} in GF({})", mul_ab, field);
    println!("Is a * b == c? {}", mul_ab == *c_congruent.numer());
}

/// Finds a modular square root, if it exists
fn q4(field: u32) {
    let rational = BigRational::from_integer(BigInt::from(12));
    let value = congruent_single(rational, field);

    if let Some(sqrt) = find_modular_square_root(value.to_u32().unwrap_or(0), field) {
        println!("A modular square root of 12 modulo {} is: {}", field, sqrt);
    } else {
        println!("No modular square root found.");
    }
}

fn q5(field: u32) {
    // p(x)= 52x^2 + 24x + 61
    // q(x) = 40x^2 + 40x + 58
    // find p(x) + q(x) and p(x)*q(x)

    // Addition p(x) + q(x)
    let p: [u32; 3] = [52, 24, 61];  // Coefficients [x^2, x^1, x^0]
    let q: [u32; 3] = [40, 40, 58];  // Coefficients [x^2, x^1, x^0]

    let sum_coeffs: Vec<u32> = p.iter()
    .zip(q.iter())
    .map(|(&p, &q)| (p + q) % field)
    .collect();


    let mut r: [u32; 5] = [0; 5];

    // Polynomial multiplication
    for i in 0..p.len() {
        for j in 0..q.len() {
            r[i + j] = (r[i + j] + p[i] * q[j]) % field;
        }
    }

    // Output the polynomial coefficients in conventional order
    println!("Product of p(x) * q(x) = {}x^4 + {}x^3 + {}x^2 + {}x + {}",
             r[0], r[1], r[2], r[3], r[4]);

    println!("Sum of p(x) + q(x) = {}x^2 + {}x + {}", sum_coeffs[0], sum_coeffs[1], sum_coeffs[2]);
}


/// Adjusts all values in the vector to be congruent within the specified field
fn congruent(values: &mut Vec<BigRational>, field: u32) {
    for value in values.iter_mut() {
        *value = congruent_single(value.clone(), field);
        println!("{} in GF({})", value, field);
    }
}

/// Returns the congruent form of a rational number within a field
fn congruent_single(value: BigRational, field: u32) -> BigRational {
    let field_bigint = BigInt::from(field);
    let numer = (value.numer() % &field_bigint + &field_bigint) % &field_bigint;
    let denom = (value.denom() % &field_bigint + &field_bigint) % &field_bigint;

    let denom_inv = mod_inverse(&denom, &field_bigint);
    let result_numer = (numer * denom_inv) % &field_bigint;

    BigRational::new(result_numer, BigInt::from(1))
}

/// Uses Extended Euclidean Algorithm to find the modular inverse
fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let mut mn = (m.clone(), a.clone());
    let mut xy = (BigInt::from(0), BigInt::from(1));

    while mn.1 != BigInt::from(0) {
        let quotient = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &quotient * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &quotient * &xy.1);
    }

    while xy.0 < BigInt::from(0) {
        xy.0 += m;
    }

    xy.0
}

/// Helper to find a modular square root, if it exists
fn find_modular_square_root(target: u32, field: u32) -> Option<u32> {
    for x in 1..field {
        if (x * x) % field == target {
            return Some(x);
        }
    }
    None
}
