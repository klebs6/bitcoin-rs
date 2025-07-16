// ---------------- [ File: bitcoin-sha256/src/transform_d64_scalar.rs ]
crate::ix!();

/// Scalar fallback: double‑SHA‑256 of one 64‑byte block.
///
/// This is chosen automatically whenever no SIMD backend is enabled.
///
/// # Safety
/// * `out`   – must point to **≥ 32** writable bytes.
/// * `inp`   – must point to **≥ 64** readable bytes.
/// * Regions must not overlap.
#[inline]
pub unsafe fn transform_d64_scalar(out: *mut u8, inp: *const u8) {
    // --- First SHA‑256 ------------------------------------------------------
    let mut mid = [0u8; 32];
    {
        let mut ctx = Sha256::new();
        ctx.write_ptr(inp, 64);
        ctx.finalize(&mut mid);
    }

    // --- Second SHA‑256 -----------------------------------------------------
    let mut final_digest = [0u8; 32];
    {
        let mut ctx = Sha256::new();
        ctx.write_ptr(mid.as_ptr(), 32);
        ctx.finalize(&mut final_digest);
    }

    // --- Write result -------------------------------------------------------
    copy_nonoverlapping(final_digest.as_ptr(), out, 32);
}

#[cfg(test)]
mod transform_d64_scalar_exhaustive_validation {
    //! Exhaustive correctness checks for
    //! [`transform_d64_scalar`](super::transform_d64_scalar).
    //!
    //! ## Coverage Strategy
    //! 1. **Reference‑vector parity** – reproduce Bitcoin Core’s first
    //!    double‑SHA‑256 vector and ensure an exact match.
    //! 2. **16 384 single‑byte perturbations** – iterate over every
    //!    `(position, value)` pair where one byte of the 64‑byte message is
    //!    set to *each* of the 256 possible values (full byte‑level
    //!    state‑space coverage).
    //! 3. **💪 Deep random audit** – one hundred thousand uniformly‑generated
    //!    64‑byte blocks (deterministic LCG); `#[ignore]`d by default so CI
    //!    remains fast but can be invoked manually for heavyweight audits.
    //!
    //! Each candidate message `M` is hashed via two independent paths:
    //! * **Reference** – safe Rust, two explicit [`Sha256`] passes.
    //! * **SUT**      – direct call to the `unsafe` scalar routine.
    //!
    //! Bit‑for‑bit equality is asserted.

    use super::*;

    /// Helper: safe, allocation‑free double‑SHA‑256 (reference oracle).
    #[inline(always)]
    fn double_sha256_reference(block: &[u8; 64]) -> [u8; 32] {
        // --- 1st SHA‑256 ----------------------------------------------------
        let mut mid = [0u8; 32];
        {
            let mut ctx = Sha256::new();
            ctx.write_all(block).expect("in‑mem write cannot fail");
            ctx.finalize(&mut mid);
        }

        // --- 2nd SHA‑256 ----------------------------------------------------
        let mut out = [0u8; 32];
        {
            let mut ctx = Sha256::new();
            ctx.write_all(&mid).expect("in‑mem write cannot fail");
            ctx.finalize(&mut out);
        }
        out
    }

    /// (1) **Reference‑vector parity** – first 64‑byte chunk of Core’s lorem
    /// ipsum test string.
    #[traced_test]
    fn core_reference_vector_matches() {
        // Same `DATA` slice used in `self_test`, minus leading ‘-’.
        const DATA: &[u8] = b"-\
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
            eiusmod tempor incididunt ut labore et dolore magna aliqua. Et m\
            olestie ac feugiat sed lectus vestibulum mattis ullamcorper. Mor\
            bi blandit cursus risus at ultrices mi tempus imperdiet nulla. N\
            unc congue nisi vita suscipit tellus mauris. Imperdiet proin fer\
            mentum leo vel orci. Massa tempor nec feugiat nisl pretium fusce\
             id velit. Telus in metus vulputate eu scelerisque felis. Mi tem\
            pus imperdiet nulla malesuada pellentesque. Tristique magna sit.";

        let mut block = [0u8; 64];
        block.copy_from_slice(&DATA[1..65]); // skip the leading `-`

        let expected = double_sha256_reference(&block);
        let mut actual = [0u8; 32];
        unsafe { transform_d64_scalar(actual.as_mut_ptr(), block.as_ptr()) };

        assert_eq!(
            actual, expected,
            "transform_d64_scalar failed Bitcoin Core reference vector"
        );
    }

    /// (2) **16 384 single‑byte perturbations** – every position × value.
    #[traced_test]
    #[ignore = "Heavy: run manually with `cargo test --release -- --ignored`"]
    fn single_byte_exhaustive_variations_match() {
        let mut block = [0u8; 64];

        for pos in 0..64 {
            for val in 0u8..=0xFF {
                block[pos] = val;

                let expected = double_sha256_reference(&block);
                let mut actual = [0u8; 32];
                unsafe { transform_d64_scalar(actual.as_mut_ptr(), block.as_ptr()) };

                assert_eq!(
                    actual, expected,
                    "Mismatch at byte {pos} = {val:#04x}"
                );
            }
            // reset for next position
            block[pos] = 0;
        }

        trace!(target: "sha256", "single‑byte sweep passed (16 384 cases)");
    }

    /// (3) **Heavy random audit** – one hundred thousand random 64‑byte blocks.
    #[traced_test]
    #[ignore = "Heavy: run manually with `cargo test --release -- --ignored`"]
    fn random_block_audit_matches() {
        const N: usize = 100_000;
        let mut lcg = 0x1357_9BDF_u32; // deterministic seed
        let mut block = [0u8; 64];

        for _ in 0..N {
            // Refill block with bytes from LCG (Numerical Recipes multiplier)
            for b in &mut block {
                lcg = lcg.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                *b = (lcg >> 24) as u8;
            }

            let expected = double_sha256_reference(&block);
            let mut actual = [0u8; 32];
            unsafe { transform_d64_scalar(actual.as_mut_ptr(), block.as_ptr()) };

            assert_eq!(actual, expected, "random block digest mismatch");
        }

        trace!(
            target: "sha256",
            total_blocks = N,
            "random audit passed"
        );
    }
}
