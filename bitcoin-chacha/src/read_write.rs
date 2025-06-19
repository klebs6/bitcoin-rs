// ---------------- [ File: bitcoin-chacha/src/read_write.rs ]
crate::ix!();

#[inline(always)]
pub fn read_le32(src: &[u8]) -> u32 {
    u32::from_le_bytes(src[..4].try_into().unwrap())
}

#[inline(always)]
pub fn write_le32(dst: &mut [u8], v: u32) {
    dst[..4].copy_from_slice(&v.to_le_bytes());
}
