// ---------------- [ File: bitcoin-sha512/src/sha512_math.rs ]
crate::ix!();

#[inline] pub fn sha512_ch(x: u64, y: u64, z: u64) -> u64 {
    z ^ (x & (y ^ z))
}

#[inline] pub fn sha512_maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) | (z & (x | y))
}

#[inline] pub fn sha512_sigma0(x: u64) -> u64 {
    // (x >> 1 | x << 63) ^ (x >> 8 | x << 56) ^ (x >> 7)
    x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
}

#[inline] pub fn sha512_sigma1(x: u64) -> u64 {
    // (x >> 19 | x << 45) ^ (x >> 61 | x << 3) ^ (x >> 6)
    x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
}
