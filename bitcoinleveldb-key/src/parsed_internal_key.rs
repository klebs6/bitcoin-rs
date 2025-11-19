// ---------------- [ File: bitcoinleveldb-key/src/parsed_internal_key.rs ]
crate::ix!();

#[derive(Setters,Getters)]
#[getset(get="pub",set="pub")]
pub struct ParsedInternalKey {
    user_key: Slice,
    sequence: SequenceNumber,
    ty:       ValueType,
}

impl Default for ParsedInternalKey {

    fn default() -> Self {
        trace!("ParsedInternalKey::default");
        unsafe {
            ParsedInternalKey {
                user_key: Slice::from_ptr_len(EMPTY_SLICE_DATA.as_ptr(), 0),
                sequence: 0,
                ty:       VALUE_TYPE_FOR_SEEK,
            }
        }
    }
}

impl ParsedInternalKey {

    pub fn new(u: &Slice, seq: &SequenceNumber, t: ValueType) -> Self {
        trace!(
            "ParsedInternalKey::new: seq={}, ty={:?}, user_key_len={}",
            *seq,
            t,
            *u.size()
        );
        unsafe {
            ParsedInternalKey {
                user_key: Slice::from_ptr_len(*u.data(), *u.size()),
                sequence: *seq,
                ty:       t,
            }
        }
    }

    pub fn debug_string(&self) -> String {
        debug!(
            "ParsedInternalKey::debug_string: seq={}, ty={:?}",
            self.sequence,
            self.ty
        );
        let bytes = slice_as_bytes(&self.user_key);
        let escaped = escape_for_debug(bytes);
        format!("'{}' @ {} : {}", escaped, self.sequence, self.ty as u8)
    }
}

/**
  | Return the length of the encoding of
  | "key".
  |
  */
#[inline]
pub fn internal_key_encoding_length(k: &ParsedInternalKey) -> usize {
    let user_len = *k.user_key.size();
    let len = user_len + 8;
    trace!(
        "internal_key_encoding_length: user_key_len={} total_len={}",
        user_len,
        len
    );
    len
}

/**
  | Attempt to parse an internal key from
  | "internal_key".  On success, stores the parsed
  | data in "*result", and returns true.
  |
  | On error, returns false, leaves "*result" in an
  | undefined state.
  */
#[inline]
pub fn parse_internal_key(
    internal_key_: &Slice,
    result: *mut ParsedInternalKey,
) -> bool {
    let n = *internal_key_.size();
    trace!("parse_internal_key: len={}", n);
    if n < 8 {
        debug!(
            "parse_internal_key: too short ({} bytes)",
            n
        );
        return false;
    }

    assert!(
        !result.is_null(),
        "parse_internal_key: result pointer must not be null"
    );

    unsafe {
        let data = *internal_key_.data();
        let num = decode_fixed64_le(data.add(n - 8));
        let c = (num & 0xff) as u8;
        let seq = num >> 8;

        match ValueType::from_tag(c) {
            Some(ty) if (c as u64) <= (VALUE_TYPE_FOR_SEEK as u64) => {
                (*result).sequence = seq;
                (*result).ty = ty;
                (*result).user_key = Slice::from_ptr_len(data, n - 8);
                true
            }
            _ => {
                debug!(
                    "parse_internal_key: invalid tag={}, seq={}",
                    c,
                    seq
                );
                false
            }
        }
    }
}

/**
  | Append the serialization of "key" to
  | *result.
  |
  */
pub fn append_internal_key(result: *mut String, k: &ParsedInternalKey) {
    trace!(
        "append_internal_key: seq={}, ty={:?}, user_key_len={}",
        k.sequence,
        k.ty,
        *k.user_key.size()
    );
    assert!(
        !result.is_null(),
        "append_internal_key: result pointer must not be null"
    );

    unsafe {
        let s: &mut String = &mut *result;
        let vec: &mut Vec<u8> = s.as_mut_vec();
        let user_slice = &k.user_key;
        let key_len = *user_slice.size();

        if key_len > 0 {
            let data = *user_slice.data();
            let bytes = std::slice::from_raw_parts(data, key_len);
            vec.extend_from_slice(bytes);
        }

        let packed = pack_sequence_and_type(k.sequence, k.ty);
        let tag = encode_fixed64_le(packed);
        vec.extend_from_slice(&tag);
    }
}

#[cfg(test)]
mod parsed_internal_key_tests {
    use super::*;

    fn build_internal_key_bytes_with_tag(
        user: &[u8],
        seq: SequenceNumber,
        raw_tag: u8,
    ) -> Vec<u8> {
        let mut v = Vec::with_capacity(user.len() + 8);
        v.extend_from_slice(user);
        let combined = (seq << 8) | (raw_tag as u64);
        let enc = encode_fixed64_le(combined);
        v.extend_from_slice(&enc);
        v
    }

    #[traced_test]
    fn parse_internal_key_rejects_too_short_input() {
        let buf = [0u8; 4];
        unsafe {
            let mut parsed = ParsedInternalKey::default();
            let s = Slice::from_ptr_len(buf.as_ptr(), buf.len());
            let ok = parse_internal_key(&s, &mut parsed as *mut ParsedInternalKey);
            assert!(!ok, "parse_internal_key must reject keys shorter than 8 bytes");
        }
    }

    #[traced_test]
    fn parse_internal_key_rejects_invalid_value_type_tag() {
        let user = b"xyz";
        let seq: SequenceNumber = 7;
        let invalid_tag: u8 = 0x02; // not a valid ValueType

        let data = build_internal_key_bytes_with_tag(user, seq, invalid_tag);
        unsafe {
            let mut parsed = ParsedInternalKey::default();
            let s = Slice::from_ptr_len(data.as_ptr(), data.len());
            let ok = parse_internal_key(&s, &mut parsed as *mut ParsedInternalKey);
            assert!(
                !ok,
                "parse_internal_key must fail for an invalid value-type tag"
            );
        }
    }

    #[traced_test]
    fn internal_key_encoding_length_matches_user_key_length_plus_eight() {
        let s = "hello";
        let seq: SequenceNumber = 1234;
        unsafe {
            let user_slice = Slice::from_ptr_len(s.as_ptr(), s.len());
            let parsed = ParsedInternalKey::new(
                &user_slice,
                &seq,
                ValueType::TypeValue,
            );
            let len = internal_key_encoding_length(&parsed);
            assert_eq!(
                len,
                s.len() + 8,
                "encoding length must be user length + 8"
            );
        }
    }
}
