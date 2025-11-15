// ---------------- [ File: bitcoinleveldb-coding/src/encode_fixed.rs ]
crate::ix!();

/**
  | Lower-level versions of Put... that write
  | directly into a character buffer
  |
  | REQUIRES: dst has enough space for the value
  | being written
  */
#[inline]
pub fn encode_fixed32(dst: *mut u8, value: u32) {
    unsafe {
        trace!(
            ptr = ?dst,
            value,
            "encode_fixed32: encoding 32-bit value to little-endian bytes"
        );
        *dst.add(0) = (value & 0xff) as u8;
        *dst.add(1) = ((value >> 8) & 0xff) as u8;
        *dst.add(2) = ((value >> 16) & 0xff) as u8;
        *dst.add(3) = ((value >> 24) & 0xff) as u8;
    }
}

#[inline]
pub fn encode_fixed64(dst: *mut u8, value: u64) {
    unsafe {
        trace!(
            ptr = ?dst,
            value,
            "encode_fixed64: encoding 64-bit value to little-endian bytes"
        );
        *dst.add(0) = (value & 0xff) as u8;
        *dst.add(1) = ((value >> 8) & 0xff) as u8;
        *dst.add(2) = ((value >> 16) & 0xff) as u8;
        *dst.add(3) = ((value >> 24) & 0xff) as u8;
        *dst.add(4) = ((value >> 32) & 0xff) as u8;
        *dst.add(5) = ((value >> 40) & 0xff) as u8;
        *dst.add(6) = ((value >> 48) & 0xff) as u8;
        *dst.add(7) = ((value >> 56) & 0xff) as u8;
    }
}
