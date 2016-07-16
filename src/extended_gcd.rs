use num::{Integer, Zero, One, BigInt, PrimInt};
use num::bigint::Sign;


/// A data structure storing the results of computing the greatest common
/// divisor of two integers.
pub struct Gcd<T> {
    pub coef_x: T,
    pub coef_y: T,
    pub g:      T,
    pub gcd_xy: T,
}

/// Extended Gcd Algorithm trait.
pub trait ExtendedGcd<T> {
    /// Implementation of the binary extended gcd algorithm.
    ///
    /// See Algorithm 14.61 of the 'Handbook of Applied Cryptography'.
    ///
    /// Given integers x and y, compute integers a and b such that
    ///
    /// a*x + b*y = v where v = gcd(x, y).
    ///
    /// Note that the coefficients found by the extended GCD algorithm are not unique: That is,
    /// there is more than one set of solutions to the diophantine equation above.
    ///
    /// # Safety
    /// Returns None if x < 0 or y < 0.
    fn extended_gcd(&self, y: &T) -> Option<Gcd<T>>;

    /// Tests whether a pair of coefficients coef_x and coef_t are 
    /// valid solutions to the equation
    /// ```
    /// coef_x * x + coef_y * y = gcd_xy
    /// ```
    /// where 
    /// ```
    /// gcd_xy = gcd(x,y)
    /// ```
    fn valid_solution(x: &T, y: &T, coef_x: &T, coef_y: &T, gcd_xy: &T) -> bool;
}

impl ExtendedGcd<BigInt> for BigInt {
    fn extended_gcd(&self, y: &BigInt) -> Option<Gcd<BigInt>> {
        if (self.sign() == Sign::Minus) || (y.sign() == Sign::Minus) {
            return None;
        }

        let zero = <BigInt as Zero>::zero();

        if (*self == zero) || (*y == zero) {
            return None;
        }

        Some(__extended_gcd_bigint(self, y))
    }

    fn valid_solution(x: &BigInt, y: &BigInt, coef_x: &BigInt, coef_y: &BigInt, gcd_xy: &BigInt) -> bool {
        coef_x * x + coef_y * y == *gcd_xy
    }
}

// Panics if x and y are nonpositive.
#[inline]
fn __extended_gcd_bigint(x: &BigInt, y: &BigInt) -> Gcd<BigInt> {
    let zero: BigInt = Zero::zero();

    assert!((*x > zero) && (*y > zero));

    let mut xx = x.clone();
    let mut yy = y.clone();
    let mut g: BigInt = One::one();

    while (&xx).is_even() && (&yy).is_even() {
        xx = xx >> 1;
        yy = yy >> 1;
        g  = g << 1;
    }

    let mut u: BigInt = xx.clone();
    let mut v: BigInt = yy.clone();
    let mut a: BigInt = One::one();
    let mut b: BigInt = Zero::zero();
    let mut c: BigInt = Zero::zero();
    let mut d: BigInt = One::one();

    loop {
        while (&u).is_even() {
            u = u >> 1;

            if (&a).is_even() && (&b).is_even() {
                a = a >> 1;
                b = b >> 1;
            } else {
                a = (a + &yy) >> 1;
                b = (b - &xx) >> 1;
            }
        }

        while (&v).is_even() {
            v = v >> 1;
            
            if (&c).is_even() && (&d).is_even() {
                c = c >> 1;
                d = d >> 1;
            } else {
                c = (c + &yy) >> 1;
                d = (d - &xx) >> 1;
            }
        }

        if u >= v {
            u = u - &v;
            a = a - &c;
            b = b - &d;
        } else {
            v = v - &u;
            c = c - &a;
            d = d - &b;
        }

        if u == zero {
            v = &g * v;

            return Gcd {
                coef_x: c,
                coef_y: d,
                g:      g,
                gcd_xy: v,
            }
        }
    }
}

// Panics if x and y are nonpositive.
#[inline]
fn __extended_gcd<T: PrimInt>(x: &T, y: &T) -> Gcd<T> {
    let zero: T = Zero::zero();
    let mask: T = One::one();

    assert!((*x > zero) && (*y > zero));

    let mut xx = *x;
    let mut yy = *y;
    let mut g: T = One::one();

    while (xx & mask == zero) && (yy & mask == zero) {
        xx = xx >> 1;
        yy = yy >> 1;
        g  = g << 1;
    }

    let mut u: T = xx;
    let mut v: T = yy;
    let mut a: T = One::one();
    let mut b: T = Zero::zero();
    let mut c: T = Zero::zero();
    let mut d: T = One::one();

    loop {
        while u & mask == zero {
            u = u >> 1;

            if (a & mask == zero) && (b & mask == zero) {
                a = a >> 1;
                b = b >> 1;
            } else {
                a = (a + yy) >> 1;
                b = (b - xx) >> 1;
            }
        }

        while v & mask == zero {
            v = v >> 1;
            
            if (c & mask == zero) && (d & mask == zero) {
                c = c >> 1;
                d = d >> 1;
            } else {
                c = (c + yy) >> 1;
                d = (d - xx) >> 1;
            }
        }

        if u >= v {
            u = u - v;
            a = a - c;
            b = b - d;
        } else {
            v = v - u;
            c = c - a;
            d = d - b;
        }

        if u == zero {
            v = g * v;

            return Gcd {
                coef_x: c,
                coef_y: d,
                g:      g,
                gcd_xy: v,
            }
        }
    }
}

#[inline]
fn safe_extended_gcd<T: PrimInt>(x: &T, y: &T) -> Option<Gcd<T>> {
    let zero = <T as Zero>::zero();

    if (*x <= zero) || (*y <= zero) {
        return None;
    }

    Some(__extended_gcd(x, y))
}

#[inline]
fn __valid_solution<T: PrimInt>(x: &T, y: &T, coef_x: &T, coef_y: &T, gcd_xy: &T) -> bool {
    (*coef_x) * (*x) + (*coef_y) * (*y) == *gcd_xy
}

macro_rules! extended_gcd {
    ( $ T : ty ) => {
        impl ExtendedGcd<$T> for $T {
            fn extended_gcd(&self, y: &$T) -> Option<Gcd<$T>> {
                safe_extended_gcd(self, y)
            }

            fn valid_solution(x: &$T, y: &$T, coef_x: &$T, coef_y: &$T, gcd_xy: &$T) -> bool {
                __valid_solution(x, y, coef_x, coef_y, gcd_xy)
            }       
        }
    }
}

extended_gcd!(i8);
extended_gcd!(i16);
extended_gcd!(i32);
extended_gcd!(i64);
extended_gcd!(isize);


#[cfg(test)]
mod tests {
    use num::{BigInt, Num};
    use super::ExtendedGcd;


    struct Test {
        data: Vec<TestCase>,
    }

    struct TestCase {
        x:      BigInt,
        y:      BigInt,
        gcd_xy: BigInt,
        coef_x: BigInt,
        coef_y: BigInt,
    }

    fn gcd_test_cases() -> Test {
        Test {
            data: vec![
                // Example 14.62 from 'Handbook of Applied Cryptography'
                TestCase {
                    x:      BigInt::from(693),
                    y:      BigInt::from(609),
                    gcd_xy: BigInt::from(21),
                    coef_x: BigInt::from(-181),
                    coef_y: BigInt::from(206),
                },
                TestCase {
                    x:      <BigInt as Num>::from_str_radix("294248851906335666255475965306356\
                                                             33692994051181214434796327203075", 10).unwrap(),
                    y:      <BigInt as Num>::from_str_radix("919087970205406919189208074679995\
                                                             123273961", 10).unwrap(),
                    gcd_xy: <BigInt as Num>::from_str_radix("3", 10).unwrap(),
                    coef_x: <BigInt as Num>::from_str_radix("-13780862337125063622355860403035\
                                                              4666602474", 10).unwrap(),
                    coef_y: <BigInt as Num>::from_str_radix("441198563405422365116499908277872\
                                                             3845569132647537334425498184473", 10).unwrap(),
                },
                TestCase {
                    x:      BigInt::from(613769282),
                    y:      BigInt::from(716888961),
                    gcd_xy: BigInt::from(29),
                    coef_x: BigInt::from(2233618),
                    coef_y: BigInt::from(-1912327),
                },
                TestCase {
                    x:      <BigInt as Num>::from_str_radix("240007433584325356921800502290759\
                                                         31889728948", 10).unwrap(),
                    y:      <BigInt as Num>::from_str_radix("476057445640162830420892551794736\
                                                             263384096274661520", 10).unwrap(),
                    gcd_xy: <BigInt as Num>::from_str_radix("4", 10).unwrap(),
                    coef_x: <BigInt as Num>::from_str_radix("57452030383374486115869206982212629264831027293013", 10).unwrap(),
                    coef_y: <BigInt as Num>::from_str_radix("-2896481189991724314184721919272517544310316", 10).unwrap(), 
                },
                TestCase {
                    x:      <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000", 10).unwrap(),
                    y:      <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000", 10).unwrap(),
                    gcd_xy: <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000\
                                                             000000000000000000000000000000000000000000", 10).unwrap(),
                    coef_x: <BigInt as Num>::from_str_radix("0", 10).unwrap(),
                    coef_y: <BigInt as Num>::from_str_radix("1", 10).unwrap(),
                }
            ]
        }
    }

    fn run_gcd_test(test: &Test) {
        for test_case in test.data.iter() {
            let result = <BigInt as ExtendedGcd<_>>::extended_gcd(&test_case.x, &test_case.y).unwrap();
            assert_eq!(result.gcd_xy, test_case.gcd_xy);
            assert!(<BigInt as ExtendedGcd<_>>::valid_solution(&test_case.x, &test_case.y, 
                                              &result.coef_x, &result.coef_y, &test_case.gcd_xy));
            // Show that coefficients returned by the extended_gcd algorithm are not unique.
            assert!(<BigInt as ExtendedGcd<_>>::valid_solution(&test_case.x, &test_case.y, 
                                        &test_case.coef_x, &test_case.coef_y, &test_case.gcd_xy));
        }
    }

    #[test]
    fn test_extended_gcd() {
        run_gcd_test(&gcd_test_cases());
    }

}