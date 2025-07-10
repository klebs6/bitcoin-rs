crate::ix!();

impl Sha256 {
    pub fn finalize(&mut self, hash: &mut [u8; SHA256_OUTPUT_SIZE]) {
        finalize_inner(self, hash.as_mut_ptr(), /*wipe=*/false);
    }
}

#[inline]
pub unsafe fn sha256_finalize(hash: *mut Sha256, out32: *mut u8) {
    debug_assert!(!hash.is_null() && !out32.is_null());
    finalize_inner(&mut *hash, out32, /*wipe=*/true);
}

// bitcoin‑sha256/src/sha256_finalize_core.rs (new, private)
#[inline(always)]
fn finalize_inner(ctx: &mut Sha256, out: *mut u8, wipe_state: bool) {
    /* ---- 1. padding ---- */
    const PAD: [u8; 64] = [
        0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let bytes = *ctx.bytes();
    let pad_len = 1 + ((119 - (bytes % 64)) % 64) as usize;

    // 64‑bit big‑endian descriptor (bits)
    let mut sizedesc = [0u8; 8];
    beio::u64_into(&mut sizedesc, bytes << 3);

    // Feed padding & length through the streaming writer
    ctx.write_ptr(PAD.as_ptr(), pad_len);
    ctx.write_ptr(sizedesc.as_ptr(), 8);

    /* ---- 2. serialise state ---- */
    let mut tmp = [0u32; 8];
    for (dst, src) in tmp.iter_mut().zip(ctx.s_mut().iter_mut()) {
        *dst = (*src).to_be();
        if wipe_state {
            *src = 0;
        }
    }

    /* ---- 3. output ---- */
    unsafe {
        std::ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, out, 32);
    }
}
