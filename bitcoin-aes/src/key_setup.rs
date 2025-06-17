crate::ix!();

/// Key‑schedule column mix (`column_c1(r) |= (column_0(s) ^= column_c2(a))`)
#[inline(always)]
pub fn key_setup_column_mix(
    s: *mut AESState,
    r: *mut AESState,
    a: *const AESState,
    c1: i32,
    c2: i32,
) {
    tracing::trace!(
        target: "aes",
        "key_setup_column_mix – s {:p}, r {:p}, a {:p}, c1 = {}, c2 = {}",
        s,
        r,
        a,
        c1,
        c2
    );

    unsafe {
        for b in 0..8 {
            let tmp = ((*a).slice[b] >> c2 as u32) & 0x1111;
            (*s).slice[b] ^= tmp;
            (*r).slice[b] |= ((*s).slice[b] & 0x1111) << c1 as u32;
        }
    }
}

/// Rotate the rows in s one position upwards, and xor in r
#[inline(always)]
pub fn key_setup_transform(s: *mut AESState, r: *const AESState) {
    tracing::trace!(
        target: "aes",
        "key_setup_transform – s {:p}, r {:p}",
        s,
        r
    );

    unsafe {
        for b in 0..8 {
            let v = (*s).slice[b];
            (*s).slice[b] = ((v >> 4) | (v << 12)) ^ (*r).slice[b];
        }
    }
}

