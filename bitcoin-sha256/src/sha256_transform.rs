// ---------------- [ File: bitcoin-sha256/src/sha256_transform.rs ]
crate::ix!();

/// Perform *`blocks`* SHA‑256 transformations over consecutive 64‑byte chunks.
///
/// # Safety
/// * `s` **must** point to **≥ 8** writable `u32` words.
/// * `chunk` **must** point to **64 × `blocks`** readable bytes.
/// * The two regions must not overlap.
#[inline]
pub unsafe fn sha256_transform(
    s: *mut u32,
    mut chunk: *const u8,
    mut blocks: usize,
) {
    trace!(
        target: "sha256",
        total_blocks = blocks,
        "sha256_transform: starting batch"
    );

    while blocks != 0 {
        sha256_transform_block(s, chunk);
        chunk = chunk.add(64);
        blocks -= 1;

        trace!(
            target: "sha256",
            remaining_blocks = blocks,
            "sha256_transform: completed block"
        );
    }
}

#[cfg(test)]
mod sha256_transform_validation {
    use super::*;

    /// State after hashing the first `i` 64‑byte blocks of `SELF_TEST_DATA[1..]`.
    #[traced_test]
    fn state_after_each_block_matches_reference() {
        for blocks in 0..=8 {
            let mut state = fixtures::INIT;
            unsafe {
                sha256_transform(
                    state.as_mut_ptr(),
                    fixtures::SELF_TEST_DATA.as_ptr().add(1), // skip the leading '-'
                    blocks,
                );
            }
            assert_eq!(
                state, fixtures::COMP_STATES[blocks],
                "state mismatch after {} block(s)",
                blocks
            );
        }
    }
}
