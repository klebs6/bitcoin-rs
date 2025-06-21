//! ShiftRows / InvShiftRows – column‑major‑aware implementation
//!
//! In this crate every byte “lane” is numbered
//!
//! ```text
//!   col 0  col 1  col 2  col 3
//! ┌──────┬──────┬──────┬──────┐
//! │  0   │  4   │  8   │ 12   │  ← row 0
//! │  1   │  5   │  9   │ 13   │  ← row 1
//! │  2   │  6   │ 10   │ 14   │  ← row 2
//! │  3   │  7   │ 11   │ 15   │  ← row 3
//! └──────┴──────┴──────┴──────┘
//! lane =  col · 4 + row      (column‑major)
//! ```
//
//! The old code used the *row‑major* mapping and therefore produced an
//! incorrect permutation.  The only fix required is to compute
//! `row = lane & 3` and `col = lane >> 2`, and to rebuild `new_lane` as
//! `new_col·4 + row`.
// ---------------- [ File: bitcoin-aes/src/shift_rows.rs ]
crate::ix!();

/// Forward AES ShiftRows (left‑rotate row *r* by *r* positions).
#[inline(always)]
pub fn shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "shift_rows – entry {:p}", s);

    unsafe {
        let original = (*s).slice;
        let mut perm = [0u16; 8];

        for lane in 0u16..16 {
            let row      = (lane & 3)  as usize;      // ← fixed
            let col      = (lane >> 2) as usize;
            let new_col  = (col + 4 - row) & 3;       // left‑rotate
            let new_lane = ((new_col << 2) | row) as u16;

            for bit in 0..8 {
                let bit_val = (original[bit] >> lane) & 1;
                perm[bit] |= bit_val << new_lane;
            }
        }
        (*s).slice = perm;
    }

    tracing::trace!(target: "aes", "shift_rows – exit");
}

/// Inverse ShiftRows (right‑rotate row *r* by *r* positions).
#[inline(always)]
pub fn inv_shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "inv_shift_rows – entry {:p}", s);

    unsafe {
        let original = (*s).slice;
        let mut perm = [0u16; 8];

        for lane in 0u16..16 {
            let row      = (lane & 3)  as usize;      // ← fixed
            let col      = (lane >> 2) as usize;
            let new_col  = (col + row) & 3;           // right‑rotate
            let new_lane = ((new_col << 2) | row) as u16;

            for bit in 0..8 {
                let bit_val = (original[bit] >> lane) & 1;
                perm[bit] |= bit_val << new_lane;
            }
        }
        (*s).slice = perm;
    }

    tracing::trace!(target: "aes", "inv_shift_rows – exit");
}

#[cfg(test)]
mod shift_rows_spec {
    use super::*;
    use rand::{thread_rng, Rng};
    use tracing::{info, debug, trace};

    /* ------------ helpers: pack / unpack  ----------------------------- */
    fn pack_bytes(bytes: &[u8; 16]) -> AESState {
        let mut slice = [0u16; 8];
        for bit in 0..8 {
            let mut w = 0u16;
            for lane in 0..16 {
                if (bytes[lane] >> bit) & 1 == 1 { w |= 1 << lane; }
            }
            slice[bit] = w;
        }
        AESState::from_slice(slice)
    }

    fn unpack_state(st: &AESState) -> [u8; 16] {
        let mut out = [0u8; 16];
        for lane in 0..16 {
            let mut b = 0u8;
            for bit in 0..8 {
                if (st.slice()[bit] >> lane) & 1 == 1 { b |= 1 << bit; }
            }
            out[lane] = b;
        }
        out
    }

    /* ---------------- specification helpers -------------------------- */
    fn spec_shift_rows(block: &[u8; 16]) -> [u8; 16] {
        let mut out = [0u8; 16];
        for row in 0..4 {
            for col in 0..4 {
                let src_idx = col * 4 + row;                 // column‑major
                let dst_col = (col + 4 - row) & 3;
                let dst_idx = dst_col * 4 + row;
                out[dst_idx] = block[src_idx];
            }
        }
        out
    }

    fn spec_inv_shift_rows(block: &[u8; 16]) -> [u8; 16] {
        let mut out = [0u8; 16];
        for row in 0..4 {
            for col in 0..4 {
                let src_idx = col * 4 + row;
                let dst_col = (col + row) & 3;
                let dst_idx = dst_col * 4 + row;
                out[dst_idx] = block[src_idx];
            }
        }
        out
    }

    /* ---------------- exhaustive / property tests -------------------- */

    /// Forward permutation matches the FIPS‑197 definition byte‑for‑byte.
    #[traced_test]
    fn forward_matches_spec() {
        let mut rng = thread_rng();
        for _ in 0..4_096 {
            let mut plain = [0u8; 16];
            rng.fill(&mut plain);

            let expect = spec_shift_rows(&plain);

            let mut st = pack_bytes(&plain);
            unsafe { shift_rows(&mut st as *mut _) };
            let got = unpack_state(&st);

            assert_eq!(got, expect, "ShiftRows mismatch");
        }
    }

    /// Inverse permutation matches the spec.
    #[traced_test]
    fn inverse_matches_spec() {
        let mut rng = thread_rng();
        for _ in 0..4_096 {
            let mut cipher = [0u8; 16];
            rng.fill(&mut cipher);

            let expect = spec_inv_shift_rows(&cipher);

            let mut st = pack_bytes(&cipher);
            unsafe { inv_shift_rows(&mut st as *mut _) };
            let got = unpack_state(&st);

            assert_eq!(got, expect, "InvShiftRows mismatch");
        }
    }

    /// Forward followed by inverse is identity (random states).
    #[traced_test]
    fn forward_then_inverse_is_identity() {
        let mut rng = thread_rng();
        for _ in 0..2_000 {
            let mut st = AESState::random(&mut rng);
            let original = st.clone();
            unsafe {
                shift_rows(&mut st as *mut _);
                inv_shift_rows(&mut st as *mut _);
            }
            assert_eq!(st.slice(), original.slice());
        }
    }

    /// Inverse followed by forward is identity.
    #[traced_test]
    fn inverse_then_forward_is_identity() {
        let mut rng = thread_rng();
        for _ in 0..2_000 {
            let mut st = AESState::random(&mut rng);
            let original = st.clone();
            unsafe {
                inv_shift_rows(&mut st as *mut _);
                shift_rows(&mut st as *mut _);
            }
            assert_eq!(st.slice(), original.slice());
        }
    }

    /// Edge vectors (all‑zero, all‑FF, and a walking‑1 bit).
    #[traced_test]
    fn edge_vectors_roundtrip() {
        let mut vectors = Vec::new();
        vectors.push([0u8; 16]);
        vectors.push([0xFFu8; 16]);
        for i in 0..128 {
            let mut v = [0u8; 16];
            v[i / 8] = 1u8 << (i % 8);
            vectors.push(v);
        }

        for v in vectors {
            let mut st = pack_bytes(&v);
            unsafe { shift_rows(&mut st as *mut _); inv_shift_rows(&mut st as *mut _); }
            assert_eq!(unpack_state(&st), v, "edge‑vector round‑trip failed");
        }
    }

    /// Population count must stay unchanged by either permutation.
    #[traced_test]
    fn bitcount_is_preserved() {
        let mut rng = thread_rng();
        for _ in 0..1_000 {
            let mut st = AESState::random(&mut rng);
            let pop_before: u32 =
                st.slice().iter().map(|w| w.count_ones()).sum();

            unsafe { shift_rows(&mut st as *mut _) };
            let pop_mid: u32 =
                st.slice().iter().map(|w| w.count_ones()).sum();
            unsafe { inv_shift_rows(&mut st as *mut _) };
            let pop_after: u32 =
                st.slice().iter().map(|w| w.count_ones()).sum();

            assert_eq!(pop_before, pop_mid);
            assert_eq!(pop_before, pop_after);
        }
    }

    /// Every byte value 0‑15 in its unique lane remains a permutation.
    #[traced_test]
    fn unique_byte_permutation() {
        let mut block = [0u8; 16];
        for i in 0u8..16 { block[i as usize] = i; }

        let expect_fwd = spec_shift_rows(&block);
        let expect_inv = spec_inv_shift_rows(&block);

        let mut st = pack_bytes(&block);
        unsafe { shift_rows(&mut st as *mut _) };
        assert_eq!(unpack_state(&st), expect_fwd, "unique‑byte forward");

        unsafe { inv_shift_rows(&mut st as *mut _) }; // back to original
        assert_eq!(unpack_state(&st), block, "round‑trip on unique‑byte");

        unsafe { inv_shift_rows(&mut st as *mut _) };
        assert_eq!(unpack_state(&st), expect_inv, "unique‑byte inverse");
    }
}
