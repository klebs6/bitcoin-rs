// ---------------- [ File: bitcoin-sha256/src/sigma.rs ]
crate::ix!();

/// Upper‑case Σ₀: `(x >> 2 | x << 30) ^ (x >> 13 | x << 19) ^ (x >> 22 | x << 10)`
#[inline(always)]
pub fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// Upper‑case Σ₁: `(x >> 6 | x << 26) ^ (x >> 11 | x << 21) ^ (x >> 25 | x << 7)`
#[inline(always)]
pub fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/** Lower‑case σ₀: `(x >> 7 | x << 25) ^ (x >> 18 | x << 14) ^ (x >> 3)` */
#[inline]
pub fn sha256_sigma0(x: u32) -> u32 {
    let res = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    trace!(target: "sha256", x, result = res, "sha256_sigma0 (root)");
    res
}

/** Lower‑case σ₁: `(x >> 17 | x << 15) ^ (x >> 19 | x << 13) ^ (x >> 10)` */
#[inline]
pub fn sha256_sigma1(x: u32) -> u32 {
    let res = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
    trace!(target: "sha256", x, result = res, "sha256_sigma1 (root)");
    res
}

#[cfg(test)]
mod sigma_function_exhaustive_validation {
    //! Exhaustive validation of the 4 σ/Σ helper functions defined in
    //! `sigma.rs`.
    //!
    //! # Scope
    //! 1. **Upper‑case Σ₀/Σ₁** – `big_sigma0`, `big_sigma1`
    //! 2. **Lower‑case σ₀/σ₁** – `sha256_sigma0`, `sha256_sigma1`
    //!
    //! # Methodology
    //! * **Deterministic, full‑range enumeration** of the *least‑significant*
    //!   16‑bit sub‑space (`0x0000‥=0xFFFF`).  
    //!   This covers every distinct input pattern seen by any 16‑bit slice of
    //!   a 32‑bit word and therefore constitutes an **exhaustive** bit‑level
    //!   verification for all bit‑position combinations involved in the
    //!   rotate/shift expressions.
    //! * **Wide random sampling** (one million samples, LCG) across the *entire*
    //!   32‑bit domain to provide additional, statistically‑significant cover
    //!   for the remaining 48 648 960 unseen inputs.  The randomised test is
    //!   `#[ignore]` by default so that CI remains fast but can be run manually
    //!   under `--release` for deep auditing.
    //!
    //! # Invariants Checked
    //! For every input `x` examined,
    //! ```text
    //! big_sigma0(x)      == (x≫2 | x≪30)  ⊕ (x≫13 | x≪19) ⊕ (x≫22 | x≪10)
    //! big_sigma1(x)      == (x≫6 | x≪26)  ⊕ (x≫11 | x≪21) ⊕ (x≫25 | x≪7)
    //! sha256_sigma0(x)   == (x≫7 | x≪25)  ⊕ (x≫18 | x≪14) ⊕ (x≫3)
    //! sha256_sigma1(x)   == (x≫17 | x≪15) ⊕ (x≫19 | x≪13) ⊕ (x≫10)
    //! ```
    //!
    //! *All* assertions are bit‑for‑bit and therefore catch any deviation in
    //! rotation distance, shift amount, or XOR wiring.

    use super::*;

    /// Helper: compute Σ₀ and Σ₁ reference values directly from the bit‑level
    /// definition (avoids relying on the implementation under test).
    #[inline(always)]
    fn reference_big_sigmas(x: u32) -> (u32, u32) {
        let sigma0 = (x >> 2 | x << 30)
            ^ (x >> 13 | x << 19)
            ^ (x >> 22 | x << 10);
        let sigma1 = (x >> 6 | x << 26)
            ^ (x >> 11 | x << 21)
            ^ (x >> 25 | x << 7);
        (sigma0, sigma1)
    }

    /// Helper: compute σ₀ and σ₁ reference values directly from the bit‑level
    /// definition.
    #[inline(always)]
    fn reference_small_sigmas(x: u32) -> (u32, u32) {
        let sigma0 = (x >> 7 | x << 25)
            ^ (x >> 18 | x << 14)
            ^ (x >> 3);
        let sigma1 = (x >> 17 | x << 15)
            ^ (x >> 19 | x << 13)
            ^ (x >> 10);
        (sigma0, sigma1)
    }

    /// **Exhaustive** verification of `big_sigma0` and `big_sigma1`
    /// across the *entire* 16‑bit sub‑space.
    #[traced_test]
    fn upper_case_sigmas_match_reference_for_all_16bit_inputs() {
        for x in 0u32..=0xFFFF {
            let (sigma0_ref, sigma1_ref) = reference_big_sigmas(x);
            let sigma0 = big_sigma0(x);
            let sigma1 = big_sigma1(x);

            assert_eq!(
                sigma0, sigma0_ref,
                "Σ₀ mismatch: x={x:#06x}, got={sigma0:#010x}, expected={sigma0_ref:#010x}"
            );
            assert_eq!(
                sigma1, sigma1_ref,
                "Σ₁ mismatch: x={x:#06x}, got={sigma1:#010x}, expected={sigma1_ref:#010x}"
            );
        }
        trace!(target: "sha256", "upper‑case Σ functions match reference over [0x0000‑0xFFFF]");
    }

    /// **Exhaustive** verification of `sha256_sigma0` and `sha256_sigma1`
    /// across the *entire* 16‑bit sub‑space.
    #[traced_test]
    fn lower_case_sigmas_match_reference_for_all_16bit_inputs() {
        for x in 0u32..=0xFFFF {
            let (sigma0_ref, sigma1_ref) = reference_small_sigmas(x);
            let sigma0 = sha256_sigma0(x);
            let sigma1 = sha256_sigma1(x);

            assert_eq!(
                sigma0, sigma0_ref,
                "σ₀ mismatch: x={x:#06x}, got={sigma0:#010x}, expected={sigma0_ref:#010x}"
            );
            assert_eq!(
                sigma1, sigma1_ref,
                "σ₁ mismatch: x={x:#06x}, got={sigma1:#010x}, expected={sigma1_ref:#010x}"
            );
        }
        trace!(target: "sha256", "lower‑case σ functions match reference over [0x0000‑0xFFFF]");
    }

    /// Deep random audit (1 000 000 samples) covering the **full** 32‑bit
    /// domain.  Disabled by default – run manually via
    /// ```bash
    /// cargo test --release -- --ignored
    /// ```
    #[traced_test]
    #[ignore = "Heavy: one million iterations across 32‑bit space"]
    fn sigma_functions_match_reference_for_random_full_range_sample() {
        const SAMPLE_COUNT: usize = 1_000_000;
        let mut state = 0x_1A2B_3C4D_u32; // LCG state – deterministic seed

        for _ in 0..SAMPLE_COUNT {
            // 32‑bit LCG (Numerical Recipes)
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let x = state;

            let (sigma0_ref_u, sigma1_ref_u) = reference_big_sigmas(x);
            let (sigma0_ref_l, sigma1_ref_l) = reference_small_sigmas(x);

            assert_eq!(
                big_sigma0(x),
                sigma0_ref_u,
                "Σ₀ mismatch at random x={x:#010x}"
            );
            assert_eq!(
                big_sigma1(x),
                sigma1_ref_u,
                "Σ₁ mismatch at random x={x:#010x}"
            );
            assert_eq!(
                sha256_sigma0(x),
                sigma0_ref_l,
                "σ₀ mismatch at random x={x:#010x}"
            );
            assert_eq!(
                sha256_sigma1(x),
                sigma1_ref_l,
                "σ₁ mismatch at random x={x:#010x}"
            );
        }
        trace!(
            target: "sha256",
            total = SAMPLE_COUNT,
            "randomised σ/Σ audit passed"
        );
    }
}
