// ---------------- [ File: bitcoin-crc32c/src/interface.rs ]
/*!
  | The API exported by the CRC32C project.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/include/crc32c/crc32c.h]
//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c.cc]

/// Computes the CRC32C of "count" bytes in the buffer pointed by "data".
///
/// Convenience helper: CRC of a single buffer.
///
/// # Safety
/// `data` **must** be valid for `count` bytes.
#[inline]
pub unsafe fn crc32c_value(data: *const u8, count: usize) -> u32 {
    trace!(count, "crc32c_value()");
    crc32c_extend(0, data, count)
}

/// Computes the CRC32C of "count" bytes in the buffer pointed by "data".
///
/// Shorthand identical to `crc32c_value`.
///
/// # Safety
/// `data` **must** be valid for `count` bytes.
#[inline]
pub unsafe fn crc32c(data: *const u8, count: usize) -> u32 {
    crc32c_extend(0, data, count)
}

/// Computes the CRC32C of the string's content.
///
/// Safe wrapper for Rust `&str`.
#[inline]
pub fn crc32c_with_str(string: &str) -> u32 {
    trace!(len = string.len(), "crc32c_with_str()");
    unsafe { crc32c_value(string.as_ptr(), string.len()) }
}

/// Extends "crc" with the CRC32C of "count" bytes in the buffer pointed by
/// "data".
///
/// Dispatch to the fastest back‑end that is *both* compiled‑in **and**
/// available at run‑time.
///
/// # Safety
/// `data` **must** be valid for `count` bytes.
#[inline]
pub unsafe fn crc32c_extend(crc: u32, data: *const u8, count: usize) -> u32 {
    trace!(crc, count, "crc32c_extend()‑dispatcher");

    // ---------- x86‑64 / SSE4.2 ----------
    #[cfg(target_arch = "x86_64")]
    {
        if can_use_sse42() {
            return crc32c_extend_sse42(crc, data, count);
        }
    }

    // ---------- AArch64 / CRC + PMULL ---
    #[cfg(target_arch = "aarch64")]
    {
        if can_use_arm64_crc32() {
            return crc32c_extend_arm64(crc, data, count);
        }
    }

    // ---------- Portable fallback -------
    crc32c_extend_portable(crc, data, count)
}
