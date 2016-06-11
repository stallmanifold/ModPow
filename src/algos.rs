use num::{Integer, Zero, One, BigInt};
use num::bigint::Sign;

pub struct GcdResult {
    pub coef_x: BigInt,
    pub coef_y: BigInt,
    pub g:      BigInt,
    pub gcd_xy: BigInt,
}

/// 
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
pub fn extended_gcd(x: &BigInt, y: &BigInt) -> Option<GcdResult> {
    if (x.sign() == Sign::Minus) || (y.sign() == Sign::Minus) {
        return None;
    }

    Some(__extended_gcd(x, y))
}


/// Panics if x and y are nonpositive.
#[inline]
fn __extended_gcd(x: &BigInt, y: &BigInt) -> GcdResult {
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

            return GcdResult {
                coef_x: c,
                coef_y: d,
                g:      g,
                gcd_xy: v,
            }
        }
    }
}

pub fn valid_solution(x: &BigInt, y: &BigInt, coef_x: &BigInt, coef_y: &BigInt, gcd_xy: &BigInt) -> bool {
    coef_x * x + coef_y * y == *gcd_xy
}

pub fn mod_inv(x: &BigInt, modulus: &BigInt) -> Option<BigInt> {
    let result = extended_gcd(x, modulus);
    match result {
        None             => None,
        Some(gcd_result) => {
            match gcd_result.coef_x.sign() {
                Sign::Plus | Sign::NoSign => Some(gcd_result.coef_x),
                Sign::Minus => Some(modulus + gcd_result.coef_x), 
            }
        }
    }
}
