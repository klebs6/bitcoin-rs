// ---------------- [ File: bitcoin-aes/src/get_one_column.rs ]
crate::ix!();

/// Extract one 4‑byte column (`column_0(dst) = column_c(src)`).
#[inline(always)]
pub fn get_one_column(dst: *mut AESState, src: *const AESState, c: i32) {
    tracing::trace!(
        target: "aes",
        "get_one_column – dst {:p} ← src {:p}, c = {}",
        dst, src, c
    );

    debug_assert!((0..=3).contains(&c), "column index out of range");

    const COL_MASK: [u16; 4] = [0x000F, 0x00F0, 0x0F00, 0xF000]; // lanes 0‑3,4‑7,8‑11,12‑15

    unsafe {
        let mask  = COL_MASK[c as usize];
        let shift = (c * 4) as u32;          // bring selected column to nibble 0

        for b in 0..8 {
            (*dst).slice[b] = ((*src).slice[b] & mask) >> shift;
        }
    }
}

#[cfg(test)]
mod column_extraction_validation {
    use super::*;

    /// Extracted column must *exactly* equal the source column for every
    /// slice bit, and all other bits must be zero.
    #[traced_test]
    fn extracts_exact_requested_column() {
        let mut rng = thread_rng();

        for _ in 0..4_096 {
            let original = AESState::random(&mut rng);

            for col in 0..4 {
                let mut extracted = AESState::default();
                unsafe { get_one_column(&mut extracted, &original, col); }

                // expected: isolate requested nibble and align to low nibble
                let mut reference = AESState::default();
                for bit in 0..8 {
                    let word = original.slice()[bit];
                    let nib  = (word >> (col * 4)) & 0x000F;
                    reference.slice[bit] = nib;
                }

                debug!(
                    target: "test",
                    col,
                    ref_slice = ?reference.slice(),
                    ext_slice = ?extracted.slice(),
                    "column extraction comparison"
                );
                assert_eq!(
                    extracted.slice(),
                    reference.slice(),
                    "column {col} mismatch"
                );
            }
        }
    }

    /// Property: population‑count of the extracted state equals the
    /// population‑count of the chosen column in the source.
    #[traced_test]
    fn popcount_matches_source_column() {
        let mut rng = thread_rng();

        for _ in 0..2_048 {
            let src = AESState::random(&mut rng);
            let col = rng.gen_range(0..4);

            // popcount in src for selected column
            let mut src_pop = 0u32;
            for bit in 0..8 {
                let nib = (src.slice()[bit] >> (col * 4)) & 0x000F;
                src_pop += nib.count_ones();
            }

            let mut dst = AESState::default();
            unsafe { get_one_column(&mut dst, &src, col); }

            let dst_pop: u32 = dst.slice().iter().map(|w| w.count_ones()).sum();

            trace!(target: "test", col, src_pop, dst_pop, "popcount verification");
            assert_eq!(dst_pop, src_pop, "wrong popcount for column {col}");
        }
    }
}
