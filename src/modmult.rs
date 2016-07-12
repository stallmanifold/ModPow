use num::{Zero, One, Num, BigInt, Integer, PrimInt};
use num::bigint::big_digit;
use std::cmp;


/// Computes the product ot two integers modulo a modulus.
pub trait ModMult {
    fn mod_mult(self: &Self, other: &Self, modulus: &Self) -> Self;
}

#[inline]
fn __mod_mult<T: PrimInt>(x: &T, y: &T, modulus: &T) -> T {
    assert!(*modulus != <T as Zero>::zero());

    ((*x) * (*y)) % *modulus
}

#[inline]
fn montgomery_multiply(num: &BigInt, other_num: &BigInt, modulus: &BigInt) -> BigInt {
    let base = BigInt::from(1 << 8);
    // The base and modulus MUST be coprime for montgomery multiplication to work.
    assert!(*modulus != <BigInt as Zero>::zero());
    assert!(base.gcd(modulus) == <BigInt as One>::one());
    // TODO: Replace this stub with the actual Montgomery multiplication algorithm.
    (num * other_num).mod_floor(modulus)
}

#[inline]
fn default_multiply(num: &BigInt, other_num: &BigInt, modulus: &BigInt) -> BigInt {
    assert!(*modulus != <BigInt as Zero>::zero());

    (num * other_num).mod_floor(modulus)
}

impl ModMult for u8 {
    fn mod_mult(self: &u8, other: &u8, modulus: &u8) -> u8 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for u16 {
    fn mod_mult(self: &u16, other: &u16, modulus: &u16) -> u16 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for u32 {
    fn mod_mult(self: &u32, other: &u32, modulus: &u32) -> u32 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for u64 {
    fn mod_mult(self: &u64, other: &u64, modulus: &u64) -> u64 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for usize {
    fn mod_mult(self: &usize, other: &usize, modulus: &usize) -> usize {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for i8 {
    fn mod_mult(self: &i8, other: &i8, modulus: &i8) -> i8 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for i16 {
    fn mod_mult(self: &i16, other: &i16, modulus: &i16) -> i16 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for i32 {
    fn mod_mult(self: &i32, other: &i32, modulus: &i32) -> i32 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for i64 {
    fn mod_mult(self: &i64, other: &i64, modulus: &i64) -> i64 {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for isize {
    fn mod_mult(self: &isize, other: &isize, modulus: &isize) -> isize {
        __mod_mult(self, other, modulus)
    }
}

impl ModMult for BigInt {
    fn mod_mult(self: &BigInt, other: &BigInt, modulus: &BigInt) -> BigInt {
        if modulus.is_odd() {
            montgomery_multiply(self, other, modulus)
        } else {
            default_multiply(self, other, modulus)
        }
    }
}

#[cfg(test)]
mod tests {

}