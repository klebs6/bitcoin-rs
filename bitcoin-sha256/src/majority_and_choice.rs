crate::ix!();

/// Choice function: `z ^ (x & (y ^ z))`
#[inline(always)]
pub fn sha256_ch(x: u32, y: u32, z: u32) -> u32 {
    let res = z ^ (x & (y ^ z));
    trace!(target: "sha256", x, y, z, res, "sha256_ch");
    res
}

/// Majority function: `(x & y) | (z & (x | y))`
#[inline(always)]
pub fn sha256_maj(x: u32, y: u32, z: u32) -> u32 {
    let res = (x & y) | (z & (x | y));
    trace!(target: "sha256", x, y, z, res, "sha256_maj");
    res
}
