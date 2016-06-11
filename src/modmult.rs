use num::{Integer, Zero, One, Num, BigInt};


pub trait ModMult<T> {
    type Output; 

    fn mod_mult(base: T, exponent: T, modulus: T) -> Self::Output;
}

