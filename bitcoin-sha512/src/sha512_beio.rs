// ---------------- [ File: bitcoin-sha512/src/sha512_beio.rs ]
crate::ix!();

// Small helpers for BE load/store (kept private to this module)
#[inline(always)]
pub fn read_be64(p: *const u8) -> u64 {
    let mut b = [0u8; 8];
    unsafe { ptr::copy_nonoverlapping(p, b.as_mut_ptr(), 8); }
    u64::from_be_bytes(b)
}

#[inline(always)]
pub fn write_be64_into(buf: &mut [u8], off: usize, x: u64) {
    let be = x.to_be_bytes();
    buf[off..off + 8].copy_from_slice(&be);
}
