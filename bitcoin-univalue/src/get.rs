// ---------------- [ File: bitcoin-univalue/src/get.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_get.cpp]

impl UniValue {

    /// Copy this object into a plain HashMap.  
    /// No‑op when the receiver is not an object.
    #[instrument(level = "trace", skip(self, kv))]
    pub fn get_obj_map(&self, kv: &mut HashMap<String, UniValue>) {
        if *self.typ() != uni_value::VType::VOBJ {
            trace!("not an object – nothing to copy");
            return;
        }

        kv.clear();
        for (k, v) in self.keys().iter().zip(self.values().iter()) {
            kv.insert(k.clone(), v.clone());
        }
    }

    /// Lightweight accessor for the underlying type.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn get_type(&self) -> uni_value::VType {
        *self.typ()
    }

    /// Returns the internal string representation *without*
    /// validating that this is a string value.  Prefer
    /// `get_str()` for type‑safe access.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn get_val_str(&self) -> &String {
        self.val()
    }
   
    /// Return the object's *key* vector.
    ///
    /// Up‑stream throws a `std::runtime_error` when the value is not an
    /// object; we mirror that behaviour with a plain Rust `panic!`.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn get_keys(&self) -> &Vec<String> {
        if *self.typ() != uni_value::VType::VOBJ {
            panic!("JSON value is not an object as expected");
        }
        self.keys()
    }
    
    /// Return the object's/array's *value* vector.
    ///
    /// Panics unless the receiver is an object **or** an array –
    /// identical contract to the original C++ implementation.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn get_values(&self) -> &Vec<UniValue> {
        match *self.typ() {
            uni_value::VType::VOBJ | uni_value::VType::VARR => self.values(),
            _ => panic!("JSON value is not an object or array as expected"),
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_bool(&self) -> bool {
        if *self.typ() != uni_value::VType::VBOOL {
            panic!("JSON value is not a boolean as expected");
        }
        self.is_true()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_str(&self) -> &str {
        if *self.typ() != uni_value::VType::VSTR {
            panic!("JSON value is not a string as expected");
        }
        self.val()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_str_mut(&mut self) -> &mut str {
        if *self.typ() != uni_value::VType::VSTR {
            panic!("JSON value is not a string as expected");
        }
        self.val_mut()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_int(&self) -> i32 {
        if *self.typ() != uni_value::VType::VNUM {
            panic!("JSON value is not an integer as expected");
        }
        i32::from_str(self.val())
            .expect("JSON integer out of range")
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_int64(&self) -> i64 {
        if *self.typ() != uni_value::VType::VNUM {
            panic!("JSON value is not an integer as expected");
        }
        i64::from_str(self.val())
            .expect("JSON integer out of range")
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_real(&self) -> f64 {
        if *self.typ() != uni_value::VType::VNUM {
            panic!("JSON value is not a number as expected");
        }
        f64::from_str(self.val())
            .expect("JSON double out of range")
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_obj(&self) -> &UniValue {
        if *self.typ() != uni_value::VType::VOBJ {
            panic!("JSON value is not an object as expected");
        }
        self
    }

    #[instrument(level = "trace", skip_all)]
    pub fn get_array(&self) -> &UniValue {
        if *self.typ() != uni_value::VType::VARR {
            panic!("JSON value is not an array as expected");
        }
        self
    }
}

#[cfg(test)]
mod get_spec {
    use super::*;

    #[traced_test]
    fn obj_map_roundtrip() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        obj.keys_mut().extend(["a", "b"].iter().map(|s| s.to_string()));
        obj.values_mut().push(1u64.into());
        obj.values_mut().push(2u64.into());

        let mut map = HashMap::new();
        obj.get_obj_map(&mut map);

        assert_eq!(map["a"].get_int64(), 1);
        assert_eq!(map["b"].get_int64(), 2);
    }

    #[traced_test]
    fn strict_getters_work() {
        let b: UniValue = true.into();
        assert!(b.get_bool());

        let s: UniValue = "hi".into();
        assert_eq!(s.get_str(), "hi");

        let n: UniValue = (-7i64).into();
        assert_eq!(n.get_int64(), -7);
    }

    #[test]
    #[should_panic(expected = "JSON value is not a boolean")]
    fn strict_getter_panics_on_type_mismatch() {
        let s: UniValue = "oops".into();
        s.get_bool(); // wrong type
    }
}
