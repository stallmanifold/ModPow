#![feature(test)]
pub use modexp::ModExp;
pub use modmult::ModMult;


extern crate num;
extern crate test;

pub mod modexp;
pub mod modmult;

mod algos;

#[cfg(test)]
mod modexp_tests;

#[cfg(all(test))]
mod modexp_bench;

#[cfg(test)]
mod algos_tests;