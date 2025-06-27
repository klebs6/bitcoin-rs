// ---------------- [ File: bitcoin-univalue/src/mutator.rs ]
crate::ix!();

impl UniValue {

     /// Set from an `i32`.
    #[instrument(level = "trace", skip(self))]
    pub fn set_i32(&mut self, val: i32) -> bool {
        self.set_int_inner(val.to_string())
    }

    /// Set from an `i64`.
    #[instrument(level = "trace", skip(self))]
    pub fn set_i64(&mut self, val: i64) -> bool {
        self.set_int_inner(val.to_string())
    }

    /// Set from a `u64`.
    #[instrument(level = "trace", skip(self))]
    pub fn set_u64(&mut self, val: u64) -> bool {
        self.set_int_inner(val.to_string())
    }

    /// Set to **null**.
    #[instrument(level = "trace", skip(self))]
    pub fn set_null(&mut self) -> bool {
        self.clear();
        true
    }
   
    /// Set to a JSON boolean.
    #[instrument(level = "trace", skip(self))]
    pub fn set_bool(&mut self, val: bool) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VBOOL);
        self.set_val(if val { "1" } else { "0" }.to_owned());
        true
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_int<T: Debug + std::fmt::Display>(&mut self, val: T) -> bool {
        self.set_num_str(&val.to_string())
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_array(&mut self) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VARR);
        true
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_object(&mut self) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VOBJ);
        true
    }
    
    /// Set from a Rust string slice (JSON string).
    #[instrument(level = "trace", skip(self))]
    pub fn set_str(&mut self, val: &str) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VSTR);
        self.set_val(val.to_owned());
        true
    }
    
    /// Helper – common implementation for signed / unsigned integers.
    fn set_int_inner(&mut self, s: String) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VNUM);
        self.set_val(s);
        true
    }

    /// Set from a JSON number string *as‑is*.
    /// Returns `false` when *val* is **not** a syntactically‑valid
    /// JSON number (per the lexical rules in `valid_num_str`).
    #[instrument(level = "trace", skip(self))]
    pub fn set_num_str(&mut self, val: &String) -> bool {
        if !parse_prechecks(val) || !valid_num_str(val) {
            trace!("rejected: invalid JSON number");
            return false;
        }
        self.clear();
        self.set_typ(uni_value::VType::VNUM);
        self.set_val(val.clone());
        true
    }


    /* -- public surface, unchanged except for the one‑liner ---------- */
    #[instrument(level = "trace", skip(self))]
    pub fn set_float(&mut self, val: f64) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VNUM);
        self.set_val(Self::format_f64_canonical(val));
        true
    }
}

#[cfg(test)]
mod core_mutator_spec {
    use super::*;

    #[traced_test]
    fn default_is_null() {
        let uv = UniValue::default();
        assert_eq!(*uv.typ(), uni_value::VType::VNULL);
    }

    #[traced_test]
    fn set_bool_updates_state() {
        let mut uv = UniValue::default();
        uv.set_bool(true);
        assert_eq!(*uv.typ(), uni_value::VType::VBOOL);
        assert_eq!(uv.val(), "1");
    }

    #[traced_test]
    fn set_str_updates_state() {
        let mut uv = UniValue::default();
        uv.set_str("hello");
        assert_eq!(*uv.typ(), uni_value::VType::VSTR);
        assert_eq!(uv.val(), "hello");
    }

    #[traced_test]
    fn set_i64_updates_state() {
        let mut uv = UniValue::default();
        uv.set_i64(-42);
        assert_eq!(*uv.typ(), uni_value::VType::VNUM);
        assert_eq!(uv.val(), "-42");
    }

    #[traced_test]
    fn set_num_str_accepts_valid() {
        let mut uv = UniValue::default();
        assert!(uv.set_num_str(&"123".to_string()));
        assert_eq!(uv.val(), "123");
    }

    #[traced_test]
    fn set_num_str_rejects_invalid() {
        let mut uv = UniValue::default();
        assert!(!uv.set_num_str(&" 12".to_string()));
        assert!(uv.is_null());
    }

    #[traced_test]
    fn generic_set_int() {
        let mut uv = UniValue::default();
        uv.set_int(99u32);
        assert_eq!(uv.val(), "99");
    }

    #[traced_test]
    fn set_array_object() {
        let mut uv = UniValue::default();
        uv.set_array();
        assert_eq!(*uv.typ(), uni_value::VType::VARR);
        uv.set_object();
        assert_eq!(*uv.typ(), uni_value::VType::VOBJ);
    }
}
