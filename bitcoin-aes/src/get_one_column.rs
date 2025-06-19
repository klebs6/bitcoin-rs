// ---------------- [ File: bitcoin-aes/src/get_one_column.rs ]
crate::ix!();

/// Extract one 4‑byte column (`column_0(s) = column_c(a)`).
#[inline(always)]
pub fn get_one_column(dst: *mut AESState, src: *const AESState, c: i32) {
    tracing::trace!(
        target: "aes",
        "get_one_column – dst {:p} ← src {:p}, c = {}",
        dst,
        src,
        c
    );

    unsafe {
        for b in 0..8 {
            (*dst).slice[b] = ((*src).slice[b] >> c as u32) & 0x1111;
        }
    }
}

#[cfg(test)]
mod column_extraction_validation {
    use super::*;

    /// Verify that `get_one_column` returns **exactly** the requested nibble
    /// positions (bits 0,4,8,12) of every slice, for all column indices 0‑3.
    #[traced_test]
    fn extracts_correct_nibbles() {
        let mut rng = thread_rng();

        for _ in 0..4_096 {
            let original = AESState::random(&mut rng);

            for col in 0..4 {
                let mut extracted = AESState::default();
                unsafe {
                    get_one_column(
                        &mut extracted as *mut _,
                        &original as *const _,
                        col,
                    );
                }

                // Build reference result in safe Rust.
                let mut reference = AESState::default();
                for bit in 0..8 {
                    reference.slice[bit] =
                        (original.slice()[bit] >> col) & 0x1111;
                }

                debug!(target: "test", col, ref_slice = ?reference.slice(), ext_slice = ?extracted.slice());
                assert_eq!(
                    extracted.slice(),
                    reference.slice(),
                    "column {col} mismatch"
                );
            }
        }
    }
}
