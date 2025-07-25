// ---------------- [ File: bitcoin-sha256/src/beio_helpers.rs ]
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

#[cfg(BIG_ENDIAN)]
macro_rules! be32 {
    ($x:ident) => {
        /*
                (x)
        */
    }
}

#[cfg(LITTLE_ENDIAN)]
macro_rules! be32 {
    ($p:ident) => {
        /*
                ((((p) & 0xFF) << 24) | (((p) & 0xFF00) << 8) | (((p) & 0xFF0000) >> 8) | (((p) & 0xFF000000) >> 24))
        */
    }
}

#[inline(always)]
pub unsafe fn read_be32(p: *const u8) -> u32 {
    u32::from_be_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}
