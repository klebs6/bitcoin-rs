// ---------------- [ File: bitcoin-crc32c/src/read_le.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_read_le.h]

/// Read a little‑endian `u32` from an arbitrary byte pointer.
///
/// # Safety
/// `buffer` **must** be valid for at least four bytes.
#[inline]
pub unsafe fn read_uint32le(buffer: *const u8) -> u32 {
    let value = if cfg!(target_endian = "big") {
        let b: &[u8] = std::slice::from_raw_parts(buffer, 4);
        (b[0] as u32)
            | ((b[1] as u32) << 8)
            | ((b[2] as u32) << 16)
            | ((b[3] as u32) << 24)
    } else {
        let mut tmp = MaybeUninit::<u32>::uninit();
        core::ptr::copy_nonoverlapping(buffer, tmp.as_mut_ptr() as *mut u8, 4);
        u32::from_le(tmp.assume_init())
    };
    trace!(addr = ?buffer, value, "read_uint32le");
    value
}

/// Read a little‑endian `u64` from an arbitrary byte pointer.
///
/// # Safety
/// `buffer` **must** be valid for at least eight bytes.
#[inline]
pub unsafe fn read_uint64le(buffer: *const u8) -> u64 {
    let value = if cfg!(target_endian = "big") {
        let b: &[u8] = std::slice::from_raw_parts(buffer, 8);
        (b[0] as u64)
            | ((b[1] as u64) << 8)
            | ((b[2] as u64) << 16)
            | ((b[3] as u64) << 24)
            | ((b[4] as u64) << 32)
            | ((b[5] as u64) << 40)
            | ((b[6] as u64) << 48)
            | ((b[7] as u64) << 56)
    } else {
        let mut tmp = MaybeUninit::<u64>::uninit();
        core::ptr::copy_nonoverlapping(buffer, tmp.as_mut_ptr() as *mut u8, 8);
        u64::from_le(tmp.assume_init())
    };
    trace!(addr = ?buffer, value, "read_uint64le");
    value
}
