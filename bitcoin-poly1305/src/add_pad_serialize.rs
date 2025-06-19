crate::ix!();

// -----------------------------------------------------------------------------
// [poly1305] pad‑addition & tag serialization
// -----------------------------------------------------------------------------
#[inline(always)]
pub fn add_pad_serialize(
    out: &mut [u8; POLY1305_TAGLEN],
    h: &LimbArr5,
    key: &[u8; POLY1305_KEYLEN],
) {
    let mut f0: u64 = (h[0] as u64) | ((h[1] as u64) << 26);
    let mut f1: u64 = ((h[1] >> 6) as u64) | ((h[2] as u64) << 20);
    let mut f2: u64 = ((h[2] >> 12) as u64) | ((h[3] as u64) << 14);
    let mut f3: u64 = ((h[3] >> 18) as u64) | ((h[4] as u64) << 8);

    // word‑by‑word addition with carry propagation
    f0 = f0.wrapping_add(read_le32(&key[16..]) as u64);
    write_le32(&mut out[0..], f0);
    let mut carry = f0 >> 32;

    f1 = f1
        .wrapping_add(read_le32(&key[20..]) as u64)
        .wrapping_add(carry);
    write_le32(&mut out[4..], f1);
    carry = f1 >> 32;

    f2 = f2
        .wrapping_add(read_le32(&key[24..]) as u64)
        .wrapping_add(carry);
    write_le32(&mut out[8..], f2);
    carry = f2 >> 32;

    f3 = f3
        .wrapping_add(read_le32(&key[28..]) as u64)
        .wrapping_add(carry);
    write_le32(&mut out[12..], f3);
}
