// ---------------- [ File: bitcoinleveldb-coding/src/decode_fixed.rs ]
crate::ix!();

/**
  | Lower-level versions of Get... that
  | read directly from a character buffer
  | without any bounds checking.
  |
  */
#[inline]
pub fn decode_fixed32(ptr: *const u8) -> u32 {
    unsafe {
        let b0 = *ptr.add(0) as u32;
        let b1 = *ptr.add(1) as u32;
        let b2 = *ptr.add(2) as u32;
        let b3 = *ptr.add(3) as u32;
        let value = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
        trace!(
            ptr = ?ptr,
            value,
            "decode_fixed32: decoded 32-bit little-endian value"
        );
        value
    }
}

#[inline]
pub fn decode_fixed64(ptr: *const u8) -> u64 {
    unsafe {
        let b0 = *ptr.add(0) as u64;
        let b1 = *ptr.add(1) as u64;
        let b2 = *ptr.add(2) as u64;
        let b3 = *ptr.add(3) as u64;
        let b4 = *ptr.add(4) as u64;
        let b5 = *ptr.add(5) as u64;
        let b6 = *ptr.add(6) as u64;
        let b7 = *ptr.add(7) as u64;
        let value = b0
            | (b1 << 8)
            | (b2 << 16)
            | (b3 << 24)
            | (b4 << 32)
            | (b5 << 40)
            | (b6 << 48)
            | (b7 << 56);
        trace!(
            ptr = ?ptr,
            value,
            "decode_fixed64: decoded 64-bit little-endian value"
        );
        value
    }
}
