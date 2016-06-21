use num::{Zero, One, BigInt, PrimInt};
use num::bigint::Sign;
use extended_gcd::ExtendedGcd;


pub trait ModInv<T, Output> {
    /// Computes the modular inverse of an integer x: 
    /// ```
    /// y = x^-1 mod p
    /// ```
    /// where p is the modulus, and
    /// ```
    /// x*y = 1 mod p
    /// ```
    fn mod_inv(x: T, modulus: T) -> Option<Output>;
}

impl<'a> ModInv<&'a BigInt, BigInt> for BigInt {

    fn mod_inv(x: &BigInt, modulus: &BigInt) -> Option<BigInt> {
        let result = <BigInt as ExtendedGcd<&_,_>>::extended_gcd(x, modulus);
        
        result.and_then(|gcd_result| {
            match gcd_result.gcd_xy == One::one() {
                true => {
                    match gcd_result.coef_x.sign() {
                        Sign::Plus | Sign::NoSign => Some(gcd_result.coef_x),
                        Sign::Minus => Some(modulus + gcd_result.coef_x), 
                    }
                }
                false => None,
            }
        })
    }
}

impl ModInv<BigInt, BigInt> for BigInt {

    fn mod_inv(x: BigInt, modulus: BigInt) -> Option<BigInt> {
        <BigInt as ModInv<&_,_>>::mod_inv(&x, &modulus)
    }
}

#[inline]
fn __mod_inv<T>(x: T, modulus: T) -> Option<T> where T: PrimInt + ExtendedGcd<T,T> {
    let result = <T as ExtendedGcd<T,T>>::extended_gcd(x, modulus);
    
    result.and_then(|gcd_result| {
        match gcd_result.gcd_xy == One::one() {
            true => {
                match gcd_result.coef_x >= Zero::zero() {
                    true  => Some(gcd_result.coef_x),
                    false => Some(modulus + gcd_result.coef_x),
                }
            }
            false => None,
        }
    })
}

impl ModInv<isize, isize> for isize {

    fn mod_inv(x: isize, modulus: isize) -> Option<isize> {
        __mod_inv(x, modulus)
    }
}

impl ModInv<i8, i8> for i8 {

    fn mod_inv(x: i8, modulus: i8) -> Option<i8> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i16, i16> for i16 {

    fn mod_inv(x: i16, modulus: i16) -> Option<i16> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i32, i32> for i32 {

    fn mod_inv(x: i32, modulus: i32) -> Option<i32> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i64, i64> for i64 {

    fn mod_inv(x: i64, modulus: i64) -> Option<i64> {
        __mod_inv(x, modulus)        
    }
}
