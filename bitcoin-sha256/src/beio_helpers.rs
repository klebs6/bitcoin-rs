crate::ix!();

/// Big‑endian helpers – private module‑local utilities
pub mod beio {

    #[inline]
    pub fn u32_into(dst: &mut [u8], v: u32) {
        dst[..4].copy_from_slice(&v.to_be_bytes());
    }

    #[inline]
    pub fn u64_into(dst: &mut [u8], v: u64) {
        dst[..8].copy_from_slice(&v.to_be_bytes());
    }
}
