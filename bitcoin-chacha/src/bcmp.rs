// ---------------- [ File: bitcoin-chacha/src/bcmp.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha_poly_aead.cpp]

/// Constant‑time byte‑wise comparison (returns 0 for equality, 1 otherwise).
#[cfg(not(HAVE_TIMINGSAFE_BCMP))]
pub fn timingsafe_bcmp(b1: *const u8, b2: *const u8, n: usize) -> i32 {
    trace!(len = n, "timingsafe_bcmp");
    if n == 0 {
        return 0;
    }

    // SAFETY: caller guarantees both pointers are valid for `n` bytes.
    let a = unsafe { core::slice::from_raw_parts(b1, n) };
    let b = unsafe { core::slice::from_raw_parts(b2, n) };

    let mut diff: u8 = 0;
    for i in 0..n {
        diff |= a[i] ^ b[i];
    }
    (diff != 0) as i32
}

#[cfg(test)]
mod bcmp_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn zero_length_returns_zero() {
        let res = timingsafe_bcmp(core::ptr::null(), core::ptr::null(), 0);
        assert_eq!(res, 0, "zero‑length compare must report equality");
    }

    #[traced_test]
    fn equal_buffers_return_zero() {
        const BUF: [u8; 32] = [42u8; 32];
        let r = timingsafe_bcmp(BUF.as_ptr(), BUF.as_ptr(), BUF.len());
        assert_eq!(r, 0, "identical input must compare equal");
    }

    #[traced_test]
    fn differing_buffers_return_one() {
        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        b[7] = 1; // single‑byte difference
        let r = timingsafe_bcmp(a.as_ptr(), b.as_ptr(), a.len());
        assert_eq!(r, 1, "different input must compare non‑equal");
    }
}
