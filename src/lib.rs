#![feature(test)]
pub use modexp::ModExp;
pub use modmult::ModMult;
pub use extended_gcd::ExtendedGcd;
//pub use modinv::ModInv;


extern crate num;
extern crate test;

pub mod modexp;
pub mod modmult;
pub mod extended_gcd;
pub mod modinv;


#[cfg(test)]
mod modexp_tests;

#[cfg(all(test))]
mod modexp_bench;

#[cfg(test)]
mod extended_gcd_tests;

#[cfg(test)]
mod modinv_tests;