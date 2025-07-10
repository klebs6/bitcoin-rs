crate::ix!();

/// FFI‑compatible front‑end to [`Sha256::write_ptr`].
///
/// # Safety
/// * **`hash`** must be a valid, writable pointer to a live [`Sha256`] value.
/// * **`data`** must be either  
///   * a valid, readable pointer to at least `len` bytes **or**  
///   * *null* **iff** `len == 0`.
///
/// These pre‑conditions mirror those enforced by the original C reference
/// implementation and are **not** re‑validated internally (aside from a
/// debug‑assert).  Violating them results in *undefined behaviour*.
///
/// # Behaviour
/// Delegates the entire write operation to [`Sha256::write_ptr`], providing a
/// thin, zero‑copy bridge for C‑style callers while preserving all batching,
/// buffering, and block‑processing semantics implemented in Rust.
///
/// The function logs its invocation and completion via the `tracing` crate
/// under the `sha256` target.
#[inline]
pub unsafe fn sha256_write(hash: *mut Sha256, data: *const u8, len: usize) {
    use tracing::{debug, trace};

    debug!(
        target: "sha256",
        ptr = ?hash,
        len,
        "sha256_write: entering (FFI call)"
    );

    debug_assert!(
        !hash.is_null(),
        "sha256_write: `hash` must be non‑null (debug build assert)"
    );
    debug_assert!(
        !data.is_null() || len == 0,
        "sha256_write: `data` is null while `len` ≠ 0 (debug build assert)"
    );

    // Convert the raw pointer into a mutable reference and forward the call.
    // SAFETY: Caller guarantees validity of `hash` for writes and of `data`
    // for reads (see function‑level safety contract above).
    (&mut *hash).write_ptr(data, len);

    trace!(target: "sha256", "sha256_write: leaving (FFI call)");
}
