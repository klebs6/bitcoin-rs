// ---------------- [ File: bitcoinleveldb-key/src/encode.rs ]
crate::ix!();

pub static EMPTY_SLICE_DATA: [u8; 0] = [];

pub fn encode_fixed64_le(value: u64) -> [u8; 8] {
    let mut buf = [0u8; 8];
    for i in 0..8 {
        buf[i] = ((value >> (8 * i)) & 0xff) as u8;
    }
    buf
}

pub unsafe fn decode_fixed64_le(ptr: *const u8) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let b = *ptr.add(i) as u64;
        result |= b << (8 * i);
    }
    result
}

pub fn put_varint32_vec(dst: &mut Vec<u8>, mut v: u32) {
    while v >= 0x80 {
        dst.push(((v & 0x7f) as u8) | 0x80);
        v >>= 7;
    }
    dst.push(v as u8);
}

pub fn slice_as_bytes(s: &Slice) -> &[u8] {
    unsafe {
        let len = *s.size();
        if len == 0 {
            &[]
        } else {
            let data = *s.data();
            std::slice::from_raw_parts(data, len)
        }
    }
}

pub fn bytewise_compare(a: &[u8], b: &[u8]) -> i32 {
    use core::cmp::Ordering;
    match a.cmp(b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub fn shorten_separator_user_keys(start: &[u8], limit: &[u8]) -> Option<Vec<u8>> {
    use core::cmp::min;

    let min_len = min(start.len(), limit.len());
    let mut diff_index = 0usize;

    while diff_index < min_len && start[diff_index] == limit[diff_index] {
        diff_index += 1;
    }

    if diff_index >= min_len {
        return None;
    }

    let diff_byte = start[diff_index];
    let limit_byte = limit[diff_index];

    if diff_byte < 0xff && diff_byte + 1 < limit_byte {
        let mut tmp = Vec::with_capacity(diff_index + 1);
        tmp.extend_from_slice(&start[..diff_index]);
        tmp.push(diff_byte + 1);
        Some(tmp)
    } else {
        None
    }
}

pub fn find_short_successor_user_key(key: &mut Vec<u8>) -> bool {
    for i in 0..key.len() {
        if key[i] != 0xff {
            key[i] = key[i].wrapping_add(1);
            key.truncate(i + 1);
            return true;
        }
    }
    false
}

pub fn escape_for_debug(input: &[u8]) -> String {
    let mut out = String::new();
    for &b in input {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'"'  => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(b as char),
            _ => {
                out.push_str("\\x");
                out.push_str(&format!("{:02x}", b));
            }
        }
    }
    out
}

/// Helper: construct a null trait-object pointer to a SliceComparator.
///
/// We never dereference this; it is used only to exercise the
/// "no user comparator provided" code paths.
pub fn null_slice_comparator() -> *const dyn SliceComparator {
    unsafe {
        std::mem::transmute::<(usize, usize), *const dyn SliceComparator>((0, 0))
    }
}

/// Helper: decode a varint32 from a byte slice.
/// Returns (value, bytes_consumed).
pub fn decode_varint32(mut src: &[u8]) -> (u32, usize) {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    let mut consumed: usize = 0;

    loop {
        assert!(
            !src.is_empty(),
            "decode_varint32: input exhausted before termination bit"
        );
        let b = src[0] as u32;
        src = &src[1..];
        consumed += 1;

        result |= (b & 0x7f) << shift;
        if (b & 0x80) == 0 {
            break;
        }
        shift += 7;
        assert!(shift < 32, "decode_varint32: shift overflow");
    }

    (result, consumed)
}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[traced_test]
    fn encode_decode_fixed64_roundtrip_for_key_patterns() {
        let values: [u64; 6] = [
            0,
            1,
            0x0123_4567_89ab_cdef,
            0xffff_ffff_ffff_ffff,
            0x0000_0000_0000_00ff,
            0xff00_ff00_ff00_ff00,
        ];

        for &v in &values {
            trace!("encode_decode_fixed64_roundtrip: v={:#x}", v);
            let enc = encode_fixed64_le(v);
            unsafe {
                let dec = decode_fixed64_le(enc.as_ptr());
                assert_eq!(
                    v, dec,
                    "roundtrip mismatch for value {:#x}",
                    v
                );
            }
        }
    }

    #[traced_test]
    fn encode_fixed64_le_has_correct_little_endian_layout() {
        let v: u64 = 0x01_02_03_04_05_06_07_08;
        let enc = encode_fixed64_le(v);
        debug!("encode_fixed64_le layout: {:?}", enc);
        assert_eq!(
            enc,
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
            "encode_fixed64_le must store bytes in little-endian order"
        );
    }

    #[traced_test]
    fn put_varint32_encodes_and_decodes_expected_values() {
        let values: [u32; 7] = [
            0,
            1,
            127,
            128,
            255,
            300,
            0x7fff_ffff,
        ];

        for &v in &values {
            trace!("put_varint32 case: v={}", v);
            let mut buf = Vec::new();
            put_varint32_vec(&mut buf, v);

            let (decoded, consumed) = decode_varint32(&buf);
            assert_eq!(decoded, v, "varint32 roundtrip mismatch for {}", v);
            assert_eq!(
                consumed,
                buf.len(),
                "decoder should consume exactly the buffer for {}",
                v
            );
        }
    }

    #[traced_test]
    fn slice_as_bytes_for_empty_and_non_empty() {
        let data = b"hello world";
        unsafe {
            let empty_slice = Slice::from_ptr_len(data.as_ptr(), 0);
            let non_empty_slice = Slice::from_ptr_len(data.as_ptr(), data.len());

            let empty_bytes = slice_as_bytes(&empty_slice);
            let non_empty_bytes = slice_as_bytes(&non_empty_slice);

            assert!(empty_bytes.is_empty());
            assert_eq!(non_empty_bytes, &data[..]);
        }
    }

    #[traced_test]
    fn bytewise_compare_orders_lexicographically() {
        let a = b"abc";
        let b = b"abd";
        let c = b"abc";

        assert_eq!(bytewise_compare(a, b), -1);
        assert_eq!(bytewise_compare(b, a), 1);
        assert_eq!(bytewise_compare(a, c), 0);

        let empty: &[u8] = &[];
        assert_eq!(bytewise_compare(empty, a), -1);
        assert_eq!(bytewise_compare(a, empty), 1);
        assert_eq!(bytewise_compare(empty, empty), 0);
    }

    #[traced_test]
    fn shorten_separator_user_keys_edge_cases() {
        // Case where shortening is not possible because diff_byte+1 == limit_byte
        let s = b"foo";
        let l = b"g";
        let shortened = shorten_separator_user_keys(s, l);
        assert!(
            shortened.is_none(),
            "no valid shortened separator should exist between {:?} and {:?}",
            s,
            l
        );

        // Case where shortening is possible: "aaa" .. "aaz" -> "aab"
        let s2 = b"aaa";
        let l2 = b"aaz";
        let shortened2 = shorten_separator_user_keys(s2, l2)
            .expect("should find a shorter separator");
        assert_eq!(
            shortened2,
            b"aab".to_vec(),
            "shortened separator for 'aaa'..'aaz' should be 'aab'"
        );
    }

    #[traced_test]
    fn find_short_successor_user_key_various_patterns() {
        // Case: simple increment
        let mut key1 = vec![0x00];
        let changed1 = find_short_successor_user_key(&mut key1);
        assert!(changed1);
        assert_eq!(key1, vec![0x01]);

        // Case: increment first non-0xff and truncate
        let mut key2 = vec![0xff, 0x10, 0x20];
        let changed2 = find_short_successor_user_key(&mut key2);
        assert!(changed2);
        assert_eq!(key2, vec![0xff, 0x11]);

        // Case: all 0xff -> no change
        let mut key3 = vec![0xff, 0xff];
        let changed3 = find_short_successor_user_key(&mut key3);
        assert!(!changed3);
        assert_eq!(key3, vec![0xff, 0xff]);
    }

    #[traced_test]
    fn escape_for_debug_escapes_control_and_special_characters() {
        let input = [
            b'\\',
            b'"',
            b'\n',
            b'\r',
            b'\t',
            0x01,
            b'A',
            0x7f,
        ];
        let escaped = escape_for_debug(&input);
        debug!("escape_for_debug output: {}", escaped);

        assert!(escaped.contains("\\\\"));
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\n"));
        assert!(escaped.contains("\\r"));
        assert!(escaped.contains("\\t"));
        assert!(escaped.contains("\\x01"));
        assert!(escaped.contains("A"));
        assert!(escaped.contains("\\x7f"));
    }
}
