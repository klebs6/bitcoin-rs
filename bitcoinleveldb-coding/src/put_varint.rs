// ---------------- [ File: bitcoinleveldb-coding/src/put_varint.rs ]
crate::ix!();

pub fn put_varint32(dst: *mut String, v: u32) {
    unsafe {
        trace!(
            value = v,
            ptr = ?dst,
            "put_varint32: appending varint32 value to string buffer"
        );
        let mut buf = [0u8; 5];
        let ptr_start = buf.as_mut_ptr();
        let ptr_end = encode_varint32(ptr_start, v);
        let len = ptr_end.offset_from(ptr_start) as usize;

        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        vec.extend_from_slice(&buf[..len]);
        debug!(
            new_len = vec.len(),
            encoded_len = len,
            "put_varint32: buffer length after append"
        );
    }
}

pub fn put_varint64(dst: *mut String, v: u64) {
    unsafe {
        trace!(
            value = v,
            ptr = ?dst,
            "put_varint64: appending varint64 value to string buffer"
        );
        let mut buf = [0u8; 10];
        let ptr_start = buf.as_mut_ptr();
        let ptr_end = encode_varint64(ptr_start, v);
        let len = ptr_end.offset_from(ptr_start) as usize;

        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        vec.extend_from_slice(&buf[..len]);
        debug!(
            new_len = vec.len(),
            encoded_len = len,
            "put_varint64: buffer length after append"
        );
    }
}
