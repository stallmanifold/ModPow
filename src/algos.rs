use num::{Integer, Zero, One, BigInt};

/// 
/// Implementation of the binary extended gcd algorithm.
///
/// See Algorithm 14.61 of the 'Handbook of Applied Cryptography'.
///
pub fn extended_gcd(x: &BigInt, y: &BigInt) -> Option<(BigInt, BigInt, BigInt, BigInt)> {
    let zero: BigInt = Zero::zero();

    if (*x <= zero) || (*y <= zero) {
        return None;
    }

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
            return Some((c, d, g, v));
        }
    }
}