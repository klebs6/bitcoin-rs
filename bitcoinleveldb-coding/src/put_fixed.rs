// ---------------- [ File: bitcoinleveldb-coding/src/put_fixed.rs ]
crate::ix!();

/**
  | Standard Put... routines append to
  | a string
  |
  */
pub fn put_fixed32(dst: *mut String, value: u32) {
    unsafe {
        trace!(
            value,
            ptr = ?dst,
            "put_fixed32: appending fixed32 value to string buffer"
        );
        let mut buf = [0u8; core::mem::size_of::<u32>()];
        encode_fixed32(buf.as_mut_ptr(), value);
        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        vec.extend_from_slice(&buf);
        debug!(
            new_len = vec.len(),
            "put_fixed32: buffer length after append"
        );
    }
}

pub fn put_fixed64(dst: *mut String, value: u64) {
    unsafe {
        trace!(
            value,
            ptr = ?dst,
            "put_fixed64: appending fixed64 value to string buffer"
        );
        let mut buf = [0u8; core::mem::size_of::<u64>()];
        encode_fixed64(buf.as_mut_ptr(), value);
        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        vec.extend_from_slice(&buf);
        debug!(
            new_len = vec.len(),
            "put_fixed64: buffer length after append"
        );
    }
}
