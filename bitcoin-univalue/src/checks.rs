// ---------------- [ File: bitcoin-univalue/src/checks.rs ]
crate::ix!();

impl UniValue {

    /// Return **true** when this value is the
    /// JSON literal `null`.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_null(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VNULL;
        trace!(result);
        result
    }

    /// Return **true** when this value is the
    /// JSON boolean `true`.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_true(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VBOOL && *self.val() == "1";
        trace!(result);
        result
    }

    /// Return **true** when this value is the
    /// JSON boolean `false`.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_false(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VBOOL && *self.val() != "1";
        trace!(result);
        result
    }

    /// Return **true** when this value is any
    /// JSON boolean (either `true` or `false`).
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_bool(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VBOOL;
        trace!(result);
        result
    }

    /// Return **true** when this value is a
    /// JSON string.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_str(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VSTR;
        trace!(result);
        result
    }

    /// Return **true** when this value is a
    /// JSON number.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_num(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VNUM;
        trace!(result);
        result
    }

    /// Return **true** when this value is a
    /// JSON array.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_array(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VARR;
        trace!(result);
        result
    }

    /// Return **true** when this value is a
    /// JSON object.
    #[inline]
    #[instrument(level = "trace", skip_all)]
    pub fn is_object(&self) -> bool {
        let result = *self.typ() == uni_value::VType::VOBJ;
        trace!(result);
        result
    }
}

#[cfg(test)]
mod univalue_checks_spec {
    use super::*;

    #[traced_test]
    fn default_is_null() {
        let uv = UniValue::default();
        assert!(uv.is_null());
        assert!(!uv.is_bool());
        assert!(!uv.is_true());
        assert!(!uv.is_false());
        assert!(!uv.is_str());
        assert!(!uv.is_num());
        assert!(!uv.is_array());
        assert!(!uv.is_object());
    }

    #[traced_test]
    fn boolean_checks() {
        let uv_true: UniValue = true.into();
        let uv_false: UniValue = false.into();

        assert!(uv_true.is_bool());
        assert!(uv_true.is_true());
        assert!(!uv_true.is_false());

        assert!(uv_false.is_bool());
        assert!(uv_false.is_false());
        assert!(!uv_false.is_true());
    }

    #[traced_test]
    fn string_check() {
        let uv: UniValue = "hello".into();
        assert!(uv.is_str());
        assert!(!uv.is_null());
        assert!(!uv.is_bool());
    }

    #[traced_test]
    fn number_check() {
        let uv: UniValue = 42u64.into();
        assert!(uv.is_num());
        assert!(!uv.is_str());
    }

    #[traced_test]
    fn array_check() {
        let uv = UniValue::new(uni_value::VType::VARR, None);
        assert!(uv.is_array());
        assert!(!uv.is_object());
    }

    #[traced_test]
    fn object_check() {
        let uv = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(uv.is_object());
        assert!(!uv.is_array());
    }
}
