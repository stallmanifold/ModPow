use num::{Zero, One, Num, BigInt, Integer, PrimInt};
use num::bigint::big_digit;
use std::cmp;


/// Computes the product of two integers modulo a modulus.
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

// Macro for generating ModMult implementations.
macro_rules! mod_mult {
    ( $T:ty ) => {
        impl ModMult for $T {
            fn mod_mult(self: &$T, other: &$T, modulus: &$T) -> $T {
                __mod_mult(self, other, modulus)
            }
        } 
    }
}

// Implementations of ModMult trait. 
mod_mult!(u8);
mod_mult!(u16);
mod_mult!(u32);
mod_mult!(u64);
mod_mult!(usize);
mod_mult!(i8);
mod_mult!(i16);
mod_mult!(i32);
mod_mult!(i64);
mod_mult!(isize);

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