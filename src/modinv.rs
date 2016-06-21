use num::{One, Integer, BigInt, Num};
use num::bigint::Sign;
use extended_gcd::ExtendedGcd;


/// Computes the modular inverse of an integer x: 
/// ```
/// y = x^-1 mod p
/// ```
/// where p is the modulus, and
/// ```
/// x*y = 1 mod p
/// ```
pub fn mod_inv(x: &BigInt, modulus: &BigInt) -> Option<BigInt> {
    let result = <BigInt as ExtendedGcd<&_>>::extended_gcd(x, modulus);
    match result {
        None             => None,
        Some(gcd_result) => {
            match gcd_result.coef_x.sign() {
                Sign::Plus | Sign::NoSign => Some(gcd_result.coef_x),
                Sign::Minus => Some(modulus + gcd_result.coef_x), 
            }
        }
    }
}
