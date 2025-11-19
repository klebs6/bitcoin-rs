// ---------------- [ File: bitcoinleveldb-key/src/extract_user_key.rs ]
crate::ix!();

/**
  | Returns the user key portion of an internal
  | key.
  |
  */
#[inline]
pub fn extract_user_key(internal_key_: &Slice) -> Slice {
    let n = *internal_key_.size();
    trace!("extract_user_key: internal_len={}", n);
    assert!(n >= 8, "internal key too short: {}", n);
    unsafe {
        let data = *internal_key_.data();
        Slice::from_ptr_len(data, n - 8)
    }
}

#[cfg(test)]
mod extract_user_key_tests {
    use super::*;

    #[traced_test]
    fn extract_user_key_with_non_empty_user_key() {
        let user = b"abc";
        let seq: SequenceNumber = 100;
        let tag = pack_sequence_and_type(seq, VALUE_TYPE_FOR_SEEK);
        let tag_bytes = encode_fixed64_le(tag);

        let mut internal = Vec::new();
        internal.extend_from_slice(user);
        internal.extend_from_slice(&tag_bytes);

        unsafe {
            let internal_slice = Slice::from_ptr_len(internal.as_ptr(), internal.len());
            let user_slice = extract_user_key(&internal_slice);
            let bytes = slice_as_bytes(&user_slice);
            assert_eq!(bytes, user);
        }
    }

    #[traced_test]
    fn extract_user_key_with_empty_user_key() {
        let seq: SequenceNumber = 1;
        let tag = pack_sequence_and_type(seq, VALUE_TYPE_FOR_SEEK);
        let tag_bytes = encode_fixed64_le(tag);
        let internal = tag_bytes;

        unsafe {
            let internal_slice = Slice::from_ptr_len(internal.as_ptr(), internal.len());
            let user_slice = extract_user_key(&internal_slice);
            let bytes = slice_as_bytes(&user_slice);
            assert!(bytes.is_empty(), "user key should be empty");
        }
    }

    #[test]
    #[should_panic(expected = "internal key too short")]
    fn extract_user_key_panics_when_internal_key_too_short() {
        let buf = [0u8; 7];
        unsafe {
            let internal_slice = Slice::from_ptr_len(buf.as_ptr(), buf.len());
            let _ = extract_user_key(&internal_slice);
        }
    }
}
