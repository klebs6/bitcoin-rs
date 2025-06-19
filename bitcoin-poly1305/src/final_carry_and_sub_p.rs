crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] final carry‑prop & conditional subtraction
// -----------------------------------------------------------------------------
#[inline(always)]
pub fn final_carry_and_sub_p(h: &mut LimbArr5) {
    // propagate carries once
    let mut c = h[0] >> 26;
    h[0] &= 0x3ffffff;
    h[1] = h[1].wrapping_add(c);
    c = h[1] >> 26;
    h[1] &= 0x3ffffff;
    h[2] = h[2].wrapping_add(c);
    c = h[2] >> 26;
    h[2] &= 0x3ffffff;
    h[3] = h[3].wrapping_add(c);
    c = h[3] >> 26;
    h[3] &= 0x3ffffff;
    h[4] = h[4].wrapping_add(c);
    c = h[4] >> 26;
    h[4] &= 0x3ffffff;
    h[0] = h[0].wrapping_add((c as u32) * 5);
    c = h[0] >> 26;
    h[0] &= 0x3ffffff;
    h[1] = h[1].wrapping_add(c);

    // compare h with p = 2¹³⁰ − 5 (compute g = h + 5 − p)
    let mut g = [0u64; 5];
    g[0] = h[0] as u64 + 5;
    let mut b = g[0] >> 26;
    g[0] &= 0x3ffffff;
    g[1] = h[1] as u64 + b;
    b = g[1] >> 26;
    g[1] &= 0x3ffffff;
    g[2] = h[2] as u64 + b;
    b = g[2] >> 26;
    g[2] &= 0x3ffffff;
    g[3] = h[3] as u64 + b;
    b = g[3] >> 26;
    g[3] &= 0x3ffffff;
    g[4] = h[4] as u64 + b - (1 << 26);

    // select h if h < p else g (i.e. h − p)
    let mask = (((g[4] >> 63) ^ 1).wrapping_sub(1)) as u32;
    let nmask = !mask;
    h[0] = (h[0] & nmask) | (g[0] as u32 & mask);
    h[1] = (h[1] & nmask) | (g[1] as u32 & mask);
    h[2] = (h[2] & nmask) | (g[2] as u32 & mask);
    h[3] = (h[3] & nmask) | (g[3] as u32 & mask);
    h[4] = (h[4] & nmask) | (g[4] as u32 & mask);
}

#[cfg(test)]
mod tests_final_and_tag {
    use super::*;
    use hex_literal::hex;

    #[traced_test]
    fn rfc7539_vector_1_tag_matches() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);
        assert_eq!(tag, expected);
    }
}
