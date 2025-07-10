crate::ix!();

/// Scalar fallback: double‑SHA‑256 of one 64‑byte block.
///
/// This is chosen automatically whenever no SIMD backend is enabled.
///
/// # Safety
/// * `out`   – must point to **≥ 32** writable bytes.
/// * `inp`   – must point to **≥ 64** readable bytes.
/// * Regions must not overlap.
#[inline]
pub unsafe fn transform_d64_scalar(out: *mut u8, inp: *const u8) {
    // --- First SHA‑256 ------------------------------------------------------
    let mut mid = [0u8; 32];
    {
        let mut ctx = Sha256::new();
        ctx.write_ptr(inp, 64);
        ctx.finalize(&mut mid);
    }

    // --- Second SHA‑256 -----------------------------------------------------
    let mut final_digest = [0u8; 32];
    {
        let mut ctx = Sha256::new();
        ctx.write_ptr(mid.as_ptr(), 32);
        ctx.finalize(&mut final_digest);
    }

    // --- Write result -------------------------------------------------------
    copy_nonoverlapping(final_digest.as_ptr(), out, 32);
}
