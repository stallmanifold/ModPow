use num::{Integer, Zero, One, PrimInt, BigInt, BigUint};


/// Trait for modular exponentiation.
///
/// The function `mod_exp` computes
/// ```
/// b^e mod m
/// ```
/// where b is the base, e is the exponent, and m is the modulus. 
pub trait ModExp<T> {
    type Output; 

    fn mod_exp(base: T, exponent: T, modulus: T) -> Self::Output;
}

#[inline]
fn __mod_exp<T: PrimInt>(base: T, exponent: T, modulus: T) -> T {
    let zero: T = Zero::zero();

    assert!(modulus != zero);

    let one = One::one();
    let two = one + one;

    if modulus == one {
        return zero;
    }

    let mut result = one;
    let mut modded_base = base % modulus;
    let mut divided_exponent = exponent;
        
    while divided_exponent > zero {
        if divided_exponent % two == one {
            result = result * modded_base % modulus;
        }
        divided_exponent = divided_exponent >> 1;
        modded_base = modded_base * modded_base % modulus;
    }

    assert!(result < modulus);

    result
}

impl<'a> ModExp<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn mod_exp(base: &'a BigInt, exponent: &'a BigInt, modulus: &'a BigInt) -> BigInt {
        let zero: BigInt = Zero::zero();

        assert!(*modulus != zero);

        let one: BigInt = One::one();
        let two: BigInt = <BigInt as One>::one() + <BigInt as One>::one();

        if *modulus == one {
            return zero;
        }

        let mut result: BigInt = One::one();
        let mut modded_base: BigInt = base.mod_floor(modulus);
        let mut divided_exponent: BigInt = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (&result * &modded_base).mod_floor(modulus);
            }
            divided_exponent = divided_exponent >> 1;
            modded_base = (&modded_base * &modded_base).mod_floor(modulus);
        }

        assert!(result < *modulus);

        result
    }
}

impl ModExp<BigInt> for BigInt {
    type Output = BigInt;

    fn mod_exp(base: BigInt, exponent: BigInt, modulus: BigInt) -> BigInt {
        <BigInt as ModExp<&BigInt>>::mod_exp(&base, &exponent, &modulus)
    }
}

impl<'a> ModExp<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn mod_exp(base: &'a BigUint, exponent: &'a BigUint, modulus: &'a BigUint) -> BigUint {
        let zero: BigUint = Zero::zero();

        assert!(*modulus != zero);

        let one: BigUint = One::one();
        let two: BigUint = <BigUint as One>::one() + <BigUint as One>::one();

        if *modulus == one {
            return zero;
        }

        let mut result: BigUint = One::one();
        let mut modded_base: BigUint = base.mod_floor(modulus);
        let mut divided_exponent: BigUint = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (&result * &modded_base).mod_floor(modulus);
            }
            divided_exponent = divided_exponent >> 1;
            modded_base = (&modded_base * &modded_base).mod_floor(modulus);
        }

        assert!(result < *modulus);

        result
    }
}

impl ModExp<BigUint> for BigUint {
    type Output = BigUint;

    fn mod_exp(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {
        <BigUint as ModExp<&BigUint>>::mod_exp(&base, &exponent, &modulus)
    }
}

impl ModExp<usize> for usize {
    type Output = usize;

    fn mod_exp(base: usize, exponent: usize, modulus: usize) -> usize {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<u8> for u8 {
    type Output = u8;

    fn mod_exp(base: u8, exponent: u8, modulus: u8) -> u8 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<u16> for u16 {
    type Output = u16;

    fn mod_exp(base: u16, exponent: u16, modulus: u16) -> u16 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<u32> for u32 {
    type Output = u32;

    fn mod_exp(base: u32, exponent: u32, modulus: u32) -> u32 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<u64> for u64 {
    type Output = u64;

    fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<isize> for isize {
    type Output = isize;

    fn mod_exp(base: isize, exponent: isize, modulus: isize) -> isize {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<i8> for i8 {
    type Output = i8;

    fn mod_exp(base: i8, exponent: i8, modulus: i8) -> i8 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<i16> for i16 {
    type Output = i16;

    fn mod_exp(base: i16, exponent: i16, modulus: i16) -> i16 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<i32> for i32 {
    type Output = i32;

    fn mod_exp(base: i32, exponent: i32, modulus: i32) -> i32 {
        __mod_exp(base, exponent, modulus)
    }
}

impl ModExp<i64> for i64 {
    type Output = i64;

    fn mod_exp(base: i64, exponent: i64, modulus: i64) -> i64 {
        __mod_exp(base, exponent, modulus)
    }
}