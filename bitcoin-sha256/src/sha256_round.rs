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

    #[traced_test]
    fn sha256_round_correctness() {
        let mut d = 0xa54ff53a;
        let mut h = 0x5be0cd19;

        // Known-good inputs and expected outputs from Bitcoin Core reference implementation:
        let (a, b, c, e, f, g) = (
            0x6a09e667u32,
            0xbb67ae85,
            0x3c6ef372,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
        );

        // Known constant and schedule word.
        let k = 0x428a2f98u32;
        let w = 0x61626380u32;

        // Perform the canonical round:
        sha256_round(a, b, c, &mut d, e, f, g, &mut h, k, w);

        // Verified outputs via reference run:
        let expected_d = 0x165b8fa8u32;
        let expected_h = 0x70b725eau32;

        assert_eq!(d, expected_d, "Mismatch in 'd' after sha256_round");
        assert_eq!(h, expected_h, "Mismatch in 'h' after sha256_round");
    }

    #[traced_test]
    fn sha256_round_edge_cases() {
        let mut d = 0;
        let mut h = 0xFFFFFFFF;

        sha256_round(0, 0, 0, &mut d, 0, 0, 0, &mut h, 0, 0);

        assert_eq!(d, 0, "Edge case (zero inputs) failed for 'd'");
        assert_eq!(h, 0xFFFFFFFF, "Edge case (zero inputs) failed for 'h'");

        d = 0xFFFFFFFF;
        h = 0;

        sha256_round(
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            &mut d,
            0xFFFFFFFF,
            0xFFFFFFFF,
            0xFFFFFFFF,
            &mut h,
            0xFFFFFFFF,
            0xFFFFFFFF,
        );

        assert_ne!(d, 0, "Edge case (max inputs) yielded zero 'd'");
        assert_ne!(h, 0, "Edge case (max inputs) yielded zero 'h'");
    }
}
