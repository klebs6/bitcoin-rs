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
