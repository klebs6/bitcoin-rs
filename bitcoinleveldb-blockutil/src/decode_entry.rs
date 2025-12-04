// ---------------- [ File: bitcoinleveldb-blockutil/src/decode_entry.rs ]
crate::ix!();

/// Helper routine: decode the next block entry starting at "p", storing the
/// number of shared key bytes, non_shared key bytes, and the length of the
/// value in "*shared", "*non_shared", and "*value_length", respectively.  Will
/// not dereference past "limit".
/// 
/// If any errors are detected, returns nullptr. Otherwise, returns a pointer to
/// the key delta (just past the three decoded values).
///
#[inline]
pub fn decode_entry(
    p: *const u8,
    limit: *const u8,
    shared: *mut u32,
    non_shared: *mut u32,
    value_length: *mut u32,
) -> *const u8 {
    unsafe {
        let available = limit.offset_from(p);
        trace!(
            "decode_entry: p={:?}, limit={:?}, available={}",
            p,
            limit,
            available
        );

        if available < 3 {
            debug!(
                "decode_entry: not enough bytes for header (available={})",
                available
            );
            return core::ptr::null();
        }

        *shared = *p.add(0) as u32;
        *non_shared = *p.add(1) as u32;
        *value_length = *p.add(2) as u32;

        trace!(
            "decode_entry: fast-path header shared={}, non_shared={}, value_length={}",
            *shared,
            *non_shared,
            *value_length
        );

        let mut cur = p;

        if (*shared | *non_shared | *value_length) < 128 {
            // Fast path: all three values fit in one byte each.
            cur = cur.add(3);
        } else {
            cur = get_varint32_ptr(cur, limit, shared);
            if cur.is_null() {
                debug!("decode_entry: failed to decode shared varint");
                return core::ptr::null();
            }

            cur = get_varint32_ptr(cur, limit, non_shared);
            if cur.is_null() {
                debug!("decode_entry: failed to decode non_shared varint");
                return core::ptr::null();
            }

            cur = get_varint32_ptr(cur, limit, value_length);
            if cur.is_null() {
                debug!("decode_entry: failed to decode value_length varint");
                return core::ptr::null();
            }

            trace!(
                "decode_entry: varint header shared={}, non_shared={}, value_length={}",
                *shared,
                *non_shared,
                *value_length
            );
        }

        let remaining = limit.offset_from(cur);
        let needed = (*non_shared + *value_length) as isize;

        if remaining < needed {
            debug!(
                "decode_entry: truncated entry (remaining={}, needed={})",
                remaining,
                needed
            );
            return core::ptr::null();
        }

        trace!(
            "decode_entry: success; key_delta_ptr={:?}, non_shared={}, value_length={}",
            cur,
            *non_shared,
            *value_length
        );

        cur
    }
}

#[cfg(test)]
mod decode_entry_behavior_tests {
    use super::*;

    fn encode_varint32(mut v: u32) -> Vec<u8> {
        let mut out = Vec::new();
        while v >= 0x80 {
            out.push(((v & 0x7f) as u8) | 0x80);
            v >>= 7;
        }
        out.push(v as u8);
        out
    }

    #[traced_test]
    fn decode_entry_fast_path_succeeds_and_advances_pointer() {
        let mut shared: u32 = 0;
        let mut non_shared: u32 = 0;
        let mut value_length: u32 = 0;

        // Header: single‑byte values (all < 128), followed by key/value bytes.
        let mut buf = vec![
            1u8,  // shared
            2u8,  // non_shared
            3u8,  // value_length
            0xaa, 0xbb, // key delta (non_shared == 2)
            0xcc, 0xdd, 0xee, // value (value_length == 3)
        ];

        let p = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        let delta_ptr = unsafe {
            decode_entry(
                p,
                limit,
                &mut shared as *mut u32,
                &mut non_shared as *mut u32,
                &mut value_length as *mut u32,
            )
        };

        assert!(
            !delta_ptr.is_null(),
            "decode_entry_fast_path_succeeds_and_advances_pointer: expected non‑null pointer"
        );
        assert_eq!(shared, 1);
        assert_eq!(non_shared, 2);
        assert_eq!(value_length, 3);

        let expected_delta_ptr = unsafe { p.add(3) };
        assert_eq!(
            delta_ptr, expected_delta_ptr,
            "decode_entry_fast_path_succeeds_and_advances_pointer: unexpected delta pointer"
        );

        unsafe {
            let key_bytes =
                core::slice::from_raw_parts(delta_ptr, non_shared as usize);
            let value_bytes = core::slice::from_raw_parts(
                delta_ptr.add(non_shared as usize),
                value_length as usize,
            );

            assert_eq!(key_bytes, &[0xaa, 0xbb]);
            assert_eq!(value_bytes, &[0xcc, 0xdd, 0xee]);
        }
    }

    #[traced_test]
    fn decode_entry_with_too_short_header_returns_null() {
        let mut shared: u32 = 0;
        let mut non_shared: u32 = 0;
        let mut value_length: u32 = 0;

        // Only 2 bytes, less than the 3 required for the fixed header fast path.
        let buf = vec![1u8, 2u8];
        let p = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        let delta_ptr = unsafe {
            decode_entry(
                p,
                limit,
                &mut shared as *mut u32,
                &mut non_shared as *mut u32,
                &mut value_length as *mut u32,
            )
        };

        assert!(
            delta_ptr.is_null(),
            "decode_entry_with_too_short_header_returns_null: expected null pointer for truncated header"
        );
    }

    #[traced_test]
    fn decode_entry_varint_header_round_trip_succeeds() {
        let mut shared_out: u32 = 0;
        let mut non_shared_out: u32 = 0;
        let mut value_length_out: u32 = 0;

        let shared: u32 = 300;
        let non_shared: u32 = 400;
        let value_length: u32 = 500;

        let key_bytes = vec![0x11u8; non_shared as usize];
        let value_bytes = vec![0x22u8; value_length as usize];

        let mut buf = Vec::<u8>::new();
        buf.extend(encode_varint32(shared));
        buf.extend(encode_varint32(non_shared));
        buf.extend(encode_varint32(value_length));
        let header_len = buf.len();
        buf.extend_from_slice(&key_bytes);
        buf.extend_from_slice(&value_bytes);

        let p = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        let delta_ptr = unsafe {
            decode_entry(
                p,
                limit,
                &mut shared_out as *mut u32,
                &mut non_shared_out as *mut u32,
                &mut value_length_out as *mut u32,
            )
        };

        assert!(
            !delta_ptr.is_null(),
            "decode_entry_varint_header_round_trip_succeeds: expected non‑null pointer"
        );
        assert_eq!(shared_out, shared);
        assert_eq!(non_shared_out, non_shared);
        assert_eq!(value_length_out, value_length);

        let expected_delta_ptr = unsafe { p.add(header_len) };
        assert_eq!(
            delta_ptr, expected_delta_ptr,
            "decode_entry_varint_header_round_trip_succeeds: unexpected delta pointer"
        );

        unsafe {
            let key_slice = core::slice::from_raw_parts(
                delta_ptr,
                non_shared_out as usize,
            );
            let value_slice = core::slice::from_raw_parts(
                delta_ptr.add(non_shared_out as usize),
                value_length_out as usize,
            );

            assert_eq!(key_slice, &key_bytes[..]);
            assert_eq!(value_slice, &value_bytes[..]);
        }
    }

    #[traced_test]
    fn decode_entry_truncated_payload_returns_null() {
        let mut shared_out: u32 = 0;
        let mut non_shared_out: u32 = 0;
        let mut value_length_out: u32 = 0;

        let shared: u32 = 10;
        let non_shared: u32 = 5;
        let value_length: u32 = 7;

        let key_bytes = vec![0x33u8; non_shared as usize];
        let value_bytes = vec![0x44u8; value_length as usize];

        let mut buf = Vec::<u8>::new();
        buf.push(shared as u8);
        buf.push(non_shared as u8);
        buf.push(value_length as u8);
        buf.extend_from_slice(&key_bytes);
        // Intentionally truncate value bytes: drop the last one.
        buf.extend_from_slice(&value_bytes[..value_bytes.len() - 1]);

        let p = buf.as_ptr();
        let limit = unsafe { p.add(buf.len()) };

        let delta_ptr = unsafe {
            decode_entry(
                p,
                limit,
                &mut shared_out as *mut u32,
                &mut non_shared_out as *mut u32,
                &mut value_length_out as *mut u32,
            )
        };

        assert!(
            delta_ptr.is_null(),
            "decode_entry_truncated_payload_returns_null: expected null pointer for truncated payload"
        );
    }
}
