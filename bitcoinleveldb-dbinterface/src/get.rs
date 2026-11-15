// ---------------- [ File: bitcoinleveldb-dbinterface/src/get.rs ]
crate::ix!();

pub trait DBGet {
    /// Read a value as UTF-8 text. Use `get_bytes` for arbitrary stored bytes.
    /// On a missing key, an error, or invalid UTF-8, leave `*value` unchanged.
    /// A non-null destination must point to a live exclusively writable String.
    /// Legacy implementations may override this text-only method.
    fn get(&mut self, options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
        if value.is_null() {
            return crate::Status::invalid_argument(&Slice::from("get: null text output"), None);
        }
        let mut bytes = Vec::new();
        let status = self.get_bytes(options, key_, &mut bytes);
        if !status.is_ok() {
            return status;
        }
        match String::from_utf8(bytes) {
            Ok(text) => {
                unsafe { *value = text; }
                status
            }
            Err(_) => crate::Status::corruption(&Slice::from("get: value is not valid UTF-8"), None),
        }
    }

    /// Read a value without text conversion. On OK, replace `*value` exactly;
    /// on NotFound or another error, leave it unchanged. A non-null output must
    /// point to a live exclusively writable Vec<u8>.
    ///
    /// The default explicitly reports NotSupported. It does not route arbitrary
    /// bytes through a legacy text implementation. DBImpl implements this path;
    /// a legacy implementation must opt in before claiming binary retrieval.
    fn get_bytes(&mut self, _options: &ReadOptions, _key_: &Slice, _value: *mut Vec<u8>) -> crate::Status {
        crate::Status::not_supported(&Slice::from("get_bytes is not implemented"), None)
    }
}

#[cfg(test)]
mod get_contract_semantics_suite {
    use super::*;
    use std::collections::BTreeMap;
    use tracing::{debug, info, trace};

    fn slice_to_vec(s: &Slice) -> Vec<u8> {
        let p = *s.data();
        let n = *s.size();
        unsafe { core::slice::from_raw_parts(p, n) }.to_vec()
    }

    fn bytes_to_string(bytes: &[u8]) -> String {
        Slice::from(bytes).to_string()
    }

    struct SimpleGetDb {
        kv: BTreeMap<Vec<u8>, Vec<u8>>,
    }

    impl SimpleGetDb {
        fn new(pairs: &[(&[u8], &[u8])]) -> Self {
            let mut kv = BTreeMap::new();
            for (k, v) in pairs {
                kv.insert(k.to_vec(), v.to_vec());
            }
            Self { kv }
        }
    }

    impl DBGet for SimpleGetDb {
        fn get(&mut self, _options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
            assert!(!value.is_null(), "value must not be null");

            let k = slice_to_vec(key_);
            trace!(key_len = k.len(), "get() called");

            if let Some(v) = self.kv.get(&k) {
                unsafe {
                    *value = bytes_to_string(v.as_slice());
                }
                debug!(value_len = v.len(), "get() found key and updated value");
                return crate::Status::ok();
            }

            debug!("get() did not find key; leaving *value unchanged");
            return crate::Status::not_found(&Slice::from("not found"), None);
        }
    }

    #[traced_test]
    fn get_found_sets_value_and_returns_ok() {
        let mut db = SimpleGetDb::new(&[(b"a", b"b")]);

        let options = ReadOptions::default();
        let key = Slice::from("a");

        let mut out = Slice::from("unchanged").to_string();

        trace!("calling get() for existing key");
        let s = db.get(&options, &key, &mut out as *mut String);

        assert!(s.is_ok());
        assert_eq!(out, Slice::from("b").to_string());

        info!("verified get() sets *value and returns OK when key exists");
    }

    #[traced_test]
    fn get_not_found_leaves_value_unmodified_and_returns_not_found() {
        let mut db = SimpleGetDb::new(&[]);

        let options = ReadOptions::default();
        let key = Slice::from("missing");

        let unchanged = Slice::from("unchanged").to_string();
        let mut out = unchanged.clone();

        trace!("calling get() for missing key");
        let s = db.get(&options, &key, &mut out as *mut String);

        assert!(s.is_not_found());
        assert_eq!(out, unchanged);

        info!("verified get() preserves *value on not found and returns NotFound");
    }

    #[traced_test]
    fn get_allows_empty_key_lookup_and_preserves_value_when_missing() {
        let mut db = SimpleGetDb::new(&[(b"x", b"y")]);

        let options = ReadOptions::default();
        let key = Slice::from("");

        let unchanged = Slice::from("still").to_string();
        let mut out = unchanged.clone();

        trace!("calling get() with empty key");
        let s = db.get(&options, &key, &mut out as *mut String);

        assert!(s.is_not_found());
        assert_eq!(out, unchanged);

        info!("verified get() can be called with empty key and preserves output on NotFound");
    }
}


#[cfg(test)]
mod byte_get_contracts {
    use super::*;
    struct ByteDb { value: Vec<u8>, found: bool, fail: bool }
    impl DBGet for ByteDb {
        fn get_bytes(&mut self, _: &ReadOptions, _: &Slice, out: *mut Vec<u8>) -> crate::Status {
            if self.fail { return crate::Status::io_error(&Slice::from("test read error"), None); }
            if !self.found { return crate::Status::not_found(&Slice::default(), None); }
            unsafe { *out = self.value.clone(); }
            crate::Status::ok()
        }
    }
    struct LegacyTextDb;
    impl DBGet for LegacyTextDb {
        fn get(&mut self, _: &ReadOptions, _: &Slice, out: *mut String) -> crate::Status {
            unsafe { *out = String::from("legacy text"); }
            crate::Status::ok()
        }
    }

    #[test]
    fn strict_text_adapter_preserves_utf8_and_rejects_invalid_values_without_mutation() {
        let options = ReadOptions::default();
        let key = Slice::default();
        let mut db = ByteDb { value: "é\0水".as_bytes().to_vec(), found: true, fail: false };
        let mut text = String::from("sentinel");
        assert!(db.get(&options, &key, &mut text).is_ok());
        assert_eq!(text, "é\0水");
        let binary: Vec<u8> = (0u8..=255).collect();
        db.value = binary.clone();
        let prior = text.clone();
        assert!(db.get(&options, &key, &mut text).is_corruption());
        assert_eq!(text, prior);
        let mut bytes = Vec::new();
        let dynamic: &mut dyn DBGet = &mut db;
        assert!(dynamic.get_bytes(&options, &key, &mut bytes).is_ok());
        assert_eq!(bytes, binary);
        db.found = false;
        assert!(db.get(&options, &key, &mut text).is_not_found());
        assert_eq!(text, prior);
        db.fail = true;
        assert!(!db.get(&options, &key, &mut text).is_ok());
        assert_eq!(text, prior);
    }

    #[test]
    fn legacy_text_only_implementation_reports_missing_byte_capability_explicitly() {
        let mut db = LegacyTextDb;
        let mut out = vec![0xff, 0x00, 0x80];
        let prior = out.clone();
        let dynamic: &mut dyn DBGet = &mut db;
        let status = dynamic.get_bytes(&ReadOptions::default(), &Slice::default(), &mut out);
        assert!(status.is_not_supported_error());
        assert_eq!(out, prior);
    }
}
