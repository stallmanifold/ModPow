use num::{Zero, BigUint, BigInt, Integer, PrimInt};


/// The `ModSub` trait defines an interface for calculating the difference of two integers
/// modulo a modulus.
pub trait ModSub {
    /// The function `mod_sub` computes the sum
    /// ```text
    /// x - y (mod m)
    /// ```
    /// where x, y, and m are integers, and m is the modulus.
    ///
    /// # Panics
    /// when the modulus is zero.
    fn mod_sub(self: &Self, other: &Self, modulus: &Self) -> Self;
}

impl ModSub for BigInt {
    fn mod_sub(self: &BigInt, other: &BigInt, modulus: &BigInt) -> BigInt {
        (self - other).mod_floor(modulus)
    }
}

impl ModSub for BigUint {
    fn mod_sub(self: &BigUint, other: &BigUint, modulus: &BigUint) -> BigUint {
        (self - other).mod_floor(modulus)
    }
}

#[inline]
fn __mod_sub<T: PrimInt>(x: &T, y: &T, modulus: &T) -> T {
    assert!(*modulus != <T as Zero>::zero());

    ((*x) - (*y)) % *modulus
}

// Macro for generating ModSub implementations.
macro_rules! mod_sub {
    ( $ T : ty ) => {
        impl ModSub for $T {
            fn mod_sub(self: &$T, other: &$T, modulus: &$T) -> $T {
                __mod_sub(self, other, modulus)
            }
        }
    }
}

// Implementations of ModSub trait.
mod_sub!(u8);
mod_sub!(u16);
mod_sub!(u32);
mod_sub!(u64);
mod_sub!(usize);
mod_sub!(i8);
mod_sub!(i16);
mod_sub!(i32);
mod_sub!(i64);
mod_sub!(isize);

#[cfg(test)]
mod tests {
    use num::BigInt;
    use super::ModSub;


    struct TestCase {
        x: BigInt,
        y: BigInt,
        modulus: BigInt,
        diff: BigInt,
    }

    struct Test {
        data: Vec<TestCase>,
    }

    fn mod_sub_test_cases() -> Test {
        Test {
            data: vec! [
                TestCase {
                    x: BigInt::from(464283712),
                    y: BigInt::from(559073817),
                    modulus: BigInt::from(635717262),
                    diff: BigInt::from(540927157),
                },
                TestCase {
                    x: BigInt::from(-812470905),
                    y: BigInt::from(228473033),
                    modulus: BigInt::from(538744916),
                    diff: BigInt::from(36545894),
                },
                TestCase {
                    x: BigInt::from(227791838),
                    y: BigInt::from(233974561),
                    modulus: BigInt::from(681539081),
                    diff: BigInt::from(675356358),
                },
                TestCase {
                    x: BigInt::from(735172765),
                    y: BigInt::from(446253906),
                    modulus: BigInt::from(53235608),
                    diff: BigInt::from(22740819),
                }
            ]
        }
    }

    fn run_tests(tests: &Test) {
        for test_case in tests.data.iter() {
            let result = test_case.x.mod_sub(&test_case.y, &test_case.modulus);

            assert_eq!(result, test_case.diff);
        }
    }

    #[test]
    fn test_mod_sub() {
        run_tests(&mod_sub_test_cases());
    }

    #[test]
    #[should_panic]
    fn test_mod_sub_zero_mod() {
        let x = BigInt::from(735172765);
        let y = BigInt::from(446253906);
        let modulus = BigInt::from(0);

        x.mod_sub(&y, &modulus);
    }
}
