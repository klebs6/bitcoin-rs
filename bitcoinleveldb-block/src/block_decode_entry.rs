// ---------------- [ File: bitcoinleveldb-block/src/block_decode_entry.rs ]
crate::ix!();

#[inline]
pub fn decode_entry(
    mut p:            *const u8,
    limit:            *const u8,
    shared:           *mut u32,
    non_shared:       *mut u32,
    value_length:     *mut u32,
) -> *const u8 {
    unsafe {
        let remaining_header = limit.offset_from(p);
        if remaining_header < 3 {
            trace!(
                "decode_entry: insufficient bytes for header (remaining={})",
                remaining_header
            );
            return core::ptr::null();
        }

        let a = *p as u32;
        let b = *p.add(1) as u32;
        let c = *p.add(2) as u32;

        *shared       = a;
        *non_shared   = b;
        *value_length = c;

        if (*shared | *non_shared | *value_length) < 128 {
            // Fast path: all three fields fit in a single byte.
            p = p.add(3);
        } else {
            p = get_varint_32ptr(p, limit, shared);
            if p.is_null() {
                trace!("decode_entry: failed to decode shared length");
                return core::ptr::null();
            }

            p = get_varint_32ptr(p, limit, non_shared);
            if p.is_null() {
                trace!("decode_entry: failed to decode non_shared length");
                return core::ptr::null();
            }

            p = get_varint_32ptr(p, limit, value_length);
            if p.is_null() {
                trace!("decode_entry: failed to decode value_length");
                return core::ptr::null();
            }
        }

        let needed: u32 = (*non_shared).saturating_add(*value_length);
        let remaining_payload = limit.offset_from(p);

        if remaining_payload < needed as isize {
            trace!(
                "decode_entry: truncated entry (needed={}, remaining={})",
                needed,
                remaining_payload
            );
            return core::ptr::null();
        }

        p
    }
}

#[cfg(test)]
mod block_decode_entry_tests {
    use super::*;

    #[traced_test]
    fn decode_entry_fast_path_with_small_header_values() {
        let shared: u8       = 1;
        let non_shared: u8   = 2;
        let value_len: u8    = 3;
        let key_bytes        = [b'a', b'b'];
        let value_bytes      = [b'X', b'Y', b'Z'];

        let mut buf = Vec::<u8>::new();
        buf.push(shared);
        buf.push(non_shared);
        buf.push(value_len);
        buf.extend_from_slice(&key_bytes);
        buf.extend_from_slice(&value_bytes);

        let mut shared_out: u32       = 0;
        let mut non_shared_out: u32   = 0;
        let mut value_len_out: u32    = 0;

        let p = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        trace!(
            "calling decode_entry on fast-path buffer len={}",
            buf.len()
        );

        let key_ptr = decode_entry(
            p,
            limit,
            &mut shared_out,
            &mut non_shared_out,
            &mut value_len_out,
        );

        assert!(!key_ptr.is_null());
        assert_eq!(shared_out, shared as u32);
        assert_eq!(non_shared_out, non_shared as u32);
        assert_eq!(value_len_out, value_len as u32);

        unsafe {
            let key_delta = core::slice::from_raw_parts(key_ptr, non_shared_out as usize);
            let value     = core::slice::from_raw_parts(key_ptr.add(non_shared_out as usize), value_len_out as usize);

            debug!("decoded key_delta={:?}, value={:?}", key_delta, value);
            assert_eq!(key_delta, &key_bytes);
            assert_eq!(value, &value_bytes);
        }
    }

    #[traced_test]
    fn decode_entry_returns_null_on_truncated_payload() {
        let shared: u8     = 1;
        let non_shared: u8 = 4;
        let value_len: u8  = 10;

        let mut buf = Vec::<u8>::new();
        buf.push(shared);
        buf.push(non_shared);
        buf.push(value_len);
        // No payload bytes – deliberately truncated.

        let mut shared_out: u32       = 0;
        let mut non_shared_out: u32   = 0;
        let mut value_len_out: u32    = 0;

        let p     = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        trace!("calling decode_entry on truncated buffer");
        let key_ptr = decode_entry(
            p,
            limit,
            &mut shared_out,
            &mut non_shared_out,
            &mut value_len_out,
        );

        assert!(key_ptr.is_null());
    }
}
