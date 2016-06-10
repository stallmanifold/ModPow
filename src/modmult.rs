use num::{Integer, Zero, One, Num, PrimInt, BigInt, BigUint};


pub trait ModMult<T> {
    type Output; 

    fn mod_mult(base: T, exponent: T, modulus: T) -> Self::Output;
}
