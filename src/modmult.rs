use num::{Zero, One, Num, BigInt};


/// Computes the product ot two integers modulo a modulus.
pub trait ModMult<T> {
    type Output; 

    fn mod_mult(base: T, exponent: T, modulus: T) -> Self::Output;
}

