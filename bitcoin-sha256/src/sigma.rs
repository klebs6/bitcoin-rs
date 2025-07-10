crate::ix!();

/// Upper‑case Σ₀: `(x >> 2 | x << 30) ^ (x >> 13 | x << 19) ^ (x >> 22 | x << 10)`
#[inline(always)]
pub fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// Upper‑case Σ₁: `(x >> 6 | x << 26) ^ (x >> 11 | x << 21) ^ (x >> 25 | x << 7)`
#[inline(always)]
pub fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/** Lower‑case σ₀: `(x >> 7 | x << 25) ^ (x >> 18 | x << 14) ^ (x >> 3)` */
#[inline]
pub fn sha256_sigma0(x: u32) -> u32 {
    let res = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    trace!(target: "sha256", x, result = res, "sha256_sigma0 (root)");
    res
}

/** Lower‑case σ₁: `(x >> 17 | x << 15) ^ (x >> 19 | x << 13) ^ (x >> 10)` */
#[inline]
pub fn sha256_sigma1(x: u32) -> u32 {
    let res = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
    trace!(target: "sha256", x, result = res, "sha256_sigma1 (root)");
    res
}
