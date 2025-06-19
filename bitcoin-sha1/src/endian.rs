crate::ix!();

//──────────────────────────────────────────────────────────────────────────────
//  Helper functions for endian conversion
//──────────────────────────────────────────────────────────────────────────────
#[inline(always)]
pub unsafe fn read_be32(p: *const u8) -> u32 {
    u32::from_be_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}

#[inline(always)]
pub unsafe fn write_be32(p: *mut u8, v: u32) {
    std::ptr::copy_nonoverlapping(v.to_be_bytes().as_ptr(), p, 4);
}

#[inline(always)]
pub unsafe fn write_be64(p: *mut u8, v: u64) {
    std::ptr::copy_nonoverlapping(v.to_be_bytes().as_ptr(), p, 8);
}
