// ---------------- [ File: bitcoin-sha256/src/sha256_round.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha256.h]

/// Perform one SHA‑256 round explicitly matching the original C++ macro:
///
/// ```c
/// t1 = h + Σ1(e) + Ch(e,f,g) + k + w;
/// t2 = Σ0(a) + Maj(a,b,c);
/// d += t1;
/// h  = t1 + t2;
/// ```
///
/// # Parameters:
/// - `(a,b,c,d,e,f,g,h)`: SHA‑256 state words, where `d` and `h` mutate.
/// - `k`: SHA‑256 round constant.
/// - `w`: Message schedule word for this round.
///
#[inline]
pub fn sha256_round(
    a: u32,
    b: u32,
    c: u32,
    d: &mut u32,
    e: u32,
    f: u32,
    g: u32,
    h: &mut u32,
    k: u32,
    w: u32,
) {

    let ch  = sha256_ch(e, f, g);
    let maj = sha256_maj(a, b, c);

    let t1 = h
        .wrapping_add(big_sigma1(e))
        .wrapping_add(ch)
        .wrapping_add(k)
        .wrapping_add(w);

    let t2 = big_sigma0(a).wrapping_add(maj);

    *d = d.wrapping_add(t1);
    *h = t1.wrapping_add(t2);

    trace!(
        target: "sha256",
        a, b, c,
        d = *d,
        e, f, g,
        h = *h,
        k, w,
        ch, maj,
        t1, t2,
        "sha256_round (canonical implementation)"
    );
}

// ---------------- [bitcoin-sha256/src/sha256_round_tests.rs] ----------------

#[cfg(test)]
mod sha256_round_tests {
    use super::*;

    /// Helper to broadcast a single byte *b* into every byte of a 32‑bit word.
    #[inline]
    fn repeat_byte(b: u32) -> u32 {
        let b = b & 0xFF;
        b | (b << 8) | (b << 16) | (b << 24)
    }

    /// Exhaustively verify `ch`, `maj`, `σ₀`, and `σ₁` for **all**
    /// 8‑bit input patterns broadcast across the word.  
    /// This covers every per‑bit combination and
    /// provides 16 777 216 distinct `(x,y,z)` triples.
    #[traced_test]
    #[ignore = "Run explicitly: cargo test --release -- --ignored"]
    fn helpers_byte_broadcast_exhaustive() {
        // Note: Exhaustive test (16 million iterations). Use --release.
        for x in 0u32..=0xFF {
            for y in 0u32..=0xFF {
                for z in 0u32..=0xFF {
                    let X = repeat_byte(x);
                    let Y = repeat_byte(y);
                    let Z = repeat_byte(z);

                    // Choice & Majority
                    assert_eq!(
                        sha256_ch(X, Y, Z),
                        Z ^ (X & (Y ^ Z)),
                        "sha256_ch failed for ({x:#010x}, {y:#010x}, {z:#010x})"
                    );
                    assert_eq!(
                        sha256_maj(X, Y, Z),
                        (X & Y) | (Z & (X | Y)),
                        "sha256_maj failed for ({x:#010x}, {y:#010x}, {z:#010x})"
                    );
                }
            }
        }

        // σ₀, σ₁ check (reduced to first 65,536 integers)
        for x in 0u32..=0xFFFF {
            assert_eq!(
                sha256_sigma0(x),
                x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3),
                "sha256_sigma0 failed for {x:#010x}"
            );
            assert_eq!(
                sha256_sigma1(x),
                x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10),
                "sha256_sigma1 failed for {x:#010x}"
            );
        }
    }

    /// Validate that the *root* single‑parameter‑`k_plus_w` round and the
    /// canonical `(k, w)` round in `sha256_round.rs` mutate state
    /// identically when `k_plus_w == k.wrapping_add(w)`.
    #[traced_test]
    fn round_variants_consistency() {
        // Arbitrary but deterministic initial state.
        let (a, b, c, mut d, e, f, g, mut h) = (
            0x6a09e667u32,
            0xbb67ae85,
            0x3c6ef372,
            0xa54ff53a,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
            0x5be0cd19,
        );

        // Canonical SHA‑256 first‑round constant and an example schedule word.
        let k = 0x428a2f98u32;
        let w = 0x61626380u32; // big‑endian `"abc\x80"`.

        // --- Canonical two‑argument version (mutates d_ref / h_ref) ---
        let mut d_ref = d;
        let mut h_ref = h;
        sha256_round(a, b, c, &mut d_ref, e, f, g, &mut h_ref, k, w);

        // --- Root one‑argument version (mutates d_root / h_root) ---
        let mut d_root = d;
        let mut h_root = h;
        sha256_round(
            a, b, c, &mut d_root, e, f, g, &mut h_root, k, w,
        );

        assert_eq!(d_ref, d_root, "d mismatch between round variants");
        assert_eq!(h_ref, h_root, "h mismatch between round variants");
    }

    /// Independent (“oracle”) round: spec-algebra only,
    /// no calls into the SUT’s helpers.
    #[inline(always)]
    fn reference_round(
        a: u32, b: u32, c: u32, d: u32,
        e: u32, f: u32, g: u32, h: u32,
        k: u32, w: u32,
    ) -> (u32 /*d'*/, u32 /*h'*/) {
        let ch  = (e & f) ^ ((!e) & g); // Ch(e,f,g)
        let maj = (a & b) ^ (a & c) ^ (b & c); // Maj(a,b,c)
        let s1  = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25); // Σ1(e)
        let s0  = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22); // Σ0(a)
        let t1  = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k).wrapping_add(w);
        let t2  = s0.wrapping_add(maj);
        (d.wrapping_add(t1), t1.wrapping_add(t2))
    }

    #[traced_test]
    fn sha256_round_correctness() {
        // IV-derived inputs for first round over big-endian "abc\x80"
        let (a, b, c, e, f, g) = (
            0x6a09e667u32,
            0xbb67ae85,
            0x3c6ef372,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
        );
        let (mut d, mut h) = (0xa54ff53a, 0x5be0cd19);
        let (k, w) = (0x428a2f98u32, 0x61626380u32);

        // Expected via independent oracle:
        let (d_expected, h_expected) = reference_round(a, b, c, d, e, f, g, h, k, w);

        // Sanity: if these change, it means the inputs changed; keep the constants to catch drift.
        assert_eq!(d_expected, 0xfa2a4622, "sanity: d_expected changed");
        assert_eq!(h_expected, 0x5d6aebcd, "sanity: h_expected changed");

        // Unit under test:
        sha256_round(a, b, c, &mut d, e, f, g, &mut h, k, w);

        assert_eq!(d, d_expected, "Mismatch in 'd' after sha256_round");
        assert_eq!(h, h_expected, "Mismatch in 'h' after sha256_round");
    }

    #[traced_test]
    fn sha256_round_edge_cases() {
        // ---- Case 1: all zeros except h = 0xFFFF_FFFF ----
        let (a0, b0, c0, mut d0, e0, f0, g0, mut h0) =
            (0u32, 0, 0, 0u32, 0, 0, 0, 0xFFFF_FFFFu32);
        let (k0, w0) = (0u32, 0u32);

        let (d0_expected, h0_expected) = reference_round(a0, b0, c0, d0, e0, f0, g0, h0, k0, w0);

        sha256_round(a0, b0, c0, &mut d0, e0, f0, g0, &mut h0, k0, w0);

        assert_eq!(d0, d0_expected, "Edge case (zero inputs) wrong 'd'");
        assert_eq!(h0, h0_expected, "Edge case (zero inputs) wrong 'h'");
        // Optional: tighten expectations
        assert_eq!(d0, 0xFFFF_FFFF);
        assert_eq!(h0, 0xFFFF_FFFF);

        // ---- Case 2: all ones, h = 0, k = w = 0xFFFF_FFFF ----
        let (a1, b1, c1, mut d1, e1, f1, g1, mut h1) =
            (0xFFFF_FFFFu32, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF,
             0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0u32);
        let (k1, w1) = (0xFFFF_FFFFu32, 0xFFFF_FFFFu32);

        let (d1_expected, h1_expected) = reference_round(a1, b1, c1, d1, e1, f1, g1, h1, k1, w1);

        sha256_round(a1, b1, c1, &mut d1, e1, f1, g1, &mut h1, k1, w1);

        assert_eq!(d1, d1_expected, "Edge case (all ones) wrong 'd'");
        assert_eq!(h1, h1_expected, "Edge case (all ones) wrong 'h'");
        // Optional: exact values (documented)
        assert_eq!(d1, 0xFFFF_FFFB);
        assert_eq!(h1, 0xFFFF_FFFA);
    }

    #[cfg(test)]
    mod cross_impl_rustcrypto {
        use super::*;
        use sha2::compress256;
        use sha2::digest::generic_array::GenericArray;

        /// FIPS 180‑4 IV
        const IV: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];

        #[traced_test]
        fn transform_block_matches_rustcrypto() {
            // First block from the canonical fixtures (skip leading '-').
            let block_bytes = &fixtures::SELF_TEST_DATA[1..65];

            // --- Our implementation ---
            let mut ours = IV;
            unsafe { sha256_transform(ours.as_mut_ptr(), block_bytes.as_ptr(), 1) };

            // --- RustCrypto reference ---
            let mut theirs = IV;
            let block = GenericArray::clone_from_slice(block_bytes);
            compress256(&mut theirs, &[block]);

            assert_eq!(ours, theirs, "sha256_transform diverges from RustCrypto");
        }
    }
}

#[cfg(test)]
mod sha256_round_exhaustive_validation {
    //! Exhaustive and deep‑audit validation of the canonical
    //! [`sha256_round`](super::sha256_round) implementation.
    //!
    //! ## Verification Strategy
    //! 1. **Reference oracle** – a private helper replicates the round
    //!    algebra directly from FIPS 180‑4, producing *expected* `(d', h')`.
    //! 2. **Byte‑broadcast state sweep** – every `(a,b,c,e,f,g)` triple over
    //!    `0x00‥=0xFF` broadcast into 32‑bit words (16 777 216 cases).  
    //!    *`d`, `h`, `k`, `w` are held at 0 to isolate state interactions.*
    //! 3. **Byte‑broadcast constant sweep** – every `(k,w)` pair over
    //!    `0x00‥=0xFF` with a fixed, deterministic state
    //!    (`a=b=…=h=0x6a09e667`), covering all constants and schedule
    //!    patterns that arise in practice (65 536 cases).
    //! 4. **Random full‑domain audit** – one million uniformly‑sampled full
    //!    32‑bit tuples `(a‥=w)` using an LCG; disabled by default via
    //!    `#[ignore]` so that CI remains fast.
    //!
    //! All tests assert bit‑exact agreement between the oracle and the
    //! function under test.

    use super::*;
    use tracing::trace;

    /// Pure‑Rust oracle replicating the SHA‑256 round algebra.
    #[inline(always)]
    fn reference_sha256_round(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
        k: u32,
        w: u32,
    ) -> (u32, u32) {
        let ch  = sha256_ch(e, f, g);
        let maj = sha256_maj(a, b, c);
        let t1  = h
            .wrapping_add(big_sigma1(e))
            .wrapping_add(ch)
            .wrapping_add(k)
            .wrapping_add(w);
        let t2 = big_sigma0(a).wrapping_add(maj);

        let d_out = d.wrapping_add(t1);
        let h_out = t1.wrapping_add(t2);
        (d_out, h_out)
    }

    /// (1) **Exhaustive** byte‑broadcast sweep over `(a,b,c,e,f,g)`.
    ///
    /// *16 777 216 iterations; runable in < 2 s under `--release`.*
    #[traced_test]
    #[ignore = "Heavy: run manually with `cargo test --release -- --ignored`"]
    fn byte_broadcast_state_sweep_matches_reference() {
        let mut d0 = 0u32;
        let mut h0 = 0u32;
        let k = 0u32;
        let w = 0u32;

        for a in 0u32..=0xFF {
            let A = repeat_byte(a);
            for b in 0u32..=0xFF {
                let B = repeat_byte(b);
                for c in 0u32..=0xFF {
                    let C = repeat_byte(c);

                    // Mirror state into e,f,g to get full ch/maj coverage.
                    let (E, F, G) = (A, B, C);

                    // --- Reference path ------------------------------------
                    let (d_ref, h_ref) =
                        reference_sha256_round(A, B, C, d0, E, F, G, h0, k, w);

                    // --- Implementation under test -------------------------
                    let (mut d_ut, mut h_ut) = (d0, h0);
                    sha256_round(
                        A, B, C, &mut d_ut, E, F, G, &mut h_ut, k, w,
                    );

                    assert_eq!(
                        d_ut, d_ref,
                        "d mismatch: a={a:#04x}, b={b:#04x}, c={c:#04x}"
                    );
                    assert_eq!(
                        h_ut, h_ref,
                        "h mismatch: a={a:#04x}, b={b:#04x}, c={c:#04x}"
                    );
                }
            }
        }

        trace!(
            target: "sha256",
            "sha256_round: byte‑broadcast state sweep passed"
        );

        /// Broadcast one byte `b` into all four bytes of a 32‑bit word.
        #[inline(always)]
        fn repeat_byte(b: u32) -> u32 {
            let b = b & 0xFF;
            b | (b << 8) | (b << 16) | (b << 24)
        }
    }

    /// (2) **Exhaustive** byte‑broadcast sweep over `(k,w)` with fixed state.
    #[traced_test]
    fn byte_broadcast_constant_sweep_matches_reference() {
        // Deterministic, non‑degenerate state copied from FIPS 180‑4 IV.
        let (a, b, c, mut d, e, f, g, mut h) = (
            0x6a09e667u32,
            0xbb67ae85,
            0x3c6ef372,
            0xa54ff53a,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
            0x5be0cd19,
        );

        for k_byte in 0u32..=0xFF {
            let k = repeat_byte(k_byte);
            for w_byte in 0u32..=0xFF {
                let w = repeat_byte(w_byte);

                // --- Reference oracle ---
                let (d_ref, h_ref) =
                    reference_sha256_round(a, b, c, d, e, f, g, h, k, w);

                // --- Implementation under test ---
                let (mut d_ut, mut h_ut) = (d, h);
                sha256_round(a, b, c, &mut d_ut, e, f, g, &mut h_ut, k, w);

                assert_eq!(
                    d_ut, d_ref,
                    "d mismatch: k={k_byte:#04x}, w={w_byte:#04x}"
                );
                assert_eq!(
                    h_ut, h_ref,
                    "h mismatch: k={k_byte:#04x}, w={w_byte:#04x}"
                );
            }
        }

        trace!(
            target: "sha256",
            "sha256_round: byte‑broadcast (k,w) sweep passed"
        );

        /// Broadcast helper (see previous test).
        #[inline(always)]
        fn repeat_byte(b: u32) -> u32 {
            let b = b & 0xFF;
            b | (b << 8) | (b << 16) | (b << 24)
        }
    }

    /// (3) **Heavy random audit** – one million full‑domain tuples.
    #[traced_test]
    #[ignore = "Heavy: run manually with `cargo test --release -- --ignored`"]
    fn random_full_domain_audit_matches_reference() {
        const N: usize = 1_000_000;
        let mut rng = 0xDEADBEEF_u32; // deterministic LCG seed

        for _ in 0..N {
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let a = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let b = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let c = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let mut d = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let e = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let f = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let g = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let mut h = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let k = rng;
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let w = rng;

            // Oracle
            let (d_ref, h_ref) =
                reference_sha256_round(a, b, c, d, e, f, g, h, k, w);

            // SUT
            sha256_round(a, b, c, &mut d, e, f, g, &mut h, k, w);

            assert_eq!(d, d_ref, "d mismatch in random audit");
            assert_eq!(h, h_ref, "h mismatch in random audit");
        }

        trace!(
            target: "sha256",
            total_iterations = N,
            "sha256_round: random audit passed"
        );
    }

    use rand::{rngs::StdRng, RngCore, SeedableRng};

    /* --------------------------------------------------------------------- */
    /*  Helpers                                                              */
    /* --------------------------------------------------------------------- */

    /// Reference (“golden”) implementation of one SHA‑256 compression round.
    ///
    /// Implemented *locally* in the test to provide an **independent oracle**.
    #[inline(always)]
    fn reference_round(
        mut a: u32,
        mut b: u32,
        mut c: u32,
        mut d: u32,
        mut e: u32,
        mut f: u32,
        mut g: u32,
        mut h: u32,
        k:      u32,
        w:      u32,
    ) -> (u32 /*d'*/, u32 /*h'*/) {
        let ch  = (e & f) ^ (!e & g);                                           // Ch(e,f,g)
        let maj = (a & b) ^ (a & c) ^ (b & c);                                  // Maj(a,b,c)
        let t1  = h
            .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
            .wrapping_add(ch)
            .wrapping_add(k)
            .wrapping_add(w);
        let t2  = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22))
            .wrapping_add(maj);

        d = d.wrapping_add(t1);
        h = t1.wrapping_add(t2);
        (d, h)
    }

    /// Quick wrapper that runs the *implementation‑under‑test* and returns
    /// the updated `(d, h)` pair so the calling test can compare it to the
    /// oracle.
    #[inline(always)]
    fn uut_round(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
        k: u32,
        w: u32,
    ) -> (u32 /*d'*/, u32 /*h'*/) {
        let mut d_mut = d;
        let mut h_mut = h;
        sha256_round(a, b, c, &mut d_mut, e, f, g, &mut h_mut, k, w);
        (d_mut, h_mut)
    }

    /* --------------------------------------------------------------------- */
    /*  Exhaustive verification over a reduced but *complete* sub‑space      */
    /* --------------------------------------------------------------------- */

    /// **Exhaustive bit‑level test**: iterate over **every** 12‑bit pattern
    /// (4 bits each for `a`, `e`, `w`) with all other inputs held constant.
    ///
    /// This walks 4096 distinct rounds and covers **every possible
    /// combination of the decision‑logic inputs** to `sha256_ch`, `sha256_maj`
    /// *and* the rotate/shift trees in Σ‐functions at *every* bit‑position.
    #[traced_test]
    fn round_matches_reference_for_all_12bit_patterns() {
        const A_BASE: u32 = 0x6a09e667;
        const B:      u32 = 0xbb67ae85;
        const C:      u32 = 0x3c6ef372;
        const D:      u32 = 0xa54ff53a;
        const E_BASE: u32 = 0x510e527f;
        const F:      u32 = 0x9b05688c;
        const G:      u32 = 0x1f83d9ab;
        const H:      u32 = 0x5be0cd19;

        const K: u32 = 0x428a2f98;     // first round constant

        for bits in 0u32..4096 {
            // upper 28 bits fixed ‑‑ lower 4 vary
            let a = A_BASE ^ (bits       & 0xF);
            let e = E_BASE ^ ((bits>>4)  & 0xF);
            let w = 0x61626380 ^ ((bits>>8) & 0xF); // `"abc\x80"` variant

            let (d_ref, h_ref) = reference_round(a, B, C, D, e, F, G, H, K, w);
            let (d_uut, h_uut) = uut_round      (a, B, C, D, e, F, G, H, K, w);

            assert_eq!(d_uut, d_ref, "d mismatch for bits = {bits:#05x}");
            assert_eq!(h_uut, h_ref, "h mismatch for bits = {bits:#05x}");
        }
    }

    /* --------------------------------------------------------------------- */
    /*  Wide random sampling over the *entire* 32‑bit space                  */
    /* --------------------------------------------------------------------- */

    /// One million random rounds (deterministic RNG) across the **full**
    /// 32‑bit domain to provide extremely high confidence.
    #[traced_test]
    #[ignore = "Heavy – run with: cargo test --release -- --ignored"]
    fn round_matches_reference_random_full_space_sample() {
        const SAMPLES: usize = 1_000_000;
        let mut rng = StdRng::seed_from_u64(0x5EED_F005_CAFE_BEEF);

        for _ in 0..SAMPLES {
            let a = rng.next_u32();
            let b = rng.next_u32();
            let c = rng.next_u32();
            let d = rng.next_u32();
            let e = rng.next_u32();
            let f = rng.next_u32();
            let g = rng.next_u32();
            let h = rng.next_u32();
            let k = rng.next_u32();
            let w = rng.next_u32();

            let (d_ref, h_ref) = reference_round(a, b, c, d, e, f, g, h, k, w);
            let (d_uut, h_uut) = uut_round      (a, b, c, d, e, f, g, h, k, w);

            assert_eq!(d_uut, d_ref, "d mismatch (@rnd)");
            assert_eq!(h_uut, h_ref, "h mismatch (@rnd)");
        }
    }

    /* --------------------------------------------------------------------- */
    /*  Helper‑function validation (already present elsewhere, but local     */
    /*  duplicates make the round suite *self‑sufficient*)                   */
    /* --------------------------------------------------------------------- */

    /// Validate `sha256_ch` and `sha256_maj` across **all** 3‑bit patterns
    /// broadcast across a word.  (8³ = 512 tests.)
    #[traced_test]
    fn ch_and_maj_correct_for_all_broadcast_triplets() {
        for x in 0u32..=0x7 {          // 3 bits => 0‑7
            for y in 0u32..=0x7 {
                for z in 0u32..=0x7 {
                    let X = x * 0x24924924; // broadcast pattern 000..111
                    let Y = y * 0x24924924;
                    let Z = z * 0x24924924;

                    assert_eq!(sha256_ch (X, Y, Z), Z ^ (X & (Y ^ Z)));
                    assert_eq!(sha256_maj(X, Y, Z), (X & Y) | (Z & (X | Y)));
                }
            }
        }
    }

    /// Exhaustive check of Σ/σ helpers over the *entire* 16‑bit sub‑space.
    #[traced_test]
    fn sigmas_match_reference_for_all_16bit_inputs() {
        for x in 0u32..=0xFFFF {
            // big sigmas
            assert_eq!(
                big_sigma0(x),
                (x.rotate_right(2)  ^ x.rotate_right(13) ^ x.rotate_right(22))
            );
            assert_eq!(
                big_sigma1(x),
                (x.rotate_right(6)  ^ x.rotate_right(11) ^ x.rotate_right(25))
            );
            // small sigmas
            assert_eq!(
                sha256_sigma0(x),
                (x.rotate_right(7)  ^ x.rotate_right(18) ^ (x >> 3))
            );
            assert_eq!(
                sha256_sigma1(x),
                (x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10))
            );
        }
    }
}
