use num::{Integer, Zero, One, Num, PrimInt, BigInt, BigUint};


pub trait ModMul<T> {
    type Output; 

    fn mod_mul(base: T, exponent: T, modulus: T) -> Self::Output;
}