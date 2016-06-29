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

    if (*x < zero) || (*y < zero) {
        return None;
    }

    if (*x == zero) || (*y == zero) {
        return None;
    }

    Some(__extended_gcd(x, y))
}

#[inline]
fn __valid_solution<T: PrimInt>(x: &T, y: &T, coef_x: &T, coef_y: &T, gcd_xy: &T) -> bool {
    (*coef_x) * (*x) + (*coef_y) * (*y) == *gcd_xy
}

impl ExtendedGcd<isize> for isize {
    fn extended_gcd(&self, y: &isize) -> Option<Gcd<isize>> {
        safe_extended_gcd(self, y)
    }

    fn valid_solution(x: &isize, y: &isize, coef_x: &isize, coef_y: &isize, gcd_xy: &isize) -> bool {
        __valid_solution(x, y, coef_x, coef_y, gcd_xy)
    }
}

impl ExtendedGcd<i8> for i8 {
    fn extended_gcd(&self, y: &i8) -> Option<Gcd<i8>> {
        safe_extended_gcd(self, y)
    }

    fn valid_solution(x: &i8, y: &i8, coef_x: &i8, coef_y: &i8, gcd_xy: &i8) -> bool {
        __valid_solution(x, y, coef_x, coef_y, gcd_xy)
    }
}

impl ExtendedGcd<i16> for i16 {
    fn extended_gcd(&self, y: &i16) -> Option<Gcd<i16>> {
        safe_extended_gcd(self, y)
    }

    fn valid_solution(x: &i16, y: &i16, coef_x: &i16, coef_y: &i16, gcd_xy: &i16) -> bool {
        __valid_solution(x, y, coef_x, coef_y, gcd_xy)
    }
}

impl ExtendedGcd<i32> for i32 {
    fn extended_gcd(&self, y: &i32) -> Option<Gcd<i32>> {
        safe_extended_gcd(self, y)
    }

    fn valid_solution(x: &i32, y: &i32, coef_x: &i32, coef_y: &i32, gcd_xy: &i32) -> bool {
        __valid_solution(x, y, coef_x, coef_y, gcd_xy)
    }
}

impl ExtendedGcd<i64> for i64 {
    fn extended_gcd(&self, y: &i64) -> Option<Gcd<i64>> {
        safe_extended_gcd(self, y)
    }

    fn valid_solution(x: &i64, y: &i64, coef_x: &i64, coef_y: &i64, gcd_xy: &i64) -> bool {
        __valid_solution(x, y, coef_x, coef_y, gcd_xy)
    }
}
