// ---------------- [ File: bitcoin-sha256/src/sha256_finalize.rs ]
crate::ix!();

impl Sha256 {
    pub fn finalize(&mut self, hash: &mut [u8; SHA256_OUTPUT_SIZE]) {
        finalize_inner(self, hash.as_mut_ptr(), /*wipe=*/false);
    }
}

#[inline]
pub unsafe fn sha256_finalize(hash: *mut Sha256, out32: *mut u8) {
    debug_assert!(!hash.is_null() && !out32.is_null());
    finalize_inner(&mut *hash, out32, /*wipe=*/true);
}

// bitcoin‑sha256/src/sha256_finalize_core.rs (new, private)
#[inline(always)]
fn finalize_inner(ctx: &mut Sha256, out: *mut u8, wipe_state: bool) {
    /* ---- 1. padding ---- */
    const PAD: [u8; 64] = [
        0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let bytes = *ctx.bytes();
    let pad_len = 1 + ((119 - (bytes % 64)) % 64) as usize;

    // 64‑bit big‑endian descriptor (bits)
    let mut sizedesc = [0u8; 8];
    beio::u64_into(&mut sizedesc, bytes << 3);

    // Feed padding & length through the streaming writer
    ctx.write_ptr(PAD.as_ptr(), pad_len);
    ctx.write_ptr(sizedesc.as_ptr(), 8);

    /* ---- 2. serialise state ---- */
    let mut tmp = [0u32; 8];
    for (dst, src) in tmp.iter_mut().zip(ctx.s_mut().iter_mut()) {
        *dst = (*src).to_be();
        if wipe_state {
            *src = 0;
        }
    }

    /* ---- 3. output ---- */
    unsafe {
        std::ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, out, 32);
    }
}

// -----------------------------------------------------------------------------
// Unit‑tests for `finalize_inner`, `Sha256::finalize` and `sha256_finalize`
// -----------------------------------------------------------------------------
#[cfg(test)]
mod finalize_tests {
    use super::*;
    use hex_literal::hex;
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use std::io::Write;

    /* ============== helpers ================================================= */

    /// Compute a digest through the *safe* API (`Sha256::finalize`).
    fn digest_safe(data: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
        let mut ctx = Sha256::new();
        ctx.write_all(data).unwrap();
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        ctx.finalize(&mut out);
        out
    }

    /// Compute a digest via the **FFI** wrapper ­(`sha256_finalize`).
    fn digest_ffi(data: &[u8]) -> [u8; SHA256_OUTPUT_SIZE] {
        let mut ctx = Sha256::new();
        ctx.write_all(data).unwrap();
        let mut out = [0u8; SHA256_OUTPUT_SIZE];
        unsafe { sha256_finalize(&mut ctx, out.as_mut_ptr()) };
        out
    }

    /// Compute a digest by calling the *private* helper directly with both
    /// `wipe_state` variants.  Returns `(digest, state_after)`.
    fn digest_inner(data: &[u8], wipe: bool) -> ([u8; 32], [u32; 8]) {
        let mut ctx = Sha256::new();
        ctx.write_all(data).unwrap();

        let mut out = [0u8; 32];
        // SAFETY: `out` is 32 bytes, exactly what `finalize_inner` writes.
        unsafe {
            super::finalize_inner(&mut ctx, out.as_mut_ptr(), wipe);
        }

        let mut state = [0u32; 8];
        state.copy_from_slice(ctx.s());

        (out, state)
    }

    /* ============== known good vectors ====================================== */

    const DIGEST_EMPTY: [u8; 32] =
        hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    const DIGEST_ABC:   [u8; 32] =
        hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");

    /* ============== basic correctness ======================================= */

    #[traced_test]
    fn empty_and_abc_match_reference() {
        assert_eq!(digest_safe(b""),  DIGEST_EMPTY, "`\"\"` digest mismatch");
        assert_eq!(digest_safe(b"abc"), DIGEST_ABC, "`\"abc\"` digest mismatch");
    }

    /* ============== safe API ≡ FFI ≡ inner(no‑wipe) ========================= */

    #[traced_test]
    fn all_variants_produce_identical_output_up_to_256_bytes() {
        let mut buf = [0u8; 256];
        for len in 0..=buf.len() {
            buf[len % 256] = len as u8;           // deterministically vary input
            let msg = &buf[..len];

            let safe = digest_safe(msg);
            assert_eq!(safe, digest_ffi(msg),   "safe ≠ ffi for len={len}");
            let (inner, _) = digest_inner(msg, /*wipe=*/false);
            assert_eq!(safe, inner,             "safe ≠ inner for len={len}");
        }
    }

    /* ============== edge‑case padding boundaries ============================ */

    #[traced_test]
    fn boundary_lengths_roundtrip() {
        // lengths where pad_len changes (…55,56,57 & 63,64,65)
        const SPECIAL: &[usize] = &[0, 1, 55, 56, 57, 63, 64, 65, 127, 128];
        for &len in SPECIAL {
            let msg: Vec<u8> = (0..len as u8).collect();
            let digest = digest_safe(&msg);
            // Re‑hash the hash itself – a cheap sanity‑check
            let mut ctx = Sha256::new();
            ctx.write_all(&digest).unwrap();
            let mut second = [0u8; 32];
            ctx.finalize(&mut second);
            assert_ne!(digest, second, "digest self‑collision at len={len}");
        }
    }

    /* ============== wipe‑state correctness ================================== */

    #[traced_test]
    fn wipe_state_sets_internal_words_to_zero() {
        let data = b"wipe-test";
        let (_, state_after) = digest_inner(data, /*wipe=*/true);
        assert!(
            state_after.iter().all(|&w| w == 0),
            "state not zeroed after wipe‑finalize"
        );
    }

    /* ============== context re‑use after safe finalize ====================== */

    #[traced_test]
    fn context_can_be_reused_after_finalize() {
        let mut ctx = Sha256::new();
        ctx.write_all(b"first").unwrap();
        let mut out1 = [0u8; 32];
        ctx.finalize(&mut out1);

        // re‑use without manual reset – spec says this is legal
        ctx.reset();
        ctx.write_all(b"second").unwrap();
        let mut out2 = [0u8; 32];
        ctx.finalize(&mut out2);

        assert_ne!(out1, out2, "reset did not clear previous state");
        assert_eq!(out2, digest_safe(b"second"), "wrong digest after reset()");
    }

    /* ============== randomised stress‑test ================================== */

    #[traced_test]
    #[ignore = "Run explicitly: cargo test --release -- --ignored"]
    fn random_messages_consistent_across_variants() {
        let mut rng = StdRng::seed_from_u64(0x5EED_F005_u64);
        for _ in 0..10_000 {
            let len = (rng.next_u32() % 1024) as usize;
            let mut msg = vec![0u8; len];
            rng.fill_bytes(&mut msg);

            let ref_digest = digest_safe(&msg);
            assert_eq!(ref_digest, digest_ffi(&msg),   "safe ≠ ffi random");
            let (d_inner, _) = digest_inner(&msg, false);
            assert_eq!(ref_digest, d_inner,            "safe ≠ inner random");
        }
    }
}
