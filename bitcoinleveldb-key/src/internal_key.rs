// ---------------- [ File: bitcoinleveldb-key/src/internal_key.rs ]
crate::ix!();

/**
  | Modules in this directory should keep internal
  | keys wrapped inside the following class instead
  | of plain strings so that we do not incorrectly
  | use string comparisons instead of an
  | InternalKeyComparator.
  */
#[derive(Clone)]
pub struct InternalKey {
    rep: String,
}

impl Default for InternalKey {
    /**
      | Leave rep_ as empty to indicate it is
      | invalid
      |
      */
    fn default() -> Self {
        trace!("InternalKey::default");
        InternalKey {
            rep: String::new(),
        }
    }
}

impl core::fmt::Debug for InternalKey {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        trace!(
            "InternalKey::fmt(Debug): rep_len={}",
            self.rep.len()
        );
        // Use the safe, structured debug representation instead of raw bytes-as-UTF8.
        let repr = self.debug_string();
        f.debug_struct("InternalKey")
            .field("repr", &repr)
            .finish()
    }
}


impl InternalKey {

    pub fn new(user_key_: &Slice, s: SequenceNumber, t: ValueType) -> Self {
        trace!(
            "InternalKey::new: seq={}, ty={:?}, user_key_len={}",
            s,
            t,
            *user_key_.size()
        );
        let mut rep = String::new();
        let parsed = ParsedInternalKey::new(user_key_, &s, t);
        append_internal_key(&mut rep as *mut String, &parsed);
        InternalKey { rep }
    }

    pub fn decode_from(&mut self, s: &Slice) -> bool {
        trace!(
            "InternalKey::decode_from: slice_len={}",
            *s.size()
        );
        unsafe {
            let len  = *s.size();
            let data = *s.data();
            if len == 0 {
                self.rep.clear();
                return false;
            }
            let bytes = std::slice::from_raw_parts(data, len);
            // NOTE: bytes here are LevelDB internal-key bytes; we preserve them as-is.
            // This is a faithful port of the C++ "string of bytes" representation.
            self.rep = String::from_utf8_unchecked(bytes.to_vec());
        }
        !self.rep.is_empty()
    }

    pub fn encode(&self) -> Slice {
        debug!("InternalKey::encode: len={}", self.rep.len());
        assert!(
            !self.rep.is_empty(),
            "InternalKey::encode called on empty key"
        );
        unsafe { Slice::from_ptr_len(self.rep.as_ptr(), self.rep.len()) }
    }

    pub fn user_key(&self) -> Slice {
        let encoded = self.encode();
        extract_user_key(&encoded)
    }

    pub fn set_from(&mut self, p: &ParsedInternalKey) {
        trace!(
            "InternalKey::set_from: seq={}, ty={:?}",
            *p.sequence(),
            *p.ty()
        );
        self.rep.clear();
        append_internal_key(&mut self.rep as *mut String, p);
    }

    pub fn clear(&mut self) {
        trace!("InternalKey::clear");
        self.rep.clear();
    }

    pub fn debug_string(&self) -> String {
        debug!(
            "InternalKey::debug_string: rep_len={}",
            self.rep.len()
        );
        let mut parsed = ParsedInternalKey::default();

        let ok = unsafe {
            let data = if self.rep.is_empty() {
                EMPTY_SLICE_DATA.as_ptr()
            } else {
                self.rep.as_ptr()
            };
            let len   = self.rep.len();
            let slice = Slice::from_ptr_len(data, len);
            parse_internal_key(&slice, &mut parsed as *mut ParsedInternalKey)
        };

        if ok {
            parsed.debug_string()
        } else {
            let escaped = escape_for_debug(self.rep.as_bytes());
            format!("(bad){}", escaped)
        }
    }
}

#[cfg(test)]
mod internal_key_tests {
    use super::*;

    #[traced_test]
    fn internal_key_roundtrip_for_arbitrary_user_bytes() {
        let user_bytes = [0xffu8, 0x00u8, 0x41u8];
        let seq: SequenceNumber = 9999;
        let ty = ValueType::TypeValue;

        let key = unsafe {
            let user_slice =
                Slice::from_ptr_len(user_bytes.as_ptr(), user_bytes.len());
            InternalKey::new(&user_slice, seq, ty)
        };

        let encoded = key.encode();
        unsafe {
            let encoded_ptr = *encoded.data();
            let encoded_len = *encoded.size();
            let encoded_bytes =
                std::slice::from_raw_parts(encoded_ptr, encoded_len);

            let mut decoded = InternalKey::default();
            let encoded_slice =
                Slice::from_ptr_len(encoded_bytes.as_ptr(), encoded_bytes.len());
            let ok = decoded.decode_from(&encoded_slice);
            assert!(ok, "decode_from should succeed on encoded internal key");

            let decoded_user_slice = decoded.user_key();
            let decoded_user_bytes = slice_as_bytes(&decoded_user_slice);
            assert_eq!(decoded_user_bytes, &user_bytes[..]);
        }
    }

    #[traced_test]
    fn internal_key_clear_makes_debug_string_bad() {
        let s = "abc";
        let seq: SequenceNumber = 5;
        let mut key = unsafe {
            let user_slice = Slice::from_ptr_len(s.as_ptr(), s.len());
            InternalKey::new(&user_slice, seq, ValueType::TypeValue)
        };

        key.clear();
        let debug_str = key.debug_string();
        assert!(
            debug_str.starts_with("(bad)"),
            "debug_string should report bad after clear; got {}",
            debug_str
        );
    }

    #[traced_test]
    fn internal_key_debug_does_not_panic_for_binary_representation() {
        trace!(
            "internal_key_debug_does_not_panic_for_binary_representation: start"
        );

        let user = Slice::from("x".as_bytes());
        let key  = InternalKey::new(&user, 123 as SequenceNumber, ValueType::TypeValue);

        // This used to panic because Debug tried to print raw internal bytes as UTF-8.
        let debug_str = format!("{:?}", key);

        debug!(%debug_str, "internal_key_debug_does_not_panic_for_binary_representation: Debug output");

        assert!(
            debug_str.contains("InternalKey"),
            "Debug output for InternalKey should contain type name"
        );
    }
}
