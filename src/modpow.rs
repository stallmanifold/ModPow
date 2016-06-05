use num::{Integer, Zero, One, Num};


pub trait ModPow {
    type IntType;

    fn mod_pow(base: &Self::IntType, exponent: &Self::IntType, modulus: &Self::IntType) -> Self::IntType;
}

impl<T> ModPow for T where T: Clone + Num + Integer {
    type IntType = T;

    fn mod_pow(base: &T, exponent: &T, modulus: &T) -> T {
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

        debug_assert!(result < *modulus);

        result
    }
}
