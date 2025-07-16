// ---------------- [ File: bitcoin-sha256/src/sha256_initialize.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/hash.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/hash_impl.h]

/**
  | Write the SHA‑256 IV (FIPS 180‑4 section 5.3.3) into the caller‑supplied
  | 8‑word state array *s*.
  |
  | # Safety
  | * `s` **must** point to **at least** eight valid `u32` values.
  | * The memory region referenced by `s` must be writable for the duration of
  |   the call.
  |
  | This routine is intentionally `unsafe` because it performs raw pointer
  | arithmetic.  The caller is responsible for upholding the above contract.
  |
  | Logging is performed at `TRACE` level under the `"sha256"` target so that
  | production builds can retain the calls with minimal overhead when the
  | `max_level_trace` feature is disabled.
  */
#[inline]
pub unsafe fn sha256_initialize(s: *mut u32) {
    const IV: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    for (i, &word) in IV.iter().enumerate() {
        // SAFETY: caller guarantees `s` has room for eight `u32`s.
        *s.add(i) = word;
    }
}

#[cfg(test)]
mod sha256_initialisation_tests {
    use super::*;

    /// Expected FIPS 180‑4 IV for SHA‑256, expressed in little‑endian host order.
    const IV: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    #[traced_test]
    fn default_constructor_sets_iv_and_zeros() {
        let ctx = Sha256::default();

        // State words must equal the IV.
        assert_eq!(*ctx.s(), IV, "state words do not match FIPS 180‑4 IV");

        // Buffer must start zero‑filled.
        assert!(ctx.buf().iter().all(|&b| b == 0), "buffer not zero‑initialised");

        // No bytes should have been processed yet.
        assert_eq!(*ctx.bytes(), 0, "byte counter not initialised to zero");
    }

    #[traced_test]
    fn pointer_initialiser_writes_correct_values() {
        let mut state = [0u32; 8];

        // SAFETY: `state.as_mut_ptr()` is valid for eight u32s.
        unsafe { sha256_initialize(state.as_mut_ptr()) };

        assert_eq!(state, IV, "sha256_initialize did not write canonical IV");
    }
}
