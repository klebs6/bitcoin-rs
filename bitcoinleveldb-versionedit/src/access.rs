crate::ix!();

pub fn get_level(input: &mut Slice, level: &mut i32) -> bool {
    let mut v: u32 = 0;
    if !get_varint32(input, &mut v) {
        return false;
    }
    *level = v as i32;
    true
}

pub fn get_internal_key(input: &mut Slice, key: &mut InternalKey) -> bool {
    let mut key_slice = Slice::default();
    if !get_length_prefixed_slice(input, &mut key_slice) {
        return false;
    }
    key.decode_from(&key_slice);
    true
}

#[cfg(test)]
mod versionedit_access_tests {
    use super::*;

    #[traced_test]
    fn get_level_decodes_single_byte_varint_and_advances_slice() {
        trace!("get_level_decodes_single_byte_varint_and_advances_slice: start");

        let mut storage = vec![5u8]; // varint32 for value 5
        let mut slice   = Slice::from(storage.as_slice());
        let mut level: i32 = -1;

        let ok = get_level(&mut slice, &mut level);

        assert!(ok, "get_level should succeed for a single-byte varint");
        assert_eq!(level, 5, "decoded level should match encoded value");
        assert_eq!(
            *slice.size(),
            0,
            "slice should have been fully consumed after get_level"
        );
    }

    #[traced_test]
    fn get_level_returns_false_on_empty_input() {
        trace!("get_level_returns_false_on_empty_input: start");

        let mut slice = Slice::default();
        let mut level: i32 = -1;

        let ok = get_level(&mut slice, &mut level);

        assert!(!ok, "get_level should fail on empty input");
        assert_eq!(
            level,
            -1,
            "level should remain unchanged when decoding fails"
        );
    }

    #[traced_test]
    fn get_internal_key_decodes_valid_length_prefixed_internal_key() {
        trace!(
            "get_internal_key_decodes_valid_length_prefixed_internal_key: start"
        );

        const SEQ: SequenceNumber = 42;

        let user = Slice::from("user-key".as_bytes());
        let original = InternalKey::new(&user, SEQ, ValueType::TypeValue);
        let encoded  = original.encode();

        let mut buf = String::new();
        unsafe {
            put_length_prefixed_slice(&mut buf as *mut String, &encoded);
        }

        let mut slice = Slice::from(buf.as_bytes());
        let mut decoded = InternalKey::default();

        let ok = get_internal_key(&mut slice, &mut decoded);

        assert!(ok, "get_internal_key should succeed on well-formed input");
        assert_eq!(
            original.debug_string(),
            decoded.debug_string(),
            "decoded internal key should roundtrip through get_internal_key"
        );
        assert_eq!(
            *slice.size(),
            0,
            "slice should be fully consumed after get_internal_key"
        );
    }

    #[traced_test]
    fn get_internal_key_returns_false_on_truncated_length_prefixed_slice() {
        trace!(
            "get_internal_key_returns_false_on_truncated_length_prefixed_slice: start"
        );

        // Encoded length=3, but only 2 bytes of payload => insufficient bytes.
        let mut storage = vec![3u8, b'a', b'b'];
        let mut slice   = Slice::from(storage.as_slice());
        let mut key     = InternalKey::default();

        let ok = get_internal_key(&mut slice, &mut key);

        assert!(
            !ok,
            "get_internal_key should fail when length-prefixed slice is truncated"
        );
    }
}
