// ---------------- [ File: bitcoin-univalue/src/uv_type_name.rs ]
crate::ix!();

#[instrument(level = "trace", skip_all)]
pub fn uv_type_name(t: uni_value::VType) -> *const u8 {
    match t {
        uni_value::VType::VNULL => b"null\0".as_ptr(),
        uni_value::VType::VBOOL => b"bool\0".as_ptr(),
        uni_value::VType::VOBJ  => b"object\0".as_ptr(),
        uni_value::VType::VARR  => b"array\0".as_ptr(),
        uni_value::VType::VSTR  => b"string\0".as_ptr(),
        uni_value::VType::VNUM  => b"number\0".as_ptr(),
    }
}

#[cfg(test)]
mod uv_type_name_spec {
    use super::*;
    use std::ffi::CStr;

    #[traced_test]
    fn returns_expected_strings() {
        let cases = [
            (uni_value::VType::VNULL,  "null"),
            (uni_value::VType::VBOOL,  "bool"),
            (uni_value::VType::VOBJ,   "object"),
            (uni_value::VType::VARR,   "array"),
            (uni_value::VType::VSTR,   "string"),
            (uni_value::VType::VNUM,   "number"),
        ];

        for (vt, expect) in cases {
            let cstr = unsafe { CStr::from_ptr(uv_type_name(vt) as *const i8) };
            assert_eq!(cstr.to_str().unwrap(), expect);
        }
    }
}
