use super::modpow::ModPow;
use num::BigInt;

#[test]
#[should_panic]
fn test_modpow_should_panic_with_zero_modulus() {
    let modulus  = BigInt::from(0);
    let exponent = BigInt::from(53);
    let base     = BigInt::from(11);

    let result   = <BigInt as ModPow>::mod_pow(&base, &exponent, &modulus);

    // mod_pow failed.
    assert!(false);
}
