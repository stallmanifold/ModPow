#![feature(test)]
pub use modexp::ModExp;
pub use modmult::ModMult;


extern crate num;
extern crate test;

pub mod modexp;
pub mod modmult;

#[cfg(test)]
mod tests;

#[cfg(all(test))]
mod bench;