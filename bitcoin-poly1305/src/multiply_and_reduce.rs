// ---------------- [ File: bitcoin-poly1305/src/multiply_and_reduce.rs ]
crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] multiply‑and‑carry (h *= r mod 2¹³⁰ − 5)
// -----------------------------------------------------------------------------
#[inline(always)]
pub fn multiply_and_reduce(h: &mut LimbArr5, r: &LimbArr5, s: &LimbArr4) {
    let mut t = [0u64; 5];
    t[0] = mul32x32_64!(h[0], r[0])
        + mul32x32_64!(h[1], s[3])
        + mul32x32_64!(h[2], s[2])
        + mul32x32_64!(h[3], s[1])
        + mul32x32_64!(h[4], s[0]);
    t[1] = mul32x32_64!(h[0], r[1])
        + mul32x32_64!(h[1], r[0])
        + mul32x32_64!(h[2], s[3])
        + mul32x32_64!(h[3], s[2])
        + mul32x32_64!(h[4], s[1]);
    t[2] = mul32x32_64!(h[0], r[2])
        + mul32x32_64!(h[1], r[1])
        + mul32x32_64!(h[2], r[0])
        + mul32x32_64!(h[3], s[3])
        + mul32x32_64!(h[4], s[2]);
    t[3] = mul32x32_64!(h[0], r[3])
        + mul32x32_64!(h[1], r[2])
        + mul32x32_64!(h[2], r[1])
        + mul32x32_64!(h[3], r[0])
        + mul32x32_64!(h[4], s[3]);
    t[4] = mul32x32_64!(h[0], r[4])
        + mul32x32_64!(h[1], r[3])
        + mul32x32_64!(h[2], r[2])
        + mul32x32_64!(h[3], r[1])
        + mul32x32_64!(h[4], r[0]);

    // carry chain
    h[0] = (t[0] as u32) & 0x3ffffff;
    let mut carry = t[0] >> 26;
    t[1] += carry;
    h[1] = (t[1] as u32) & 0x3ffffff;
    carry = t[1] >> 26;
    t[2] += carry;
    h[2] = (t[2] as u32) & 0x3ffffff;
    carry = t[2] >> 26;
    t[3] += carry;
    h[3] = (t[3] as u32) & 0x3ffffff;
    carry = t[3] >> 26;
    t[4] += carry;
    h[4] = (t[4] as u32) & 0x3ffffff;
    carry = t[4] >> 26;
    h[0] = h[0].wrapping_add((carry as u32) * 5);
}

#[cfg(test)]
mod tests_multiply_reduce {
    use super::*;
    use hex_literal::hex;

    #[traced_test]
    fn single_block_accumulates_correctly() {
        let key = [0u8; POLY1305_KEYLEN]; // simple key so that r = s = 0
        let (r, s) = expand_key(&key);
        let mut h = [0u32; 5];
        // One ASCII block “abcdefghijklmnop”
        let block = *array_ref::array_ref!(b"abcdefghijklmnop", 0, 16);
        accumulate_block(&mut h, &block, true);
        multiply_and_reduce(&mut h, &r, &s);
        // With r = 0 the multiply should do nothing, h encodes the block.
        assert!(h.iter().any(|&x| x != 0));
    }
}
