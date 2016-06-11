use num::{BigInt};
use algos;

struct GcdTest {
    data: Vec<GcdTestCase>,
}

struct GcdTestCase {
    x: BigInt,
    y: BigInt,
    v: BigInt,
    a: BigInt,
    b: BigInt,
}

fn gcd_test_cases() -> GcdTest {
    GcdTest {
        data: vec![
            GcdTestCase {
                x: BigInt::from(693),
                y: BigInt::from(609),
                v: BigInt::from(21),
                a: BigInt::from(-181),
                b: BigInt::from(206),
            }
        ]
    }
}

fn run_gcd_test(test: & GcdTest) {
    for test_case in test.data.iter() {
        let result = algos::extended_gcd(&test_case.x, &test_case.y).unwrap();
        assert_eq!(result.3, test_case.v);
        assert_eq!(result.0, test_case.a);
        assert_eq!(result.1, test_case.b);
    }
}

#[test]
fn test_extended_gcd() {
    run_gcd_test(&gcd_test_cases());
}