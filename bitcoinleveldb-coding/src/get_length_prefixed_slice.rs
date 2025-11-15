// ---------------- [ File: bitcoinleveldb-coding/src/get_length_prefixed_slice.rs ]
crate::ix!();

pub fn get_length_prefixed_slice(
    input: *mut Slice,
    result: *mut Slice,
) -> bool {
    unsafe {
        trace!(
            input_ptr = ?input,
            "get_length_prefixed_slice: starting decode from Slice"
        );

        let mut len32: u32 = 0;
        if !get_varint32(input, &mut len32 as *mut u32) {
            warn!(
                "get_length_prefixed_slice: failed to decode length varint32 from Slice"
            );
            return false;
        }

        let len = len32 as usize;
        let remaining_before = *(*input).size();

        if remaining_before < len {
            warn!(
                requested_len = len,
                remaining_before,
                "get_length_prefixed_slice: Slice shorter than declared length"
            );
            return false;
        }

        let base = *(*input).data();
        *result = Slice::from_ptr_len(base, len);
        (*input).remove_prefix(len);

        debug!(
            len,
            remaining_after = *(*input).size(),
            "get_length_prefixed_slice: successfully decoded length-prefixed slice"
        );
        true
    }
}

pub fn get_length_prefixed_slice_with_limit(
    p: *const u8,
    limit: *const u8,
    result: *mut Slice,
) -> *const u8 {
    unsafe {
        trace!(
            ptr = ?p,
            limit = ?limit,
            "get_length_prefixed_slice_with_limit: starting decode"
        );
        let mut len32: u32 = 0;
        let mut current = get_varint_32ptr(p, limit, &mut len32 as *mut u32);
        if current.is_null() {
            warn!(
                "get_length_prefixed_slice_with_limit: failed to decode length varint32"
            );
            return core::ptr::null();
        }

        let len = len32 as usize;
        let remaining = (limit as usize).wrapping_sub(current as usize);
        if remaining < len {
            warn!(
                len,
                remaining,
                "get_length_prefixed_slice_with_limit: not enough bytes for slice"
            );
            return core::ptr::null();
        }

        *result = Slice::from_ptr_len(current, len);
        let next = current.add(len);
        trace!(
            len,
            next = ?next,
            "get_length_prefixed_slice_with_limit: successfully decoded slice"
        );
        next
    }
}
