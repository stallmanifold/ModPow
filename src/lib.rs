#![feature(test)]
pub use modexp::ModExp;
pub use modmul::ModMul;


extern crate num;
extern crate test;

pub mod modexp;
pub mod modmul;

#[cfg(test)]
mod tests;

#[cfg(all(test))]
mod bench;