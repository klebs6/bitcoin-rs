// ---------------- [ File: bitcoinleveldb-dbinterface/src/get.rs ]
crate::ix!();

pub trait DBGet {

    /// If the database contains an entry for "key" store the corresponding value in *value and
    /// return OK.
    /// 
    /// If there is no entry for "key" leave *value unchanged and return a status for which
    /// Status::IsNotFound() returns true.
    /// 
    /// May return some other Status on an error.
    ///
    fn get(&mut self, 
            options: &ReadOptions,
            key_:     &Slice,
            value:   *mut String) -> crate::Status;
}

#[cfg(test)]
mod get_contract_semantics_suite {
    use super::*;
    use std::collections::BTreeMap;
    use tracing::{debug, error, info, trace, warn};

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

    impl Get for SimpleGetDb {
        fn get(&mut self, _options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
            assert!(!value.is_null(), "value must not be null");

            let k = slice_to_vec(key_);
            trace!(key_len = k.len(), "get()called");

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
