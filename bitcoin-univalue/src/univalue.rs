// ---------------- [ File: bitcoin-univalue/src/univalue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/include/univalue.h]

#[derive(Setters,MutGetters,Getters,Clone,Debug)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct UniValue {

    typ:    uni_value::VType,

    /// numbers are stored as strings
    val:    String,
    keys:   Vec<String>,
    values: Vec<UniValue>,
}

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue.cpp]

lazy_static!{
    pub static ref NULL_UNI_VALUE: UniValue = UniValue::default();
}

pub mod uni_value {

    #[derive(Copy,PartialEq,Eq,Clone,Debug)]
    pub enum VType { 
        VNULL, 
        VOBJ, 
        VARR, 
        VSTR, 
        VNUM, 
        VBOOL, 
    }
}

impl Default for UniValue {
    /// A default `UniValue` is the JSON literal **null**.
    fn default() -> Self {
        Self {
            typ:    uni_value::VType::VNULL,
            val:    String::new(),
            keys:   Vec::new(),
            values: Vec::new(),
        }
    }
}

impl UniValue {

    /// Create a new `UniValue` with the provided type
    /// and (optionally) initial string value.
    #[instrument(level = "trace", skip_all)]
    pub fn new(initial_type: uni_value::VType, initial_str: Option<&str>) -> Self {
        let mut uv = Self::default();
        match initial_type {
            uni_value::VType::VNULL => { /* nothing */ }
            uni_value::VType::VBOOL => {
                uv.set_bool(initial_str == Some("1"));
            }
            uni_value::VType::VSTR => {
                uv.set_str(initial_str.unwrap_or_default());
            }
            uni_value::VType::VNUM => {
                uv.val = initial_str.unwrap_or_default().to_owned();
                uv.typ = uni_value::VType::VNUM;
            }
            uni_value::VType::VARR => {
                uv.typ = uni_value::VType::VARR;
            }
            uni_value::VType::VOBJ => {
                uv.typ = uni_value::VType::VOBJ;
            }
        }
        uv
    }

    /// Return `true` when the *values* vector is empty.
    #[inline]
    pub fn empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Number of child values (arrays/objects) or `0`
    /// for scalars.
    #[inline]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// `true` if *key* exists in this object.
    #[inline]
    pub fn exists(&self, key: &str) -> bool {
        let mut idx = 0usize;
        self.find_key(key, &mut idx)
    }

    /// Shorthand for `get_type()` in the C++ original.
    #[inline]
    pub fn ty(&self) -> uni_value::VType {
        *self.typ()
    }

    /// Linear search helper (same algorithm as upstream).
    pub fn find_key(&self, key: &str, ret_idx: &mut usize) -> bool {
        for (i, k) in self.keys.iter().enumerate() {
            if k == key {
                *ret_idx = i;
                return true;
            }
        }
        false
    }

    /// Clear the current value, turning it into **null**.
    #[instrument(level = "trace", skip(self))]
    pub fn clear(&mut self) {
        self.typ    = uni_value::VType::VNULL;
        self.val.clear();
        self.keys.clear();
        self.values.clear();
    }
}

#[cfg(test)]
mod univalue_aux_spec {
    use super::*;

    #[traced_test]
    fn helpers_behave() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(obj.empty());
        assert_eq!(obj.size(), 0);

        obj.keys_mut().push("k".into());
        obj.values_mut().push(7u64.into());

        assert!(!obj.empty());
        assert_eq!(obj.size(), 1);
        assert!(obj.exists("k"));
        assert!(!obj.exists("nope"));
        assert_eq!(obj.ty(), uni_value::VType::VOBJ);
    }
}
