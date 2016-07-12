use num::{Zero, One, Num, BigInt, Integer, PrimInt};
use num::bigint::big_digit;
use std::cmp;


/// Computes the modular product of an integer and the inverse of another integer,
/// provided that the inverse exists.
pub trait ModDiv {
    fn mod_div(self: &Self, other: &Self, modulus: &Self) -> Option<Self>;
}