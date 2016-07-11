use num::{Zero, One, BigInt, PrimInt};
use num::bigint::Sign;
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
            match gcd_result.gcd_xy == One::one() {
                true => {
                    match gcd_result.coef_x.sign() {
                        Sign::Plus | Sign::NoSign => Some(gcd_result.coef_x),
                        Sign::Minus => Some(modulus + gcd_result.coef_x), 
                    }
                }
                false => None,
            }
        })
    }
}

#[inline]
fn __mod_inv<T>(x: &T, modulus: &T) -> Option<T> where T: PrimInt + ExtendedGcd<T> {
    let result = x.extended_gcd(modulus);
    
    result.and_then(|gcd_result| {
        match gcd_result.gcd_xy == One::one() {
            true => {
                match gcd_result.coef_x >= Zero::zero() {
                    true  => Some(gcd_result.coef_x),
                    false => Some(*modulus + gcd_result.coef_x),
                }
            }
            false => None,
        }
    })
}

impl ModInv<isize> for isize {
    fn mod_inv(&self, modulus: &isize) -> Option<isize> {
        __mod_inv(self, modulus)
    }
}

impl ModInv<i8> for i8 {
    fn mod_inv(&self, modulus: &i8) -> Option<i8> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i16> for i16 {
    fn mod_inv(&self, modulus: &i16) -> Option<i16> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i32> for i32 {
    fn mod_inv(&self, modulus: &i32) -> Option<i32> {
        __mod_inv(self, modulus)        
    }
}

impl ModInv<i64> for i64 {
    fn mod_inv(&self, modulus: &i64) -> Option<i64> {
        __mod_inv(self, modulus)        
    }
}


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