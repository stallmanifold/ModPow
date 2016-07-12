#![feature(test)]
pub use modexp::ModExp;
pub use modmult::ModMult;
pub use extended_gcd::ExtendedGcd;
pub use modinv::ModInv;


extern crate num;
extern crate test;

pub mod modexp;
pub mod modmult;
pub mod extended_gcd;
pub mod modinv;
pub mod modular;