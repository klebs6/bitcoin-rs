crate::ix!();

/// Choice function: z ^ (x & (y ^ z))
#[inline]
pub fn sha256_ch(x: u32, y: u32, z: u32) -> u32 {
    let res = z ^ (x & (y ^ z));
    trace!(target: "sha256", x, y, z, result = res, "sha256_ch");
    res
}

/// Majority function: (x & y) | (z & (x | y))
#[inline]
pub fn sha256_maj(x: u32, y: u32, z: u32) -> u32 {
    let res = (x & y) | (z & (x | y));
    trace!(target: "sha256", x, y, z, result = res, "sha256_maj");
    res
}

/// Lower‑case σ₀: (x >> 7 | x << 25) ^ (x >> 18 | x << 14) ^ (x >> 3)
#[inline]
pub fn sha256_sigma0(x: u32) -> u32 {
    let res = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    trace!(target: "sha256", x, result = res, "sha256_sigma0");
    res
}

/// Lower‑case σ₁: (x >> 17 | x << 15) ^ (x >> 19 | x << 13) ^ (x >> 10)
#[inline]
pub fn sha256_sigma1(x: u32) -> u32 {
    let res = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
    trace!(target: "sha256", x, result = res, "sha256_sigma1");
    res
}

/// Perform one SHA‑256 round:
/// (a, b, c, *d, e, f, g, *h) ← 1 round using constant *k* and schedule word *w*.
///
/// This matches the original C++ macro:
/// ```c
/// t1 = h + Σ1(e) + Ch(e,f,g) + k + w;
/// t2 = Σ0(a) + Maj(a,b,c);
/// d += t1;
/// h  = t1 + t2;
/// ```
#[inline]
pub fn sha256_round(
    a: u32,
    b: u32,
    c: u32,
    d: &mut u32,
    e: u32,
    f: u32,
    g: u32,
    h: &mut u32,
    k: u32,
    w: u32,
) {
    #[inline(always)]
    fn big_sigma0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }

    #[inline(always)]
    fn big_sigma1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }

    let ch  = sha256_ch(e, f, g);
    let maj = sha256_maj(a, b, c);

    let t1 = h
        .wrapping_add(big_sigma1(e))
        .wrapping_add(ch)
        .wrapping_add(k)
        .wrapping_add(w);

    let t2 = big_sigma0(a).wrapping_add(maj);

    *d = d.wrapping_add(t1);
    *h = t1.wrapping_add(t2);

    trace!(
        target: "sha256",
        a, b, c,
        d = *d,
        e, f, g,
        h = *h,
        k, w,
        ch, maj,
        t1, t2,
        "sha256_round (full signature)"
    );
}
