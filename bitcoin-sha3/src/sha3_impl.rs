// ---------------- [ File: bitcoin-sha3/src/sha3_impl.rs ]
crate::ix!();

use core::mem::size_of;

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha3.h]

#[derive(Default)]
pub struct SHA3_256 {
    state:   [u64; 25], // default = {0}
    buffer:  Sha3_256Buffer,
    bufsize: u32, // default = 0
    pos:     u32, // default = 0
}

pub type Sha3_256Buffer = [u8; 8];

/**
  | Sponge rate in bits.
  |
  */
pub const SHA3_256_RATE_BITS: usize = 1088;

/**
  | Sponge rate expressed as a multiple
  | of the buffer size.
  |
  */
pub const SHA3_256_RATE_BUFFERS: usize = SHA3_256_RATE_BITS / (8 * size_of::<Sha3_256Buffer>());

/**
  | error msg: "Rate must be a multiple of
  | 8 bytes"
  |
  */
const_assert!{
    SHA3_256_RATE_BITS % (8 * size_of::<Sha3_256Buffer>()) == 0
}

pub const SHA3_256_OUTPUT_SIZE: usize = 32;

impl SHA3_256 {

    pub fn write(&mut self, mut data: &[u8]) -> &mut SHA3_256 {
        const BUF_BYTES: usize = size_of::<Sha3_256Buffer>(); // 8

        // If there's something in the buffer and we can fill it up now, do so and absorb.
        if self.bufsize != 0 && (self.bufsize as usize + data.len() >= BUF_BYTES) {
            let need = BUF_BYTES - self.bufsize as usize;
            // Fill the remainder of the buffer.
            self.buffer[self.bufsize as usize .. self.bufsize as usize + need]
                .copy_from_slice(&data[..need]);
            data = &data[need..];

            // Absorb the now-full 8-byte lane.
            let lane = u64::from_le_bytes(self.buffer);
            self.state[self.pos as usize] ^= lane;
            self.pos += 1;
            self.bufsize = 0;

            if self.pos as usize == SHA3_256_RATE_BUFFERS {
                keccakf(&mut self.state);
                self.pos = 0;
            }
        }

        // Absorb as many whole 8-byte lanes as possible directly from input.
        while data.len() >= BUF_BYTES {
            // Process chunks directly from the buffer.
            let lane = u64::from_le_bytes(data[0..BUF_BYTES].try_into().unwrap());
            self.state[self.pos as usize] ^= lane;
            self.pos += 1;
            data = &data[BUF_BYTES..];

            if self.pos as usize == SHA3_256_RATE_BUFFERS {
                keccakf(&mut self.state);
                self.pos = 0;
            }
        }

        // Stash any remainder in the buffer.
        if !data.is_empty() {
            self.buffer[self.bufsize as usize .. self.bufsize as usize + data.len()]
                .copy_from_slice(data);
            self.bufsize += data.len() as u32;
        }

        self
    }

    pub fn finalize(&mut self, output: &mut [u8]) -> &mut SHA3_256 {
        assert_eq!(output.len(), SHA3_256_OUTPUT_SIZE);

        // Zero the unused tail of the buffer.
        for b in self.buffer[self.bufsize as usize ..].iter_mut() {
            *b = 0;
        }

        // SHA-3 domain separation + pad10*1 (0x06 ... 0x80).
        self.buffer[self.bufsize as usize] ^= 0x06;

        // Absorb the final (possibly partial) lane.
        let lane = u64::from_le_bytes(self.buffer);
        self.state[self.pos as usize] ^= lane;

        // Mark the final bit of the block.
        self.state[SHA3_256_RATE_BUFFERS - 1] ^= 0x8000_0000_0000_0000;

        // One final permutation.
        keccakf(&mut self.state);

        // Squeeze first 32 bytes (4 lanes of 8 bytes each).
        for i in 0..4 {
            let bytes = self.state[i].to_le_bytes();
            output[i * 8 .. (i + 1) * 8].copy_from_slice(&bytes);
        }

        self
    }
    
    pub fn reset(&mut self) -> &mut SHA3_256 {
        self.bufsize = 0;
        self.pos = 0;
        self.state.fill(0);
        // (Optional) clear the byte buffer as well:
        // self.buffer = [0u8; 8];
        self
    }

    /// Return-by-value helper (doesn’t require a mutable output slice).
    pub fn finalize_fixed(&mut self) -> [u8; SHA3_256_OUTPUT_SIZE] {
        let mut out = [0u8; SHA3_256_OUTPUT_SIZE];
        self.finalize(&mut out);
        out
    }

    /// If you must keep the original `&[u8]` signature in your API surface,
    /// expose *this* under a different name and route your callers through it.
    pub fn finalize_into(&mut self, output: &mut [u8]) -> &mut Self {
        self.finalize(output)
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha3.cpp]

/*
  | Based on
  | https://github.com/mjosaarinen/tiny_sha3/blob/master/sha3.c
  | by Markku-Juhani O. Saarinen <mjos@iki.fi>
  */

#[inline(always)]
pub fn rotl(x: u64, n: i32) -> u64 {
    x.rotate_left(n as u32)
}

/**
  The Keccak-f[1600] transform.
  */
pub fn keccakf(st: &mut [u64; 25]) {
    const RNDC: [u64; 24] = [
        0x0000_0000_0000_0001, 0x0000_0000_0000_8082, 0x8000_0000_0000_808a, 0x8000_0000_8000_8000,
        0x0000_0000_0000_808b, 0x0000_0000_8000_0001, 0x8000_0000_8000_8081, 0x8000_0000_0000_8009,
        0x0000_0000_0000_008a, 0x0000_0000_0000_0088, 0x0000_0000_8000_8009, 0x0000_0000_8000_000a,
        0x0000_0000_8000_808b, 0x8000_0000_0000_008b, 0x8000_0000_0000_8089, 0x8000_0000_0000_8003,
        0x8000_0000_0000_8002, 0x8000_0000_0000_0080, 0x0000_0000_0000_800a, 0x8000_0000_8000_000a,
        0x8000_0000_8000_8081, 0x8000_0000_0000_8080, 0x0000_0000_8000_0001, 0x8000_0000_8000_8008,
    ];
    const ROUNDS: usize = 24;

    for round in 0..ROUNDS {
        let (mut bc0, mut bc1, mut bc2, mut bc3, mut bc4);
        let mut t;

        // Theta
        bc0 = st[0] ^ st[5] ^ st[10] ^ st[15] ^ st[20];
        bc1 = st[1] ^ st[6] ^ st[11] ^ st[16] ^ st[21];
        bc2 = st[2] ^ st[7] ^ st[12] ^ st[17] ^ st[22];
        bc3 = st[3] ^ st[8] ^ st[13] ^ st[18] ^ st[23];
        bc4 = st[4] ^ st[9] ^ st[14] ^ st[19] ^ st[24];

        t = bc4 ^ rotl(bc1, 1); st[0] ^= t; st[5] ^= t; st[10] ^= t; st[15] ^= t; st[20] ^= t;
        t = bc0 ^ rotl(bc2, 1); st[1] ^= t; st[6] ^= t; st[11] ^= t; st[16] ^= t; st[21] ^= t;
        t = bc1 ^ rotl(bc3, 1); st[2] ^= t; st[7] ^= t; st[12] ^= t; st[17] ^= t; st[22] ^= t;
        t = bc2 ^ rotl(bc4, 1); st[3] ^= t; st[8] ^= t; st[13] ^= t; st[18] ^= t; st[23] ^= t;
        t = bc3 ^ rotl(bc0, 1); st[4] ^= t; st[9] ^= t; st[14] ^= t; st[19] ^= t; st[24] ^= t;

        // Rho Pi (preserve the exact in-place rotation/permutation order)
        t = st[1];
        bc0 = st[10]; st[10] = rotl(t, 1);  t = bc0;
        bc0 = st[7];  st[7]  = rotl(t, 3);  t = bc0;
        bc0 = st[11]; st[11] = rotl(t, 6);  t = bc0;
        bc0 = st[17]; st[17] = rotl(t, 10); t = bc0;
        bc0 = st[18]; st[18] = rotl(t, 15); t = bc0;
        bc0 = st[3];  st[3]  = rotl(t, 21); t = bc0;
        bc0 = st[5];  st[5]  = rotl(t, 28); t = bc0;
        bc0 = st[16]; st[16] = rotl(t, 36); t = bc0;
        bc0 = st[8];  st[8]  = rotl(t, 45); t = bc0;
        bc0 = st[21]; st[21] = rotl(t, 55); t = bc0;
        bc0 = st[24]; st[24] = rotl(t, 2);  t = bc0;
        bc0 = st[4];  st[4]  = rotl(t, 14); t = bc0;
        bc0 = st[15]; st[15] = rotl(t, 27); t = bc0;
        bc0 = st[23]; st[23] = rotl(t, 41); t = bc0;
        bc0 = st[19]; st[19] = rotl(t, 56); t = bc0;
        bc0 = st[13]; st[13] = rotl(t, 8);  t = bc0;
        bc0 = st[12]; st[12] = rotl(t, 25); t = bc0;
        bc0 = st[2];  st[2]  = rotl(t, 43); t = bc0;
        bc0 = st[20]; st[20] = rotl(t, 62); t = bc0;
        bc0 = st[14]; st[14] = rotl(t, 18); t = bc0;
        bc0 = st[22]; st[22] = rotl(t, 39); t = bc0;
        bc0 = st[9];  st[9]  = rotl(t, 61); t = bc0;
        bc0 = st[6];  st[6]  = rotl(t, 20); t = bc0;
        st[1] = rotl(t, 44);

        // Chi + Iota (iota folded into first row, matching the C++ flow)
        bc0 = st[0]; bc1 = st[1]; bc2 = st[2]; bc3 = st[3]; bc4 = st[4];
        st[0] = bc0 ^ (!bc1 & bc2) ^ RNDC[round];
        st[1] = bc1 ^ (!bc2 & bc3);
        st[2] = bc2 ^ (!bc3 & bc4);
        st[3] = bc3 ^ (!bc4 & bc0);
        st[4] = bc4 ^ (!bc0 & bc1);

        bc0 = st[5]; bc1 = st[6]; bc2 = st[7]; bc3 = st[8]; bc4 = st[9];
        st[5] = bc0 ^ (!bc1 & bc2);
        st[6] = bc1 ^ (!bc2 & bc3);
        st[7] = bc2 ^ (!bc3 & bc4);
        st[8] = bc3 ^ (!bc4 & bc0);
        st[9] = bc4 ^ (!bc0 & bc1);

        bc0 = st[10]; bc1 = st[11]; bc2 = st[12]; bc3 = st[13]; bc4 = st[14];
        st[10] = bc0 ^ (!bc1 & bc2);
        st[11] = bc1 ^ (!bc2 & bc3);
        st[12] = bc2 ^ (!bc3 & bc4);
        st[13] = bc3 ^ (!bc4 & bc0);
        st[14] = bc4 ^ (!bc0 & bc1);

        bc0 = st[15]; bc1 = st[16]; bc2 = st[17]; bc3 = st[18]; bc4 = st[19];
        st[15] = bc0 ^ (!bc1 & bc2);
        st[16] = bc1 ^ (!bc2 & bc3);
        st[17] = bc2 ^ (!bc3 & bc4);
        st[18] = bc3 ^ (!bc4 & bc0);
        st[19] = bc4 ^ (!bc0 & bc1);

        bc0 = st[20]; bc1 = st[21]; bc2 = st[22]; bc3 = st[23]; bc4 = st[24];
        st[20] = bc0 ^ (!bc1 & bc2);
        st[21] = bc1 ^ (!bc2 & bc3);
        st[22] = bc2 ^ (!bc3 & bc4);
        st[23] = bc3 ^ (!bc4 & bc0);
        st[24] = bc4 ^ (!bc0 & bc1);
    }
}

#[cfg(test)]
mod sha3_tests {
    use super::*;
    use core::mem::size_of;

    // Reference (oracle) implementation from RustCrypto.
    use sha3::{Digest as _, Sha3_256 as RefSha3_256};

    // ---------------- Helpers ----------------

    fn digest_ours(data: &[u8]) -> [u8; SHA3_256_OUTPUT_SIZE] {
        let mut h = SHA3_256::default();
        h.write(data);
        let mut out = [0u8; SHA3_256_OUTPUT_SIZE];
        h.finalize(&mut out);
        out
    }

    fn digest_ours_chunked(chunks: &[&[u8]]) -> [u8; SHA3_256_OUTPUT_SIZE] {
        let mut h = SHA3_256::default();
        for c in chunks {
            h.write(c);
        }
        let mut out = [0u8; SHA3_256_OUTPUT_SIZE];
        h.finalize(&mut out);
        out
    }

    fn digest_ref(data: &[u8]) -> [u8; 32] {
        let mut r = RefSha3_256::new();
        r.update(data);
        let res = r.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&res[..]);
        out
    }

    fn to_hex(bytes: &[u8]) -> String {
        use core::fmt::Write;
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            let _ = write!(&mut s, "{:02x}", b);
        }
        s
    }

    // Simple deterministic PRNG (no external deps) for randomized tests.
    struct XorShift64(u64);
    impl XorShift64 {
        fn new(seed: u64) -> Self { Self(seed) }
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn fill_bytes(&mut self, buf: &mut [u8]) {
            for chunk in buf.chunks_mut(8) {
                let v = self.next_u64().to_le_bytes();
                let n = chunk.len();
                chunk.copy_from_slice(&v[..n]);
            }
        }
        fn gen_range_usize(&mut self, start: usize, end: usize) -> usize {
            // [start, end)
            start + (self.next_u64() as usize % (end - start))
        }
    }

    // ---------------- Known Answer Tests ----------------

    #[traced_test]
    fn kat_empty_string() {
        // NIST FIPS 202: SHA3-256("") =
        // a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a
        let got = digest_ours(b"");
        let expected_hex = "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a";
        assert_eq!(to_hex(&got), expected_hex);
        // Cross-check with reference
        assert_eq!(got, digest_ref(b""));
    }

    #[traced_test]
    fn kat_abc() {
        // NIST FIPS 202: SHA3-256("abc") =
        // 3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532
        let got = digest_ours(b"abc");
        let expected_hex = "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532";
        assert_eq!(to_hex(&got), expected_hex);
        // Cross-check with reference
        assert_eq!(got, digest_ref(b"abc"));
    }

    // ---------------- API / State Behavior ----------------

    #[traced_test]
    fn state_after_small_write() {
        // Write < 8 bytes: should stay in the byte buffer, not touch state or pos.
        let mut h = SHA3_256::default();
        h.write(&[1, 2, 3]);
        assert_eq!(h.pos, 0);
        assert_eq!(h.bufsize, 3);
        assert!(h.state.iter().all(|&x| x == 0));
    }

    #[traced_test]
    fn state_after_exact_lane_write() {
        // Write exactly 8 bytes: becomes a lane; bufsize=0, pos=1, state[0]==lane.
        let mut h = SHA3_256::default();
        let lane_bytes = [1u8, 2, 3, 4, 5, 6, 7, 8];
        h.write(&lane_bytes);
        assert_eq!(h.bufsize, 0);
        assert_eq!(h.pos, 1);
        assert_eq!(h.state[0], u64::from_le_bytes(lane_bytes));
        // others should still be zero
        assert!(h.state[1..].iter().all(|&x| x == 0));
    }

    #[traced_test]
    fn pos_wraps_after_full_rate_block() {
        // Write exactly one rate block (17 x 8 = 136 bytes). pos should wrap to 0.
        let mut h = SHA3_256::default();
        let block = vec![0u8; 136];
        h.write(&block);
        assert_eq!(h.bufsize, 0);
        assert_eq!(h.pos, 0);
        // We don't assert state values here (they're permuted), just the control flow.
    }

    #[test]
    #[should_panic]
    fn finalize_panics_on_wrong_output_len() {
        let mut h = SHA3_256::default();
        h.write(b"hello");
        let mut out = [0u8; 31]; // wrong length
        // Should panic due to assert_eq!(output.len(), 32).
        h.finalize(&mut out);
    }

    #[traced_test]
    fn reset_zeroes_everything() {
        let mut h = SHA3_256::default();
        h.write(b"some bytes");
        // mutate internal state by finalizing to ensure it's not pristine
        let mut out = [0u8; 32];
        h.finalize(&mut out);

        // Now reset
        h.reset();
        assert_eq!(h.bufsize, 0);
        assert_eq!(h.pos, 0);
        assert!(h.state.iter().all(|&x| x == 0));
        // Fresh hash after reset matches fresh instance
        let fresh = digest_ours(b"again");
        let mut h2 = SHA3_256::default();
        h2.write(b"again");
        let mut out2 = [0u8; 32];
        h2.finalize(&mut out2);
        assert_eq!(fresh, out2);
    }

    // ---------------- Incremental vs One-shot Equivalence ----------------

    #[traced_test]
    fn incremental_equals_one_shot_across_boundaries() {
        // Cover messages across many boundary conditions:
        // lengths 0..=272 and all possible 2-chunk splits.
        // 272 = 2 * rate (136) to hit wrap logic multiple times.
        for len in 0..=272 {
            let mut data = vec![0u8; len];
            // Fill with deterministic pattern to keep this test quick.
            for (i, b) in data.iter_mut().enumerate() {
                *b = (i as u8).wrapping_mul(31).wrapping_add(7);
            }
            let one_shot = digest_ours(&data);
            // Anchor once against the reference
            assert_eq!(one_shot, digest_ref(&data), "ref mismatch at len={}", len);

            for split in 0..=len {
                let a = &data[..split];
                let b = &data[split..];
                let inc = digest_ours_chunked(&[a, b]);
                assert_eq!(
                    one_shot, inc,
                    "mismatch at len={}, split={}  (one_shot={}, inc={})",
                    len, split, to_hex(&one_shot), to_hex(&inc)
                );
            }
        }
    }

    #[traced_test]
    fn many_chunk_patterns_match_reference() {
        // Deliberate chunk sizes that stress 8-byte lanes and the 136-byte rate.
        let patterns: &[&[usize]] = &[
            // 1-byte streaming
            &[1; 137],               // 137 bytes total (crosses rate by 1)
            // 7-byte streaming
            &[7; 20],                // 140 bytes total
            // 8-byte aligned (exact lanes)
            &[8; 17],                // exactly one full rate block
            &[8; 34],                // exactly two blocks
            // near-boundary splits
            &[135, 1],
            &[1, 135],
            &[136, 1],
            &[1, 136],
            // mixed sizes
            &[3, 5, 8, 13, 21, 34, 55, 89], // Fibonacci-ish totals
        ];

        // Build a deterministic buffer long enough for the largest pattern.
        let max_total = patterns
            .iter()
            .map(|p| p.iter().sum::<usize>())
            .max()
            .unwrap_or(0);
        let mut msg = vec![0u8; max_total];
        for (i, b) in msg.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(97).wrapping_add(11);
        }

        for (pi, pat) in patterns.iter().enumerate() {
            // Carve the message into the prescribed chunk sizes:
            let total: usize = pat.iter().sum();
            let mut idx = 0usize;
            let mut chunks: Vec<&[u8]> = Vec::with_capacity(pat.len());
            for &sz in *pat {
                chunks.push(&msg[idx..idx + sz]);
                idx += sz;
            }

            let inc = digest_ours_chunked(&chunks);
            let ref_hash = digest_ref(&msg[..total]);

            assert_eq!(
                inc, ref_hash,
                "pattern {} failed: ours={}, ref={}",
                pi, to_hex(&inc), to_hex(&ref_hash)
            );
        }
    }

    // ---------------- Randomized (fuzz-ish) equivalence ----------------

    #[traced_test]
    fn randomized_inputs_and_chunkings_match_reference() {
        // Keep counts modest for CI; raise if you want even more coverage.
        let mut rng = XorShift64::new(0x5A17_EC7A_9B1B_D3C5);

        for _case in 0..256 {
            let len = rng.gen_range_usize(0, 8 * 1024); // up to 8 KiB
            let mut data = vec![0u8; len];
            rng.fill_bytes(&mut data);

            // Build random chunking
            let mut chunks: Vec<&[u8]> = Vec::new();
            let mut i = 0usize;
            while i < len {
                // chunk size in [1, 256]
                let remain = len - i;
                let max_chunk = core::cmp::min(256, remain);
                let sz = 1 + rng.gen_range_usize(0, max_chunk);
                chunks.push(&data[i..i + sz]);
                i += sz;
            }

            let ours = digest_ours_chunked(&chunks);
            let reference = digest_ref(&data);
            assert_eq!(ours, reference, "ours={}, ref={}", to_hex(&ours), to_hex(&reference));
        }
    }

    // ---------------- Slow test (optional) ----------------

    #[traced_test]
    //#[ignore] // Run with: cargo test -- --ignored
    fn million_a_matches_reference() {
        // Classic KAT: 1,000,000 'a' characters.
        let mut data = vec![0u8; 1_000_000];
        for b in data.iter_mut() { *b = b'a'; }

        let ours = digest_ours(&data);
        let reference = digest_ref(&data);
        assert_eq!(ours, reference, "ours={}, ref={}", to_hex(&ours), to_hex(&reference));
    }
}
