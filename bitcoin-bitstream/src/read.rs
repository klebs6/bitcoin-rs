// ---------------- [ File: bitcoin-bitstream/src/read.rs ]
crate::ix!();

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readle16(ptr: *const u8) -> u16 {
    info!("readle16 called");
    unsafe {
        let mut tmp = [0u8; 2];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 2);
        let val = u16::from_le_bytes(tmp);
        debug!("readle16 returning 0x{:04X}", val);
        val
    }
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readle32(ptr: *const u8) -> u32 {
    info!("readle32 called");
    unsafe {
        let mut tmp = [0u8; 4];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 4);
        let val = u32::from_le_bytes(tmp);
        debug!("readle32 returning 0x{:08X}", val);
        val
    }
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readle64(ptr: *const u8) -> u64 {
    info!("readle64 called");
    unsafe {
        let mut tmp = [0u8; 8];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 8);
        let val = u64::from_le_bytes(tmp);
        debug!("readle64 returning 0x{:016X}", val);
        val
    }
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readbe16(ptr: *const u8) -> u16 {
    info!("readbe16 called");
    unsafe {
        let mut tmp = [0u8; 2];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 2);
        let val = u16::from_be_bytes(tmp);
        debug!("readbe16 returning 0x{:04X}", val);
        val
    }
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readbe32(ptr: *const u8) -> u32 {
    info!("readbe32 called");
    unsafe {
        let mut tmp = [0u8; 4];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 4);
        let val = u32::from_be_bytes(tmp);
        debug!("readbe32 returning 0x{:08X}", val);
        val
    }
}

#[instrument(level = "trace", skip(ptr))]
#[inline]
pub fn readbe64(ptr: *const u8) -> u64 {
    info!("readbe64 called");
    unsafe {
        let mut tmp = [0u8; 8];
        std::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr(), 8);
        let val = u64::from_be_bytes(tmp);
        debug!("readbe64 returning 0x{:016X}", val);
        val
    }
}
