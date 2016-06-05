use num::{Integer, Zero, One, Num, PrimInt, BigInt, BigUint};


pub trait ModExp<T> {
    //type IntType;
    type Output;

    fn mod_exp(base: T, exponent: T, modulus: T) -> Self::Output;
}

pub trait BigInteger: Clone + Integer + Num {}

impl BigInteger for BigInt {}
impl BigInteger for BigUint {}

impl<'a, T> ModExp<&'a T> for T where T: BigInteger {
    //type IntType = &'a T;
    type Output = T;

    fn mod_exp(base: &'a T, exponent: &'a T, modulus: &'a T) -> T {
        let zero: T = Zero::zero();

        assert!(*modulus != zero);

        let one: T = One::one();
        let two: T = <T as One>::one() + <T as One>::one();

        if *modulus == one {
            return zero;
        }

        let mut result: T = One::one();
        let mut modded_base: T = base.mod_floor(modulus);
        let mut divided_exponent: T = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (result * modded_base.clone()).mod_floor(modulus);
            }
            divided_exponent = divided_exponent.div_floor(&two);
            modded_base = (modded_base.clone() * modded_base.clone()).mod_floor(modulus);
        }

        assert!(result < *modulus);

        result
    }
}

impl<T> ModExp<T> for T where T: BigInteger {
    //type IntType = T;
    type Output = T;

    fn mod_exp(base: T, exponent: T, modulus: T) -> T {
        let zero: T = Zero::zero();

        assert!(modulus != zero);

        let one: T = One::one();
        let two: T = <T as One>::one() + <T as One>::one();

        if modulus == one {
            return zero;
        }

        let mut result: T = One::one();
        let mut modded_base: T = base.mod_floor(&modulus);
        let mut divided_exponent: T = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (result * modded_base.clone()).mod_floor(&modulus);
            }
            divided_exponent = divided_exponent.div_floor(&two);
            modded_base = (modded_base.clone() * modded_base.clone()).mod_floor(&modulus);
        }

        assert!(result < modulus);

        result
    }
}
/*
impl<T> ModExp<T> for T where T: PrimInt  {
    type Output = T;

    fn mod_exp(base: T, exponent: T, modulus: T) -> T {
        let zero = Zero::zero();

        assert!(modulus != zero);

        let one = One::one();
        let two = <usize as One>::one() + <usize as One>::one();

        if modulus == one {
            return zero;
        }

        let mut result  = one;
        let mut modded_base = base % modulus;
        let mut divided_exponent = exponent;
        
        while divided_exponent > zero {
            if divided_exponent % two == one {
                result = result * modded_base % modulus;
            }
            divided_exponent = divided_exponent / two;
            modded_base = modded_base * modded_base % modulus;
        }

        assert!(result < modulus);

        result
    }
}
*/