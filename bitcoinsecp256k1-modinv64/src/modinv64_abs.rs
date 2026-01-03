// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_abs.rs ]
crate::ix!();

/// Helper function to compute the absolute value of an int64_t. (we don't use abs/labs/llabs as it
/// depends on the int sizes).
/// 
#[cfg(VERIFY)]
pub fn modinv64_abs(v: i64) -> i64 {

    VERIFY_CHECK!(v > i64::MIN);
    if v < 0 { -v } else { v }
}

#[cfg(all(test, VERIFY))]
mod modinv64_abs_contract {
    use super::*;

    #[traced_test]
    fn abs_handles_zero_and_signs() {
        trace!(v = 0i64);
        assert!(modinv64_abs(0) == 0);

        trace!(v = 1i64);
        assert!(modinv64_abs(1) == 1);

        trace!(v = -1i64);
        assert!(modinv64_abs(-1) == 1);
    }

    #[traced_test]
    fn abs_handles_large_values() {
        let a = i64::MAX;
        let b = -(i64::MAX);
        trace!(a = a, b = b);
        assert!(modinv64_abs(a) == i64::MAX);
        assert!(modinv64_abs(b) == i64::MAX);
    }
}
