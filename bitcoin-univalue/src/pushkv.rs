// ---------------- [ File: bitcoin-univalue/src/pushkv.rs ]
crate::ix!();

impl UniValue {
    /// Copy every key/value pair from *obj* into `self`, **without**
    /// deduplication.  Fails when either receiver or source is not
    /// a JSON object (faithfully mirrors C++ `pushKVs`).
    #[instrument(level = "trace", skip(self, obj))]
    pub fn push_kvs(&mut self, obj: &UniValue) -> bool {
        if *self.typ() != uni_value::VType::VOBJ || *obj.typ() != uni_value::VType::VOBJ {
            trace!("receiver or source not an object – push_kvs rejected");
            return false;
        }

        for (k, v) in obj.keys().iter().cloned().zip(obj.values().iter().cloned()) {
            self.push_kv_raw(k, v);
        }
        true
    }

    /// Internal helper that **always** appends, bypassing duplicate
    /// checking (up‑stream name: `__pushKV`).
    #[inline]
    fn push_kv_raw(&mut self, key: String, val: UniValue) {
        self.keys_mut().push(key);
        self.values_mut().push(val);
    }

    /// Insert or replace *key* with *val*.
    /// - If `self` is **not** an object → **false**  
    /// - If *key* exists → value is **replaced**  
    /// - Otherwise      → pair is **appended**
    #[instrument(level = "trace", skip(self, val))]
    pub fn pushkv<T: Into<UniValue>>(&mut self, key: &str, val: T) -> bool {
        if *self.typ() != uni_value::VType::VOBJ {
            trace!("receiver is not an object – pushkv rejected");
            return false;
        }

        let uv_val: UniValue = val.into();
        match self.keys().iter().position(|k| k == key) {
            Some(idx) => {
                trace!(action = "replace", key);
                self.values_mut()[idx] = uv_val;
            }
            None => {
                trace!(action = "insert", key);
                self.push_kv_raw(key.to_owned(), uv_val);
            }
        }
        true
    }
}

#[cfg(test)]
mod pushkv_spec {
    use super::*;

    /// Helper – build an empty JSON object.
    fn empty_object() -> UniValue {
        UniValue::new(uni_value::VType::VOBJ, None)
    }

    #[traced_test]
    fn insert_and_replace() {
        let mut obj = empty_object();
        assert!(obj.pushkv("a", 1u64));
        assert_eq!(obj["a"].get_int64(), 1);

        // replace existing
        assert!(obj.pushkv("a", 2i64));
        assert_eq!(obj["a"].get_int64(), 2);

        // insert new
        assert!(obj.pushkv("b", true));
        assert!(obj["b"].get_bool());
        assert_eq!(obj.size(), 2);
    }

    #[traced_test]
    fn non_object_rejected() {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        assert!(!arr.pushkv("x", 1u64));
        assert_eq!(arr.size(), 0);
    }

    #[traced_test]
    fn push_kvs_bulk_copy() {
        let mut src = empty_object();
        src.pushkv("x", 10u64);
        src.pushkv("y", "hello");

        let mut dst = empty_object();
        assert!(dst.push_kvs(&src));
        assert_eq!(dst["x"].get_int64(), 10);
        assert_eq!(dst["y"].get_str(), "hello");
        assert_eq!(dst.size(), 2);
    }

    #[traced_test]
    fn push_kvs_type_mismatch() {
        let src = UniValue::new(uni_value::VType::VARR, None);
        let mut dst = empty_object();
        assert!(!dst.push_kvs(&src));
    }
}
