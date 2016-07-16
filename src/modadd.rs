use num::{Zero, BigUint, BigInt, Integer, PrimInt};


/// Computes the product of two integers modulo a modulus.
pub trait ModAdd {
    fn mod_add(self: &Self, other: &Self, modulus: &Self) -> Self;
}

#[inline]
fn __mod_add<T: PrimInt>(x: &T, y: &T, modulus: &T) -> T {
    assert!(*modulus != <T as Zero>::zero());

    ((*x) + (*y)) % *modulus
}

impl ModAdd for BigInt {
    fn mod_add(self: &BigInt, other: &BigInt, modulus: &BigInt) -> BigInt {
        (self + other).mod_floor(modulus)
    }
}

impl ModAdd for BigUint {
    fn mod_add(self: &BigUint, other: &BigUint, modulus: &BigUint) -> BigUint {
        (self + other).mod_floor(modulus)
    }
}

// Macro for generating ModAdd implementations.
macro_rules! mod_add {
    ( $ T : ty ) => {
        impl ModAdd for $T {
            fn mod_add(self: &$T, other: &$T, modulus: &$T) -> $T {
                __mod_add(self, other, modulus)
            }
        } 
    }
}

// Implementations of ModAdd trait. 
mod_add!(u8);
mod_add!(u16);
mod_add!(u32);
mod_add!(u64);
mod_add!(usize);
mod_add!(i8);
mod_add!(i16);
mod_add!(i32);
mod_add!(i64);
mod_add!(isize);

#[cfg(test)]
mod tests {
    use num::BigInt;
    use super::ModAdd;


    struct TestCase {
        x: BigInt,
        y: BigInt,
        modulus: BigInt,
        sum: BigInt,
    }

    struct Test {
        data: Vec<TestCase>,
    }

    fn mod_add_test_cases() -> Test {
        Test {
            data: vec! [
                TestCase {
                    x: BigInt::from(464283712),
                    y: BigInt::from(559073817),
                    modulus: BigInt::from(635717262),
                    sum: BigInt::from(387640267),
                },
                TestCase {
                    x: BigInt::from(-812470905),
                    y: BigInt::from(228473033),
                    modulus: BigInt::from(538744916),
                    sum: BigInt::from(493491960),
                },
                TestCase {
                    x: BigInt::from(227791838),
                    y: BigInt::from(233974561),
                    modulus: BigInt::from(681539081),
                    sum: BigInt::from(461766399),
                },
                TestCase {
                    x: BigInt::from(735172765),
                    y: BigInt::from(446253906),
                    modulus: BigInt::from(53235608),
                    sum: BigInt::from(10243295),
                }
            ]
        }
    }

    fn run_tests(tests: &Test) {
        for test_case in tests.data.iter() {
            let result = test_case.x.mod_add(&test_case.y, &test_case.modulus);

            assert_eq!(result, test_case.sum);
        }
    }

    #[test]
    fn test_mod_add() {
        run_tests(&mod_add_test_cases());
    }

    #[test]
    #[should_panic]
    fn test_mod_add_zero_mod() {
        let x = BigInt::from(735172765);
        let y = BigInt::from(446253906);
        let modulus = BigInt::from(0);

        x.mod_add(&y, &modulus);
    }
}
