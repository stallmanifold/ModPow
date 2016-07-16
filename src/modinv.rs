use num::{Zero, One, BigInt, PrimInt};
use extended_gcd::ExtendedGcd;


pub trait ModInv<T> {
    /// Computes the modular inverse of an integer x: 
    /// ```
    /// y = x^-1 mod p
    /// ```
    /// where p is the modulus, and
    /// ```
    /// x*y = 1 mod p
    /// ```
    fn mod_inv(&self, modulus: &T) -> Option<T>;
}

impl ModInv<BigInt> for BigInt {
    fn mod_inv(&self, modulus: &BigInt) -> Option<BigInt> {
        let result = self.extended_gcd(modulus);
        
        result.and_then(|gcd_result| {
            if gcd_result.gcd_xy == One::one() {
                if gcd_result.coef_x >= Zero::zero() {
                    Some(gcd_result.coef_x)
                } else {
                    Some(modulus + gcd_result.coef_x)
                }
            } else {
                None
            }
        })
    }
}

#[inline]
fn __mod_inv<T>(x: &T, modulus: &T) -> Option<T> where T: PrimInt + ExtendedGcd<T> {
    let result = x.extended_gcd(modulus);
    
    result.and_then(|gcd_result| {
        if gcd_result.gcd_xy == One::one() {
            if gcd_result.coef_x >= Zero::zero() {
                Some(gcd_result.coef_x)
            } else {
                Some(*modulus + gcd_result.coef_x)
            }
        } else {
            None
        }
    })
}

// macro for defining ModInv implementations.
macro_rules! mod_inv {
    ( $ T : ty ) => {
        impl ModInv<$T> for $T {
            fn mod_inv(&self, modulus: &$T) -> Option<$T> {
                __mod_inv(self, modulus)
            }
        }
    } 
}

mod_inv!(i8);
mod_inv!(i16);
mod_inv!(i32);
mod_inv!(i64);
mod_inv!(isize);


#[cfg(test)]
mod tests {
    use num::{One, Integer, BigInt, Num};
    use super::ModInv;
    use std::fmt::Debug;


    struct TestCase<T> {
        a:       T,
        a_inv:   T,
        modulus: T,
    }

    struct Test<T> {
        data: Vec<TestCase<T>>,
    }

    // Non-invertible test cases for mod_inv.
    struct NonInvTestCase<T> {
        a: T,
        modulus: T,
    }

    struct NonInvTest<T> {
        data: Vec<NonInvTestCase<T>>,
    }

    fn bigint_test_cases() -> Test<BigInt> {
        Test {
            data: vec![
                TestCase {
                    a:       BigInt::from(633),
                    a_inv:   BigInt::from(177),
                    modulus: BigInt::from(2801),
                },
                TestCase {
                    a:       BigInt::from(271),
                    a_inv:   BigInt::from(106),
                    modulus: BigInt::from(383),
                },
                TestCase {
                    a:       <BigInt as Num>::from_str_radix("2983498573497", 10).unwrap(),
                    a_inv:   <BigInt as Num>::from_str_radix("515317423113", 10).unwrap(),
                    modulus: <BigInt as Num>::from_str_radix("903455098240", 10).unwrap(),
                },
                TestCase {
                    a:       <BigInt as Num>::from_str_radix("60192921923322822", 10).unwrap(),
                    a_inv:   <BigInt as Num>::from_str_radix("368992488398249", 10).unwrap(),
                    modulus: <BigInt as Num>::from_str_radix("427414198414469", 10).unwrap(),
                }
            ]
        }
    }

    fn isize_test_cases() -> Test<isize> {
        Test {
            data: vec![
                TestCase {
                    a:       633,
                    a_inv:   177,
                    modulus: 2801,
                },
                TestCase {
                    a:       271,
                    a_inv:   106,
                    modulus: 383,
                },
                TestCase {
                    a:       67,
                    a_inv:   9567,
                    modulus: 17324,
                },
                TestCase {
                    a:       39357,
                    a_inv:   10218,
                    modulus: 33695,
                }
            ]
        }
    }

    fn non_invertible_isize_test_cases() -> NonInvTest<isize> {
        NonInvTest {
            data: vec![
                NonInvTestCase {
                    a:       61,
                    modulus: 17324,
                },
                NonInvTestCase {
                    a:       3404,
                    modulus: 456458,
                }
            ]
        }
    }

    fn non_invertible_bigint_test_cases() -> NonInvTest<BigInt> {
        NonInvTest {
            data: vec![
                NonInvTestCase {
                    a:       BigInt::from(61),
                    modulus: BigInt::from(17324),
                },
                NonInvTestCase {
                    a:       BigInt::from(3404),
                    modulus: BigInt::from(456458),
                }
            ]
        }
    }

    fn run_tests<T>(test: &Test<T>) 
        where T: Integer + ModInv<T> + Debug + Clone {

        for test_case in test.data.iter() {
            let result = test_case.a.mod_inv(&test_case.modulus);

            assert!(result.is_some());

            let result = result.unwrap();

            assert_eq!(Integer::gcd(&test_case.a, &test_case.modulus), One::one());
            assert_eq!(result, test_case.a_inv);
        }
    }

    fn run_non_inv_tests<T>(test: &NonInvTest<T>) 
       where T: Integer + ModInv<T> + Debug + Clone {

        for test_case in test.data.iter() {
            let result = test_case.a.mod_inv(&test_case.modulus);

            assert!(result.is_none());
        }
    }

    #[test]
    fn test_mod_inverse() {
        run_tests(&bigint_test_cases());
    }

    #[test]
    fn test_mod_inverse_isize() {
        run_tests(&isize_test_cases());
    }

    #[test]
    fn test_non_invertible() {
        run_non_inv_tests(&non_invertible_isize_test_cases());
    }

    #[test]
    fn test_non_invertible_bigint() {
        run_non_inv_tests(&non_invertible_bigint_test_cases());
    }
}

#[cfg(test)]
mod bench {
    use super::ModInv;
    use test::Bencher;
    use num::{Num, BigInt};

    struct TestCase {
        value: BigInt,
        modulus: BigInt,
    }

    struct Test {
        data: Vec<TestCase>,
    }

    fn bench_test_case() -> Test {
        let value = <BigInt as Num>::from_str_radix("2983498573497", 10).unwrap();
        let modulus =  <BigInt as Num>::from_str_radix("903455098240", 10).unwrap();

        Test {
            data: vec! [
                TestCase {
                    value: value,
                    modulus: modulus,
                }
            ]
        }
    }

    #[bench]
    fn bench_mod_inv(bencher: &mut Bencher) {
        let test = bench_test_case();

        for test_case in test.data.iter() {
            bencher.iter(|| test_case.value.mod_inv(&test_case.modulus));      
        }

    }
}
