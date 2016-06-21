use num::{Zero, One, BigInt, PrimInt};
use num::bigint::Sign;
use extended_gcd::ExtendedGcd;


pub trait ModInv<T> {
    type Output;

    /// Computes the modular inverse of an integer x: 
    /// ```
    /// y = x^-1 mod p
    /// ```
    /// where p is the modulus, and
    /// ```
    /// x*y = 1 mod p
    /// ```
    fn mod_inv(x: T, modulus: T) -> Option<Self::Output>;
}

impl<'a> ModInv<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn mod_inv(x: &BigInt, modulus: &BigInt) -> Option<BigInt> {
        let result = <BigInt as ExtendedGcd<&_,_>>::extended_gcd(x, modulus);
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
}

impl ModInv<BigInt> for BigInt {
    type Output = BigInt;

    fn mod_inv(x: BigInt, modulus: BigInt) -> Option<BigInt> {
        <BigInt as ModInv<&_>>::mod_inv(&x, &modulus)
    }
}

fn __mod_inv<T: PrimInt + ExtendedGcd<T,T>>(x: T, modulus: T) -> Option<T> {
    let result = <T as ExtendedGcd<T,T>>::extended_gcd(x, modulus);
    match result {
        None             => None,
        Some(gcd_result) => {
            let zero = Zero::zero();

            if gcd_result.coef_x >= zero {
                Some(gcd_result.coef_x)
            } else {
                Some(modulus + gcd_result.coef_x)
            }
        }
    }
}

impl ModInv<isize> for isize {
    type Output = isize;

    fn mod_inv(x: isize, modulus: isize) -> Option<isize> {
        __mod_inv(x, modulus)
    }
}

impl ModInv<i8> for i8 {
    type Output = i8;

    fn mod_inv(x: i8, modulus: i8) -> Option<i8> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i16> for i16 {
    type Output = i16;

    fn mod_inv(x: i16, modulus: i16) -> Option<i16> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i32> for i32 {
    type Output = i32;

    fn mod_inv(x: i32, modulus: i32) -> Option<i32> {
        __mod_inv(x, modulus)        
    }
}

impl ModInv<i64> for i64 {
    type Output = i64;

    fn mod_inv(x: i64, modulus: i64) -> Option<i64> {
        __mod_inv(x, modulus)        
    }
}
