use num::{Zero, One, BigInt, PrimInt};
use num::bigint::Sign;
use extended_gcd::ExtendedGcd;


pub trait ModInv<T> {
    /// Computes the modular inverse of an integer x: 
    /// ```
    /// y = x^-1 mod p
    /// ```
    /// where p is the modulus, and
    /// ```
    /// x*y = 1 mod p
    /// ```
    fn mod_inv(&self, modulus: &T) -> Option<T>;
}

impl ModInv<BigInt> for BigInt {
    fn mod_inv(&self, modulus: &BigInt) -> Option<BigInt> {
        let result = self.extended_gcd(modulus);
        
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

#[inline]
fn __mod_inv<T>(x: &T, modulus: &T) -> Option<T> where T: PrimInt + ExtendedGcd<T> {
    let result = x.extended_gcd(modulus);
    
    result.and_then(|gcd_result| {
        match gcd_result.gcd_xy == One::one() {
            true => {
                match gcd_result.coef_x >= Zero::zero() {
                    true  => Some(gcd_result.coef_x),
                    false => Some(*modulus + gcd_result.coef_x),
                }
            }
            false => None,
        }
    })
}

impl ModInv<isize> for isize {
    fn mod_inv(&self, modulus: &isize) -> Option<isize> {
        __mod_inv(self, modulus)
    }
}

impl ModInv<i8> for i8 {
    fn mod_inv(&self, modulus: &i8) -> Option<i8> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i16> for i16 {
    fn mod_inv(&self, modulus: &i16) -> Option<i16> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i32> for i32 {
    fn mod_inv(&self, modulus: &i32) -> Option<i32> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i64> for i64 {
    fn mod_inv(&self, modulus: &i64) -> Option<i64> {
        __mod_inv(self, modulus)        
    }
}
