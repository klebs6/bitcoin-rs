// ---------------- [ File: bitcoinleveldb-coding/src/get_varint.rs ]
crate::ix!();

/**
  | Pointer-based variants of GetVarint...  These
  | either store a value in *v and return a pointer
  | just past the parsed value, or return nullptr
  | on error.  These routines only look at bytes in
  | the range [p..limit-1]
  */
#[inline]
pub fn get_varint_32ptr(
    p: *const u8,
    limit: *const u8,
    value: *mut u32,
) -> *const u8 {
    unsafe {
        trace!(
            ptr = ?p,
            limit = ?limit,
            "get_varint_32ptr: attempting fast-path varint32 decode"
        );
        if (p as usize) < (limit as usize) {
            let result = *p as u32;
            if (result & 0x80) == 0 {
                *value = result;
                let next = p.add(1);
                trace!(
                    result,
                    next = ?next,
                    "get_varint_32ptr: decoded single-byte varint32"
                );
                return next;
            }
        }
        debug!("get_varint_32ptr: falling back to multi-byte decoder");
        get_varint_32ptr_fallback(p, limit, value)
    }
}

/**
  | Internal routine for use by fallback
  | path of GetVarint32Ptr
  |
  */
pub fn get_varint_32ptr_fallback(
    p: *const u8,
    limit: *const u8,
    value: *mut u32,
) -> *const u8 {
    unsafe {
        trace!(
            ptr = ?p,
            limit = ?limit,
            "get_varint_32ptr_fallback: starting fallback varint32 decode"
        );
        let mut result: u32 = 0;
        let mut shift: u32 = 0;
        let mut current = p;

        while shift <= 28 && (current as usize) < (limit as usize) {
            let byte = *current as u32;
            current = current.add(1);

            if (byte & 0x80) != 0 {
                result |= (byte & 0x7f) << shift;
            } else {
                result |= byte << shift;
                *value = result;
                trace!(
                    result,
                    next = ?current,
                    "get_varint_32ptr_fallback: successfully decoded varint32"
                );
                return current;
            }

            shift += 7;
        }

        warn!(
            "get_varint_32ptr_fallback: reached limit or overflow while decoding varint32"
        );
        core::ptr::null()
    }
}

/**
  | Standard Get... routines parse a value
  | from the beginning of a Slice and advance
  | the slice past the parsed value.
  |
  */
pub fn get_varint32(input: *mut Slice, value: *mut u32) -> bool {
    unsafe {
        trace!(
            input_ptr = ?input,
            "get_varint32: attempting to decode varint32 from Slice"
        );
        let base_ptr: *const u8 = *(*input).data();
        let size: usize = *(*input).size();
        let limit = base_ptr.add(size);

        let p = get_varint_32ptr(base_ptr, limit, value);
        if p.is_null() {
            warn!("get_varint32: decode failed, returning false");
            return false;
        }

        let consumed = p.offset_from(base_ptr) as usize;
        (*input).remove_prefix(consumed);
        debug!(
            consumed,
            remaining = *(*input).size(),
            "get_varint32: successfully decoded varint32 and advanced slice"
        );
        true
    }
}

pub fn get_varint_64ptr(
    p: *const u8,
    limit: *const u8,
    value: *mut u64,
) -> *const u8 {
    unsafe {
        trace!(
            ptr = ?p,
            limit = ?limit,
            "get_varint_64ptr: starting varint64 decode"
        );
        let mut result: u64 = 0;
        let mut shift: u32 = 0;
        let mut current = p;

        while shift <= 63 && (current as usize) < (limit as usize) {
            let byte = *current as u64;
            current = current.add(1);

            if (byte & 0x80) != 0 {
                result |= (byte & 0x7f) << shift;
            } else {
                result |= byte << shift;
                *value = result;
                trace!(
                    result,
                    next = ?current,
                    "get_varint_64ptr: successfully decoded varint64"
                );
                return current;
            }

            shift += 7;
        }

        warn!(
            "get_varint_64ptr: reached limit or overflow while decoding varint64"
        );
        core::ptr::null()
    }
}

pub fn get_varint64(input: *mut Slice, value: *mut u64) -> bool {
    unsafe {
        trace!(
            input_ptr = ?input,
            "get_varint64: attempting to decode varint64 from Slice"
        );
        let base_ptr: *const u8 = *(*input).data();
        let size: usize = *(*input).size();
        let limit = base_ptr.add(size);

        let p = get_varint_64ptr(base_ptr, limit, value);
        if p.is_null() {
            warn!("get_varint64: decode failed, returning false");
            return false;
        }

        let consumed = p.offset_from(base_ptr) as usize;
        (*input).remove_prefix(consumed);
        debug!(
            consumed,
            remaining = *(*input).size(),
            "get_varint64: successfully decoded varint64 and advanced slice"
        );
        true
    }
}

#[inline]
pub fn get_varint64_from_slice(input: &mut Slice, result: &mut u64) -> bool {
    unsafe {
        let mut p = *input.data();
        let limit = p.add(*input.size());

        trace!(
            "get_varint64_from_slice: start ptr={:?}, size={}",
            p,
            input.size()
        );

        let mut shift: u32 = 0;
        let mut value: u64 = 0;
        let mut consumed: usize = 0;

        while p < limit && shift <= 63 {
            let byte = *p;
            p = p.add(1);
            consumed += 1;

            value |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                *result = value;
                input.remove_prefix(consumed);
                trace!(
                    "get_varint64_from_slice: decoded value={}, bytes_used={}",
                    value,
                    consumed
                );
                return true;
            }

            shift += 7;
        }

        debug!(
            "get_varint64_from_slice: failed (consumed={}, shift={}, input_size={})",
            consumed,
            shift,
            input.size()
        );
        false
    }
}

#[inline]
pub unsafe fn get_varint32_ptr(
    mut p: *const u8,
    limit: *const u8,
    result: *mut u32,
) -> *const u8 {
    trace!(
        "get_varint32_ptr: start={:?}, limit={:?}",
        p,
        limit
    );

    let mut shift: u32 = 0;
    let mut value: u32 = 0;

    while p < limit && shift <= 28 {
        let byte = *p;
        p = p.add(1);

        value |= ((byte & 0x7F) as u32) << shift;
        if byte & 0x80 == 0 {
            *result = value;
            trace!(
                "get_varint32_ptr: decoded value={} (bytes_used={}, final_ptr={:?})",
                value,
                (shift / 7) + 1,
                p
            );
            return p;
        }

        shift += 7;
    }

    debug!(
        "get_varint32_ptr: failed to decode varint32 (p={:?}, limit={:?}, shift={})",
        p,
        limit,
        shift
    );
    core::ptr::null()
}
