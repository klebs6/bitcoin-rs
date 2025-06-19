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
