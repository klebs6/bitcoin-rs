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

    /// Initial SHA‑256 state (FIPS 180‑4 IV).
    const IV: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    /// The test message used by Bitcoin Core self‑tests.
    const DATA: &[u8] = br#"-Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Et molestie ac feugiat sed lectus vestibulum mattis ullamcorper. Morbi blandit cursus risus at ultrices mi tempus imperdiet nulla. Nunc congue nisi vita suscipit tellus mauris. Imperdiet proin fermentum leo vel orci. Massa tempor nec feugiat nisl pretium fusce id velit. Telus in metus vulputate eu scelerisque felis. Mi tempus imperdiet nulla malesuada pellentesque. Tristique magna sit."#;

    /// Expected state after hashing the first *i* 64‑byte blocks of `DATA[1..]`.
    const EXPECTED: [[u32; 8]; 9] = [
        IV,
        [0x91f8ec6b, 0x4da10fe3, 0x1c9c292c, 0x45e18185, 0x435cc111, 0x3ca26f09, 0xeb954cae, 0x402a7069],
        [0xcabea5ac, 0x374fb97c, 0x182ad996, 0x7bd69cbf, 0x450ff900, 0xc1d2be8a, 0x6a41d505, 0xe6212dc3],
        [0xbcff09d6, 0x3e76f36e, 0x3ecb2501, 0x78866e97, 0xe1c1e2fd, 0x32f4eaff, 0x8aa6c4e5, 0xdfc024bc],
        [0xa08c5d94, 0x0a862f93, 0x6b7f2f40, 0x8f9fae76, 0x6d40439f, 0x79dcee0c, 0x3e39ff3a, 0xdc3bdbb1],
        [0x216a0895, 0x9f1a3662, 0xe99946f9, 0x87ba4364, 0x0fb5db2c, 0x12bed3d3, 0x6689c0c7, 0x292f1b04],
        [0xca3067f8, 0xbc8c2656, 0x37cb7e0d, 0x9b6b8b0f, 0x46dc380b, 0xf1287f57, 0xc42e4b23, 0x3fefe94d],
        [0x3e4c4039, 0xbb6fca8c, 0x6f27d2f7, 0x301e44a4, 0x8352ba14, 0x5769ce37, 0x48a1155f, 0xc0e1c4c6],
        [0xfe2fa9dd, 0x69d0862b, 0x1ae0db23, 0x471f9244, 0xf55c0145, 0xc30f9c3b, 0x40a84ea0, 0x5b8a266c],
    ];

    #[traced_test]
    fn state_after_each_block_matches_reference() {
        for blocks in 0..=8 {
            let mut state = IV;
            unsafe {
                sha256_transform(
                    state.as_mut_ptr(),
                    DATA.as_ptr().add(1), // Skip the leading '-' as per reference test.
                    blocks,
                );
            }
            assert_eq!(
                state, EXPECTED[blocks],
                "state mismatch after {} block(s)",
                blocks
            );
        }
    }
}
