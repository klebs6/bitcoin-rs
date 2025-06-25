// ---------------- [ File: bitcoin-compat/src/strnlen.rs ]
//! Safe replacement for the C `strnlen` routine.
//!
//! The C++ reference implementation relied on `memchr`
//! to locate the first `NUL` byte or to stop after
//! `max_len` bytes.  
//! In Rust we can achieve the same behaviour with
//! `slice::from_raw_parts` while adding *robust*
//! tracing so that potential UB is detectable in
//! production.
//
//! # Safety
//! This function takes a raw pointer because it must
//! operate on arbitrary buffers originating from FFI.
//! The caller must uphold the usual guarantees:
//!
//! * `start` must be valid for reads of at least
//!   `max_len` bytes.
//! * The buffer must not be mutated for the lifetime
//!   of the call (no concurrent writes).
//!
//! If either condition is violated the behaviour is
//! undefined – identical to the original C contract.
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/strnlen.cpp]

use tracing::{error, trace};

/// Return the length of the C‑string starting at `start`
/// but never examine more than `max_len` bytes.
///
/// When no `NUL` byte is found within `max_len`, the
/// function returns `max_len`.
#[inline]
pub unsafe fn strnlen(start: *const u8, max_len: usize) -> usize {
    // Null pointer → programmer error.  Log loudly and
    // return zero to avoid walking random memory.
    if start.is_null() {
        error!(
            target: "compat::strnlen",
            "called with null pointer – returning 0"
        );
        return 0;
    }

    trace!(
        target: "compat::strnlen",
        ptr = ?start,
        max_len,
        "enter strnlen"
    );

    // SAFETY: caller promises the region is readable
    // for `max_len` bytes.
    let bytes: &[u8] = std::slice::from_raw_parts(start, max_len);

    let len = bytes
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(max_len);

    trace!(
        target: "compat::strnlen",
        result = len,
        "exit strnlen"
    );

    len
}
