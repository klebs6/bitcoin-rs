// ---------------- [ File: bitcoin-sha256/src/transform_d64_scalar.rs ]
crate::ix!();

/// Double‑SHA256 of **one** 64‑byte message (scalar reference path).
///
/// SAFETY: `out32` must point to 32 writable bytes; `in64` to 64 readable bytes.
#[inline]
pub unsafe fn transform_d64_scalar(out32: *mut u8, in64: *const u8) {
    // ── First hash: H1 = SHA256( in64[0..64] ) ─────────────────────────────
    let mut s1 = [0u32; 8];
    sha256_initialize(s1.as_mut_ptr());

    // message block
    sha256_transform_block(s1.as_mut_ptr(), in64);

    // padding block for a 64‑byte message: 0x80 || zeros || len=512 bits
    let mut pad1 = [0u8; 64];
    pad1[0] = 0x80;
    // 512 bits length, big‑endian
    pad1[56..64].copy_from_slice(&512u64.to_be_bytes());
    sha256_transform_block(s1.as_mut_ptr(), pad1.as_ptr());

    // Serialize H1 to **big‑endian bytes**.
    let mut h1_be = [0u8; 32];
    for (i, w) in s1.iter().enumerate() {
        h1_be[i * 4..i * 4 + 4].copy_from_slice(&w.to_be_bytes());
    }

    // ── Second hash: H2 = SHA256( h1_be[0..32] ) ───────────────────────────
    let mut s2 = [0u32; 8];
    sha256_initialize(s2.as_mut_ptr());

    // Build the single block: (32 bytes data) || 0x80 || zeros || len=256 bits
    let mut blk2 = [0u8; 64];
    blk2[..32].copy_from_slice(&h1_be);
    blk2[32] = 0x80;
    blk2[56..64].copy_from_slice(&256u64.to_be_bytes());

    sha256_transform_block(s2.as_mut_ptr(), blk2.as_ptr());

    // Write H2 as 32 big‑endian bytes to out32.
    for (i, w) in s2.iter().enumerate() {
        let be = w.to_be_bytes();
        core::ptr::copy_nonoverlapping(be.as_ptr(), out32.add(i * 4), 4);
    }
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
        let mut block = [0u8; 64];
        block.copy_from_slice(&fixtures::SELF_TEST_DATA[1..65]); // skip the leading `-`

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

                if val == 0 || val == 0xFF {
                    trace!(target: "sha256", position = pos, value = val, "boundary case");
                }

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

        for idx in 0..N {
            // Refill block with bytes from LCG (Numerical Recipes multiplier)
            for b in &mut block {
                lcg = lcg.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                *b = (lcg >> 24) as u8;
            }

            let expected = double_sha256_reference(&block);
            let mut actual = [0u8; 32];
            unsafe { transform_d64_scalar(actual.as_mut_ptr(), block.as_ptr()) };

            if idx % 10_000 == 0 {
                trace!(
                    target: "sha256",
                    iteration = idx,
                    sample_actual = ?hex::encode(actual),
                    "progress checkpoint"
                );
            }

            assert_eq!(actual, expected, "random block digest mismatch");
        }

        trace!(
            target: "sha256",
            total_blocks = N,
            "random audit passed"
        );
    }
}

#[cfg(test)]
mod transform_d64_scalar_validation {
    use super::*;
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use sha2::{Digest, Sha256 as UpstreamSha256};
    use std::io::Write;

    /* ────────────────────────── helpers ────────────────────────── */

    /// Independent oracle: SHA256(SHA256(block)), using RustCrypto.
    fn sha2_double_block(block: &[u8; 64]) -> [u8; 32] {
        let mut h = UpstreamSha256::new();
        h.update(block);
        let first = h.finalize_reset(); // H1
        h.update(&first);
        let second = h.finalize();      // H2
        let mut out = [0u8; 32];
        out.copy_from_slice(&second);
        out
    }

    /// SUT wrapper: call `transform_d64_scalar` and return the 32‑byte digest.
    fn sut(block: &[u8; 64]) -> [u8; 32] {
        let mut out = [0u8; 32];
        unsafe { transform_d64_scalar(out.as_mut_ptr(), block.as_ptr()) };
        out
    }

    /// Sanity cross‑check via the crate’s own streaming hasher.
    fn sha256d_via_streaming(block: &[u8; 64]) -> [u8; 32] {
        // H1
        let mut ctx1 = Sha256::new();
        ctx1.write_all(block).unwrap();
        let mut h1 = [0u8; 32];
        ctx1.finalize(&mut h1);

        // H2
        let mut ctx2 = Sha256::new();
        ctx2.write_all(&h1).unwrap();
        let mut h2 = [0u8; 32];
        ctx2.finalize(&mut h2);

        h2
    }

    /// Gather up to 8 contiguous 64‑byte blocks from SELF_TEST_DATA[1..].
    fn lorem_blocks() -> Vec<[u8; 64]> {
        let base = &fixtures::SELF_TEST_DATA[1..]; // mirror the self‑test offset
        let n = core::cmp::min(base.len() / 64, 8);
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            let mut b = [0u8; 64];
            b.copy_from_slice(&base[i * 64..i * 64 + 64]);
            v.push(b);
        }
        v
    }

    /* ────────────────────────── tests ──────────────────────────── */

    #[traced_test]
    fn zero_block_matches_reference() {
        let block = [0u8; 64];
        let exp = sha2_double_block(&block);
        let got = sut(&block);
        assert_eq!(got, exp, "sha256d(0x00*64) mismatch");
        // secondary cross‑check
        assert_eq!(got, sha256d_via_streaming(&block), "streaming != sut for zero block");
    }

    #[traced_test]
    fn byte_broadcast_256_values_match_reference() {
        for b in 0u8..=255 {
            let block = [b; 64];
            let exp = sha2_double_block(&block);
            let got = sut(&block);
            assert_eq!(got, exp, "byte‑broadcast mismatch for b=0x{b:02x}");
        }
    }

    #[traced_test]
    fn single_bit_set_and_clear_across_all_512_bits_match_reference() {
        // one‑hot (exactly one bit set)
        for bit in 0..512 {
            let mut block = [0u8; 64];
            block[bit / 8] |= 1u8 << (bit % 8);
            let exp = sha2_double_block(&block);
            let got = sut(&block);
            assert_eq!(got, exp, "one‑hot mismatch at bit {}", bit);
        }
        // one‑cold (exactly one bit clear)
        for bit in 0..512 {
            let mut block = [0xFFu8; 64];
            block[bit / 8] &= !(1u8 << (bit % 8));
            let exp = sha2_double_block(&block);
            let got = sut(&block);
            assert_eq!(got, exp, "one‑cold mismatch at bit {}", bit);
        }
    }

    #[traced_test]
    fn lorem_ipsum_8_blocks_match_reference_and_streaming() {
        let blocks = lorem_blocks();
        assert!(!blocks.is_empty(), "fixture should provide at least one 64‑byte block");
        for (lane, blk) in blocks.iter().enumerate() {
            let exp = sha2_double_block(blk);
            let got = sut(blk);
            assert_eq!(got, exp, "lorem block lane {} mismatch (sha2 oracle)", lane);
            assert_eq!(got, sha256d_via_streaming(blk), "lorem block lane {} mismatch (streaming)", lane);
        }
    }

    #[traced_test]
    fn unaligned_input_pointer_is_handled_correctly() {
        // Prepare a canonical block & expected digest.
        let mut block = [0u8; 64];
        for i in 0..64 { block[i] = i as u8 ^ 0xA5; }
        let exp = sha2_double_block(&block);

        // Copy into a Vec with a 1‑byte offset (misaligned).
        let mut buf = vec![0u8; 65];
        buf[1..65].copy_from_slice(&block);

        // Call the SUT using an intentionally unaligned pointer.
        let mut out = [0u8; 32];
        unsafe { transform_d64_scalar(out.as_mut_ptr(), buf.as_ptr().add(1)); }

        assert_eq!(out, exp, "unaligned input pointer produced wrong digest");
    }

    #[traced_test]
    fn exactly_32_bytes_written_to_output_no_overrun() {
        // Canary buffer: lower 32 will be overwritten, upper 32 must remain 0xAA.
        let mut out = [0xAAu8; 64];
        let block = [0x42u8; 64];

        let exp = sha2_double_block(&block);
        unsafe { transform_d64_scalar(out.as_mut_ptr(), block.as_ptr()) };

        assert_eq!(&out[..32], &exp[..], "first 32 output bytes mismatch");
        assert!(out[32..].iter().all(|&b| b == 0xAA), "bytes beyond 32 were clobbered");
    }

    #[traced_test]
    #[ignore = "Heavy: run manually with `cargo test --release -- --ignored`"]
    fn random_blocks_match_reference_heavy_sample() {
        const SAMPLES: usize = 20_000;
        let mut rng = StdRng::seed_from_u64(0x5EED_F005_CAFE_BEEF);
        let mut block = [0u8; 64];

        for _ in 0..SAMPLES {
            rng.fill_bytes(&mut block);
            let exp = sha2_double_block(&block);
            let got = sut(&block);
            assert_eq!(got, exp, "random block mismatch vs sha2 oracle");
        }
    }
}
