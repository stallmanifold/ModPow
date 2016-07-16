use num::{Zero, One, BigInt, Integer, PrimInt};


/// Computes the product of two integers modulo a modulus.
pub trait ModMult {
    fn mod_mult(self: &Self, other: &Self, modulus: &Self) -> Self;
}

#[inline]
fn __mod_mult<T: PrimInt>(x: &T, y: &T, modulus: &T) -> T {
    assert!(*modulus != <T as Zero>::zero());

    ((*x) * (*y)) % *modulus
}

#[inline]
fn montgomery_multiply(num: &BigInt, other_num: &BigInt, modulus: &BigInt) -> BigInt {
    let base = BigInt::from(1 << 8);
    // The base and modulus MUST be coprime for montgomery multiplication to work.
    assert!(*modulus != <BigInt as Zero>::zero());
    assert!(base.gcd(modulus) == <BigInt as One>::one());
    // TODO: Replace this stub with the actual Montgomery multiplication algorithm.
    (num * other_num).mod_floor(modulus)
}

#[inline]
fn default_multiply(num: &BigInt, other_num: &BigInt, modulus: &BigInt) -> BigInt {
    assert!(*modulus != <BigInt as Zero>::zero());

    (num * other_num).mod_floor(modulus)
}

impl ModMult for BigInt {
    fn mod_mult(self: &BigInt, other: &BigInt, modulus: &BigInt) -> BigInt {
        if modulus.is_odd() {
            montgomery_multiply(self, other, modulus)
        } else {
            default_multiply(self, other, modulus)
        }
    }
}

// Macro for generating ModMult implementations.
macro_rules! mod_mult {
    ( $ T : ty ) => {
        impl ModMult for $T {
            fn mod_mult(self: &$T, other: &$T, modulus: &$T) -> $T {
                __mod_mult(self, other, modulus)
            }
        } 
    }
}

// Implementations of ModMult trait. 
mod_mult!(u8);
mod_mult!(u16);
mod_mult!(u32);
mod_mult!(u64);
mod_mult!(usize);
mod_mult!(i8);
mod_mult!(i16);
mod_mult!(i32);
mod_mult!(i64);
mod_mult!(isize);


#[cfg(test)]
mod tests {
    use num::BigInt;
    use modmult::ModMult;


    struct TestCase {
        x: BigInt,
        y: BigInt,
        modulus: BigInt,
        product: BigInt,
    }

    struct Test {
        data: Vec<TestCase>,
    }

    fn mod_mult_test_cases() -> Test {
        Test {
            data: vec! [
                TestCase {
                    x: BigInt::from(464283712),
                    y: BigInt::from(559073817),
                    modulus: BigInt::from(635717262),
                    product: BigInt::from(474093474),
                },
                TestCase {
                    x: BigInt::from(812470905),
                    y: BigInt::from(228473033),
                    modulus: BigInt::from(538744916),
                    product: BigInt::from(435097989),
                },
                TestCase {
                    x: BigInt::from(227791838),
                    y: BigInt::from(233974561),
                    modulus: BigInt::from(681539081),
                    product: BigInt::from(309428767),
                },
                TestCase {
                    x: BigInt::from(735172765),
                    y: BigInt::from(446253906),
                    modulus: BigInt::from(53235608),
                    product: BigInt::from(14862186),
                }
            ]
        }
    }

    fn run_tests(tests: &Test) {
        for test_case in tests.data.iter() {
            let result = test_case.x.mod_mult(&test_case.y, &test_case.modulus);

            assert_eq!(result, test_case.product);
        }
    }

    #[test]
    fn test_mod_mult() {
        run_tests(&mod_mult_test_cases());
    }
}