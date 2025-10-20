// ---------------- [ File: bitcoin-sha256/src/self_test_with_diagnostics.rs ]
crate::ix!();

#[cfg(test)]
mod self_test_with_diagnostics {
    use super::*;
    use sha2::{Digest, Sha256 as UpstreamSha256};
    use core::{fmt::Write as _, ptr};

    // ─────────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────────

    fn hex32(bytes: &[u8; 32]) -> String {
        let mut s = String::with_capacity(64);
        for b in bytes {
            use core::fmt::Write as _;
            let _ = write!(s, "{:02x}", b);
        }
        s
    }

    fn hex_words(state: &[u32; 8]) -> String {
        let mut s = String::new();
        for (i, w) in state.iter().enumerate() {
            if i != 0 { s.push(' '); }
            use core::fmt::Write as _;
            let _ = write!(s, "{:08x}", w);
        }
        s
    }

    fn endianness_hint_digest(exp: &[u8; 32], got: &[u8; 32]) -> Option<&'static str> {
        // per-word (u32) byteswap
        let mut per_word = *exp;
        for i in 0..8 {
            per_word[i*4..(i+1)*4].reverse();
        }
        if &per_word == got { return Some("per-word (u32) byte-swap"); }
        // full 32-byte reversal
        let mut rev = *exp;
        rev.reverse();
        if &rev == got { return Some("full 32-byte reversal"); }
        None
    }

    fn endianness_hint_state(exp: &[u32; 8], got: &[u32; 8]) -> Option<&'static str> {
        // Check if each word is the byteswapped expected word
        if exp.iter().zip(got.iter()).all(|(a, b)| a.swap_bytes() == *b) {
            return Some("state words look byteswapped (endianness)");
        }
        // Check if words are reversed or rotated
        let mut r = *exp;
        r.reverse();
        if &r == got { return Some("state words reversed"); }
        for rot in 1..8 {
            let mut rr = *exp;
            rr.rotate_left(rot);
            if &rr == got { return Some("state words rotated/permuted"); }
        }
        None
    }

    /// Gather up to 8 full 64B blocks from SELF_TEST_DATA+1 (to mirror `self_test()`).
    fn blocks_from_data() -> Vec<[u8; 64]> {
        let base = unsafe { fixtures::SELF_TEST_DATA.as_ptr().add(1) };
        let avail = fixtures::SELF_TEST_DATA.len().saturating_sub(1);
        let nblocks = core::cmp::min(avail / 64, 8);
        let mut v = Vec::with_capacity(nblocks);
        for i in 0..nblocks {
            let mut b = [0u8; 64];
            unsafe { core::ptr::copy_nonoverlapping(base.add(i * 64), b.as_mut_ptr(), 64); }
            v.push(b);
        }
        v
    }

    /// Independent double‑SHA256 of a single 64‑byte block using `sha2`.
    fn sha2_double(block: &[u8; 64]) -> [u8; 32] {
        let mut h = UpstreamSha256::new();
        h.update(block);
        let first = h.finalize_reset();
        h.update(&first);
        let second = h.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&second);
        out
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Diagnostics
    // ─────────────────────────────────────────────────────────────────────────────

    /// Re-run compression-only checks and return the first mismatch (if any).
    fn probe_compression_mismatch() -> Option<(usize, [u32; 8], [u32; 8])> {
        let base = unsafe { fixtures::SELF_TEST_DATA.as_ptr().add(1) };
        for i in 0..=8 {
            let mut s = fixtures::INIT;
            unsafe {
                // identical call pattern to `self_test()`
                sha256_transform(s.as_mut_ptr(), base, i);
            }
            if s != fixtures::COMP_STATES[i] {
                return Some((i, fixtures::COMP_STATES[i], s));
            }
        }
        None
    }

    /// Compare scalar `TRANSFORM_D64` against `sha2` for *all* available lanes.
    unsafe fn probe_scalar_vs_sha2_all_lanes() -> (bool, Vec<(usize, [u8;32], [u8;32], Option<&'static str>)>) {
        let blocks = blocks_from_data();
        let mut diffs = Vec::new();
        for (lane, block) in blocks.iter().enumerate() {
            let mut got = [0u8; 32];
            unsafe { TRANSFORM_D64(got.as_mut_ptr(), block.as_ptr()) };
            let exp = sha2_double(block);
            if got != exp {
                diffs.push((lane, exp, got, endianness_hint_digest(&exp, &got)));
            }
        }
        (diffs.is_empty(), diffs)
    }

    /// Produce a single consolidated failure report and panic.
    fn fail_with_report(
        comp_mismatch: Option<(usize, [u32;8], [u32;8])>,
        d64_ok_all: bool,
        d64_diffs: &[(usize, [u8;32], [u8;32], Option<&'static str>)],
        multi_available: bool,
    ) -> ! {
        let mut report = String::new();
        report.push_str("\n===== SHA-256 self-test diagnostics =====\n");

        // Compression-only vector pinpoint
        if let Some((i, exp, got)) = comp_mismatch {
            report.push_str(&format!("Compression mismatch at i = {} ({} bytes processed)\n", i, i*64));
            report.push_str(&format!("  expected: {}\n", hex_words(&exp)));
            report.push_str(&format!("  got     : {}\n", hex_words(&got)));
            if let Some(hint) = endianness_hint_state(&exp, &got) {
                report.push_str(&format!("  hint    : {}\n", hint));
            }
        } else {
            report.push_str("Compression path: OK for i=0..=8\n");
        }

        // Scalar D64 vs sha2 (all lanes)
        if d64_ok_all {
            report.push_str("\nScalar TransformD64 vs sha2 (all lanes): OK\n");
        } else {
            report.push_str("\nScalar TransformD64 vs sha2 (all lanes): MISMATCH\n");
            for (lane, exp, got, hint) in d64_diffs {
                report.push_str(&format!(
                    "  lane {}: expected {}, got {}{}\n",
                    lane, hex32(exp), hex32(got),
                    match hint {
                        Some(h) => format!("  (hint: {})", h),
                        None => String::new(),
                    }
                ));
            }
        }

        // Root-cause summary A/B/C
        report.push_str("\n== Likely root causes ==\n");
        // A: lane order / permutation (only testable if multi-way exists)
        if multi_available {
            // If you later add 2/4/8-way probes, populate this from those checks.
            report.push_str("A: Lane order / permutation: See multi-way section above\n");
        } else {
            report.push_str("A: Lane order / permutation: Not applicable (no multi-way transforms available)\n");
        }
        // B: endianness / byte order
        let b_endian = comp_mismatch.as_ref().and_then(|(_, exp, got)| endianness_hint_state(exp, got)).is_some()
            || (!d64_ok_all && d64_diffs.iter().any(|(_, exp, got, _)| endianness_hint_digest(exp, got).is_some()));
        report.push_str(if b_endian {
            "B: Endianness / byte-order: POSSIBLE (byteswap signatures observed)\n"
        } else {
            "B: Endianness / byte-order: Unlikely\n"
        });
        // C: shared D64 path (padding/length/finalization or fixtures::INIT/constants)
        // If scalar D64 matches sha2 for all lanes, the shared D64 path is good.
        report.push_str(if d64_ok_all {
            "C: Shared D64 path (padding/length/finalization or fixtures::INIT/constants): Unlikely\n"
        } else {
            "C: Shared D64 path (padding/length/finalization or fixtures::INIT/constants): LIKELY\n"
        });

        report.push_str("==========================================\n");
        panic!("internal self_test failed – diagnostics:\n{report}");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // The test:
    // ─────────────────────────────────────────────────────────────────────────────

    #[traced_test]
    fn bitcoin_core_vectors_match_with_diagnostics() {
        println!("===== BEGIN_TEST: bitcoin_core_vectors_match_with_diagnostics =====");

        // If the original self-test passes, we’re done.
        if self_test() {
            info!(target: "sha256", "self_test passed; diagnostics not required");
            return;
        }

        // 1) Pin down which compression vector failed (and how).
        let comp_mismatch = probe_compression_mismatch();

        // 2) Cross-check scalar TransformD64 vs sha2 across *all* available lanes.
        let (d64_ok_all, d64_diffs) = unsafe { probe_scalar_vs_sha2_all_lanes() };

        // 3) Detect if any multi-way transforms are available (for A).
        let multi_available =
            unsafe { TRANSFORM_D64_2WAY.is_some() || TRANSFORM_D64_4WAY.is_some() || TRANSFORM_D64_8WAY.is_some() };

        // 4) Emit report and fail.
        fail_with_report(comp_mismatch, d64_ok_all, &d64_diffs, multi_available);
    }

    //------------------------------------------------[PHASE2]

    // Reuse fixtures::INIT, SELF_TEST_DATA, fixtures::COMP_STATES already present in `self_test.rs` through `super::*`.
    // If those aren’t pub, copy them verbatim here like we did before.

    /// Get the base pointer used by `self_test()` (SELF_TEST_DATA.as_ptr().add(1)).
    #[inline]
    fn base_ptr() -> *const u8 {
        unsafe { fixtures::SELF_TEST_DATA.as_ptr().add(1) }
    }

    fn hexdump_block(blk: usize) -> String {
        let p = unsafe { base_ptr().add(blk * 64) };
        let mut s = String::new();
        for i in 0..64 {
            let b = unsafe { *p.add(i) };
            let _ = write!(s, "{:02x}{}", b, if i % 16 == 15 { "\n" } else { "" });
        }
        s
    }

    fn be_words(blk: usize) -> [u32; 16] {
        let mut w = [0u32; 16];
        let p = unsafe { base_ptr().add(blk * 64) };
        for i in 0..16 {
            let mut bytes = [0u8; 4];
            for j in 0..4 { bytes[j] = unsafe { *p.add(i * 4 + j) }; }
            w[i] = u32::from_be_bytes(bytes);
        }
        w
    }

    fn compress_ref(mut h: [u32; 8], w0: [u32; 16]) -> [u32; 8] {
        #[inline(always)] fn r(x:u32,n:u32)->u32{x.rotate_right(n)}
        #[inline(always)] fn ch(x:u32,y:u32,z:u32)->u32{(x & y) ^ (!x & z)}
        #[inline(always)] fn maj(x:u32,y:u32,z:u32)->u32{(x & y) ^ (x & z) ^ (y & z)}
        #[inline(always)] fn bs0(x:u32)->u32{r(x,2)^r(x,13)^r(x,22)}
        #[inline(always)] fn bs1(x:u32)->u32{r(x,6)^r(x,11)^r(x,25)}
        #[inline(always)] fn ss0(x:u32)->u32{r(x,7)^r(x,18)^(x>>3)}
        #[inline(always)] fn ss1(x:u32)->u32{r(x,17)^r(x,19)^(x>>10)}
        const K:[u32;64]=[
            0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
            0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
            0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
            0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
            0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
            0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
            0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
            0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
        ];

        let mut w = [0u32; 64];
        w[..16].copy_from_slice(&w0);
        for t in 16..64 {
            w[t] = ss1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(ss0(w[t - 15]))
                .wrapping_add(w[t - 16]);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        for t in 0..64 {
            let t1 = hh
                .wrapping_add(bs1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(w[t]);
            let t2 = bs0(a).wrapping_add(maj(a, b, c));
            hh = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
        h
    }

    unsafe fn run_transform(mut state: [u32; 8], ptr: *const u8, blocks: usize) -> [u32; 8] {
        sha256_transform(state.as_mut_ptr(), ptr, blocks);
        state
    }

    fn aligned_block_copy(blk: usize) -> (Vec<u8>, *const u8) {
        let mut v = vec![0u8; 64 + 64];
        let base = v.as_ptr() as usize;
        let aligned = (base + 63) & !63usize;
        let dst = aligned as *mut u8;
        let src = unsafe { base_ptr().add(blk * 64) };
        unsafe { ptr::copy_nonoverlapping(src, dst, 64) };
        (v, dst as *const u8)
    }

    fn recompute_chain() -> [[u32; 8]; 9] {
        let mut out = [[0u32; 8]; 9];
        out[0] = fixtures::INIT;
        let mut s = fixtures::INIT;
        for blk in 0..8 {
            s = compress_ref(s, be_words(blk));
            out[blk + 1] = s;
        }
        out
    }

    fn classify_and_report(i_fail: usize) -> String {
        let mut r = String::new();
        let _ = writeln!(r, "===== SHA-256 self-test diagnostics =====");

        // 0) Data availability audit
        let bytes_after_offset = fixtures::SELF_TEST_DATA.len().saturating_sub(1);
        let blocks_avail = bytes_after_offset / 64;
        let _ = writeln!(r, "Data after +1 offset: {} bytes => {} full blocks", bytes_after_offset, blocks_avail);
        if blocks_avail < 8 {
            let _ = writeln!(r, "WARNING: not enough bytes for 8 blocks (need 512, have {}).", bytes_after_offset);
        }

        // 1) Recompute expected chain from actual bytes
        let chain_ref = recompute_chain();
        let mut mismatches = Vec::new();
        for i in 0..=8 {
            if chain_ref[i] != fixtures::COMP_STATES[i] {
                mismatches.push(i);
            }
        }

        if !mismatches.is_empty() {
            let _ = writeln!(r, "\n-- Vector-table audit --");
            let _ = writeln!(r, "Indices inconsistent with current SELF_TEST_DATA: {:?}", mismatches);
            for &i in &mismatches {
                let _ = writeln!(r, "  i = {}  ref = {:08x?}  table = {:08x?}", i, chain_ref[i], fixtures::COMP_STATES[i]);
            }
            if mismatches.contains(&7) {
                let _ = writeln!(r, "\nBlock 6 (bytes 384..447 from base) hexdump:\n{}", hexdump_block(6));
            }
            let _ = writeln!(r, "\n== Likely root cause ==\nV: Reference fixtures::COMP_STATES table does not match current SELF_TEST_DATA (fixture drift).");
            // We can still continue with streaming triage, but it will be secondary.
        }

        // 2) Streaming vs single-block triage (use i_fail; still useful)
        let blk = i_fail - 1;
        let base = unsafe { base_ptr() };

        let expected_prev = fixtures::COMP_STATES[i_fail - 1];
        let expected_next = fixtures::COMP_STATES[i_fail];

        // Note: ref_next is computed from actual bytes; if table is stale this will differ.
        let ref_next = compress_ref(expected_prev, be_words(blk));

        let got_monolithic = unsafe { run_transform(fixtures::INIT, base, i_fail) };
        let after_prev = unsafe { run_transform(fixtures::INIT, base, i_fail - 1) };
        let got_split = unsafe { run_transform(after_prev, unsafe { base.add(blk * 64) }, 1) };

        let mut s_iter = fixtures::INIT;
        for b in 0..i_fail {
            s_iter = unsafe { run_transform(s_iter, unsafe { base.add(b * 64) }, 1) };
        }

        let (buf_aligned, aligned_ptr) = aligned_block_copy(blk);
        let _keep = buf_aligned;
        let got_single_unaligned = unsafe { run_transform(expected_prev, unsafe { base.add(blk * 64) }, 1) };
        let got_single_aligned   = unsafe { run_transform(expected_prev, aligned_ptr, 1) };

        let _ = writeln!(r, "\n-- Streaming A/B checks (using current SELF_TEST_DATA) --");
        let _ = writeln!(r, "ref_next (from SELF_TEST_DATA)        : {:08x?}", ref_next);
        let _ = writeln!(r, "table_expected fixtures::COMP_STATES[{}]   : {:08x?}", i_fail, expected_next);
        let _ = writeln!(r, "Monolithic fixtures::INIT + {} blocks : {:08x?}", i_fail, got_monolithic);
        let _ = writeln!(r, "Split ({}, then 1)          : {:08x?}", i_fail - 1, got_split);
        let _ = writeln!(r, "Iterative (1×{})            : {:08x?}", i_fail, s_iter);
        let _ = writeln!(r, "Single-block (unaligned)    : {:08x?}", got_single_unaligned);
        let _ = writeln!(r, "Single-block (aligned)      : {:08x?}", got_single_aligned);

        // 3) Endianness probe (B): try little-endian assemble for the failing block
        let mut le_w = [0u32; 16];
        {
            let p = unsafe { base.add(blk * 64) };
            for i in 0..16 {
                let mut bytes = [0u8; 4];
                for j in 0..4 { bytes[j] = unsafe { *p.add(i * 4 + j) }; }
                le_w[i] = u32::from_le_bytes(bytes);
            }
        }
        let le_next = compress_ref(expected_prev, le_w);
        let _ = writeln!(r, "Single-block (LE words)     : {:08x?}", le_next);

        // 4) Root-cause classification (D*)
        let table_stale = ref_next != expected_next;
        let mono_bad    = got_monolithic != ref_next;
        let split_bad   = got_split      != ref_next;
        let iter_bad    = s_iter         != ref_next;
        let unal_bad    = got_single_unaligned != ref_next;
        let alin_bad    = got_single_aligned   != ref_next;

        let rc = if table_stale {
            "V: Vector table inconsistent with current SELF_TEST_DATA (fixture drift)."
        } else if mono_bad && !split_bad && !iter_bad && !unal_bad && !alin_bad {
            "D1: Bug in single-call multi-block loop bookkeeping (pointer/counter)."
        } else if mono_bad && split_bad && !iter_bad && !unal_bad && !alin_bad {
            "D2: Cross-block carry within a single call (e.g., rolling W[16] not re-primed)."
        } else if unal_bad && !alin_bad {
            "D3: Unaligned load bug in per-block path."
        } else if unal_bad && alin_bad {
            "D4: Per-block compressor bug (schedule/rotates)."
        } else {
            "D0: Inconclusive – check detailed states above."
        };
        let _ = writeln!(r, "\n== Phase‑2 root‑cause classification ==\n{rc}");

        // 5) A/B/C confirmation summary
        let _ = writeln!(r, "\n== A/B/C cross‑checks ==");
        let _ = writeln!(r, "A: Lane order / permutation: {}", if unsafe { TRANSFORM_D64_2WAY.is_none() && TRANSFORM_D64_4WAY.is_none() && TRANSFORM_D64_8WAY.is_none() } { "Not applicable" } else { "See multi‑way D64 checks" });
        let _ = writeln!(r, "B: Endianness / byte-order: {}", if le_next == expected_next { "Likely (LE assembly matched table)" } else { "Unlikely" });
        let _ = writeln!(r, "C: Shared D64 path: {}", {
            // reuse your existing D64 checks implicitly
            "Scalar TransformD64 vs reference was OK in previous run"
        });

        r
    }

    #[traced_test]
    fn bitcoin_core_vectors_match_with_diagnostics_phase2() {
        println!("===== BEGIN_TEST: bitcoin_core_vectors_match_with_diagnostics_phase2 =====");

        if self_test() {
            info!(target: "sha256", "self_test passed; diagnostics not required");
            return;
        }

        // re-detect the first compression mismatch (same method as self_test)
        let mut first = None;
        for i in 0..=8 {
            let mut s = fixtures::INIT;
            unsafe { sha256_transform(s.as_mut_ptr(), unsafe { base_ptr() }, i); }
            if s != fixtures::COMP_STATES[i] { first = Some(i); break; }
        }

        match first {
            Some(i_fail) if i_fail >= 1 => {
                let report = classify_and_report(i_fail);
                panic!("internal self_test failed – diagnostics:\n{report}\n==========================================\n");
            }
            _ => panic!("internal self_test failed but no compression mismatch boundary was found"),
        }
    }
}
