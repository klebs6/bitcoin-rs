// ---------------- [ File: bitcoin-univalue/src/conversion.rs ]
crate::ix!();

impl From<Instant> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(_val: Instant) -> Self {
        // No canonical JSON representation for `Instant` in
        // Bitcoin‑Core.  We model it as null (matching upstream).
        UniValue::default()
    }
}

impl From<u64> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: u64) -> Self {
        let mut uv = UniValue::default();
        uv.set_u64(val);
        uv
    }
}

impl From<usize> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: usize) -> Self {
        let mut uv = UniValue::default();
        uv.set_u64(val as u64);
        uv
    }
}

impl From<uni_value::VType> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: uni_value::VType) -> Self {
        UniValue::new(val, None)
    }
}

impl From<i64> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: i64) -> Self {
        let mut uv = UniValue::default();
        uv.set_i64(val);
        uv
    }
}

impl From<i32> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: i32) -> Self {
        let mut uv = UniValue::default();
        uv.set_i32(val);
        uv
    }
}

impl From<bool> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: bool) -> Self {
        let mut uv = UniValue::default();
        uv.set_bool(val);
        uv
    }
}

impl From<f64> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: f64) -> Self {
        let mut uv = UniValue::default();
        uv.set_float(val);
        uv
    }
}

impl From<&str> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: &str) -> Self {
        let mut uv = UniValue::default();
        uv.set_str(val);
        uv
    }
}

impl From<String> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: String) -> Self {
        let mut uv = UniValue::default();
        uv.set_str(&val);
        uv
    }
}

impl From<*const u8> for UniValue {
    #[instrument(level = "trace", skip_all)]
    fn from(val: *const u8) -> Self {
        // SAFETY: caller promises `val` points to a valid
        //         NUL‑terminated C string.
        let c_str = unsafe { std::ffi::CStr::from_ptr(val as *const i8) };
        let s     = c_str.to_string_lossy();
        UniValue::from(s.as_ref())
    }
}

#[cfg(test)]
mod conversion_spec {
    use super::*;

    #[traced_test]
    fn bool_conversion() {
        let uv: UniValue = true.into();
        assert_eq!(uv.val(), "1");
        assert_eq!(uv.typ(), &uni_value::VType::VBOOL);
    }

    #[traced_test]
    fn int_conversion() {
        let uv: UniValue = (-123i64).into();
        assert_eq!(uv.val(), "-123");
        assert_eq!(uv.typ(), &uni_value::VType::VNUM);
    }

    #[traced_test]
    fn str_conversion() {
        let uv: UniValue = "hi".into();
        assert_eq!(uv.val(), "hi");
        assert_eq!(uv.typ(), &uni_value::VType::VSTR);
    }

    #[traced_test]
    fn converts_from_owned_string() {
        let owned_str = String::from("hello");
        let uv: UniValue = owned_str.clone().into();
        assert_eq!(uv.typ(), &uni_value::VType::VSTR);
        assert_eq!(uv.val(), &owned_str);
    }
}
