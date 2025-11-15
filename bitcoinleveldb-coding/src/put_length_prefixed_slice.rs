// ---------------- [ File: bitcoinleveldb-coding/src/put_length_prefixed_slice.rs ]
crate::ix!();

pub fn put_length_prefixed_slice(dst: *mut String, value: &Slice) {
    unsafe {
        let len: usize = *value.size();
        trace!(
            len,
            ptr = ?dst,
            "put_length_prefixed_slice: appending length-prefixed slice"
        );
        if len > u32::MAX as usize {
            error!(
                len,
                "put_length_prefixed_slice: slice length exceeds u32::MAX"
            );
            return;
        }

        put_varint32(dst, len as u32);

        if len == 0 {
            debug!(
                "put_length_prefixed_slice: empty slice, nothing more to append"
            );
            return;
        }

        let data_ptr: *const u8 = *value.data();
        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        let bytes = core::slice::from_raw_parts(data_ptr, len);
        vec.extend_from_slice(bytes);
        debug!(
            new_len = vec.len(),
            "put_length_prefixed_slice: buffer length after append"
        );
    }
}
