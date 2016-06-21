use num::{One, Integer, BigInt, Num};
use mod_inv::ModInv;


struct TestCase {
    a:       BigInt,
    a_inv:   BigInt,
    modulus: BigInt,
}

struct Test {
    data: Vec<TestCase>,
}

fn mod_inverse_test_cases() -> Test {
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

fn run_mod_inverse_test(test: &Test) {
    for test_case in test.data.iter() {
        let result = algos::mod_inv(&test_case.a, &test_case.modulus);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(Integer::gcd(&test_case.a, &test_case.modulus), One::one());
        assert_eq!(result, test_case.a_inv);
    }
}

#[test]
fn test_mod_inverse() {
    run_mod_inverse_test(&mod_inverse_test_cases());
}
