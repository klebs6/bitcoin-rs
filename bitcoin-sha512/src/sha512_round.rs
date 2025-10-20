// ---------------- [ File: bitcoin-sha512/src/sha512_round.rs ]
crate::ix!();

/**
  | One round of SHA-512.
  |
  */
#[inline] pub fn sha512_round(
    a: u64, b: u64, c: u64, d: &mut u64,
    e: u64, f: u64, g: u64, h: &mut u64,
    k: u64, w: u64
) {
    #[inline(always)] fn my_sigma0(x: u64) -> u64 {
        // (x >> 28 | x << 36) ^ (x >> 34 | x << 30) ^ (x >> 39 | x << 25)
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }
    #[inline(always)] fn my_sigma1(x: u64) -> u64 {
        // (x >> 14 | x << 50) ^ (x >> 18 | x << 46) ^ (x >> 41 | x << 23)
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    let h_in = *h;
    let t1 = h_in
        .wrapping_add(my_sigma1(e))
        .wrapping_add(sha512_ch(e, f, g))
        .wrapping_add(k)
        .wrapping_add(w);
    let t2 = my_sigma0(a).wrapping_add(sha512_maj(a, b, c));
    *d = d.wrapping_add(t1);
    *h = t1.wrapping_add(t2);
}
