// ---------------- [ File: bitcoinleveldb-coding/src/encode_varint.rs ]
crate::ix!();

/**
  | Lower-level versions of Put... that write
  | directly into a character buffer and return
  | a pointer just past the last byte written.
  |
  | REQUIRES: dst has enough space for the value
  | being written
  */
pub fn encode_varint32(dst: *mut u8, v: u32) -> *mut u8 {
    unsafe {
        trace!(
            ptr = ?dst,
            value = v,
            "encode_varint32: encoding 32-bit varint"
        );
        let mut ptr = dst;
        const B: u32 = 128;

        if v < (1 << 7) {
            *ptr = v as u8;
            ptr = ptr.add(1);
        } else if v < (1 << 14) {
            *ptr = (v | B) as u8;
            ptr = ptr.add(1);
            *ptr = (v >> 7) as u8;
            ptr = ptr.add(1);
        } else if v < (1 << 21) {
            *ptr = (v | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 7) | B) as u8;
            ptr = ptr.add(1);
            *ptr = (v >> 14) as u8;
            ptr = ptr.add(1);
        } else if v < (1 << 28) {
            *ptr = (v | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 7) | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 14) | B) as u8;
            ptr = ptr.add(1);
            *ptr = (v >> 21) as u8;
            ptr = ptr.add(1);
        } else {
            *ptr = (v | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 7) | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 14) | B) as u8;
            ptr = ptr.add(1);
            *ptr = ((v >> 21) | B) as u8;
            ptr = ptr.add(1);
            *ptr = (v >> 28) as u8;
            ptr = ptr.add(1);
        }

        trace!(
            end_ptr = ?ptr,
            "encode_varint32: finished encoding varint32"
        );
        ptr
    }
}

pub fn encode_varint64(dst: *mut u8, v: u64) -> *mut u8 {
    unsafe {
        trace!(
            ptr = ?dst,
            value = v,
            "encode_varint64: encoding 64-bit varint"
        );
        let mut ptr = dst;
        let mut val = v;
        const B: u8 = 128;

        while val >= B as u64 {
            *ptr = (val as u8) | B;
            ptr = ptr.add(1);
            val >>= 7;
        }

        *ptr = val as u8;
        ptr = ptr.add(1);

        trace!(
            end_ptr = ?ptr,
            "encode_varint64: finished encoding varint64"
        );
        ptr
    }
}
