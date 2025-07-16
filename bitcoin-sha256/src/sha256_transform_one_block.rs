// ---------------- [ File: bitcoin-sha256/src/sha256_transform_one_block.rs ]
crate::ix!();

/// Perform exactly one SHA-256 transformation, processing exactly 16 big-endian
/// 32-bit words (i.e., exactly one 64-byte chunk).
///
/// # Safety
/// - `s` **must** point to **at least** eight valid, writable `u32` words.
/// - `chunk` **must** point to exactly 16 readable big-endian `u32` words.
/// - Regions pointed to by `s` and `chunk` must not overlap.
///
/// Behaviour is *wrapping* on 32‑bit overflow, matching the reference C++.
#[inline]
pub unsafe fn sha256_transform_one_block_be_words(
    s: *mut u32,
    chunk: *const u32,
) {
    trace!(
        target: "sha256",
        "sha256_transform_one_block_be_words: forwarding call as single block"
    );

    sha256_transform(s, chunk as *const u8, 1);
}
