use num::{One, Integer, BigInt, Num};
use modinv::ModInv;
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