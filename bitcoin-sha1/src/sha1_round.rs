crate::ix!();

/**
  | One round of SHA-1.
  |
  */
#[inline(always)]
pub fn sha1_round(
    a: u32,
    b: &mut u32,
    _c: u32,
    _d: u32,
    e: &mut u32,
    f: u32,
    k: u32,
    w: u32,
) {
    trace!(
        "sha1_round: a={:#010x} b={:#010x} e={:#010x} f={:#010x} k={:#010x} w={:#010x}",
        a,
        *b,
        *e,
        f,
        k,
        w
    );
    *e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(f)
        .wrapping_add(k)
        .wrapping_add(w);
    *b = b.rotate_left(30);
}

#[inline(always)]
pub const fn sha1_f1(b: u32, c: u32, d: u32) -> u32 {
    d ^ (b & (c ^ d))
}

#[inline(always)]
pub const fn sha1_f2(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

#[inline(always)]
pub const fn sha1_f3(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (d & (b | c))
}

#[inline(always)]
pub const fn sha1_left(x: u32) -> u32 {
    (x << 1) | (x >> 31)
}
