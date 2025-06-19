// ---------------- [ File: bitcoin-poly1305/src/accumulate_block.rs ]
crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] block accumulation
// -----------------------------------------------------------------------------
#[inline(always)]
pub fn accumulate_block(
    h: &mut LimbArr5,
    block: &[u8; 16],
    add_high_bit: bool,
) {
    let t0 = read_le32(&block[0..]);
    let t1 = read_le32(&block[4..]);
    let t2 = read_le32(&block[8..]);
    let t3 = read_le32(&block[12..]);

    h[0] = h[0].wrapping_add(t0 & 0x3ffffff);
    h[1] = h[1].wrapping_add((((t1 as u64) << 32 | t0 as u64) >> 26) as u32 & 0x3ffffff);
    h[2] = h[2].wrapping_add((((t2 as u64) << 32 | t1 as u64) >> 20) as u32 & 0x3ffffff);
    h[3] = h[3].wrapping_add((((t3 as u64) << 32 | t2 as u64) >> 14) as u32 & 0x3ffffff);
    h[4] = h[4].wrapping_add((t3 >> 8) | if add_high_bit { 1 << 24 } else { 0 });
}
