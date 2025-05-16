crate::ix!();

impl From<&Vec<u8>> for u256 {
    fn from(v: &Vec<u8>) -> Self {
        if v.len() != 32 {
            panic!("u256::from(&Vec<u8>): input must be 32 bytes, got={}", v.len());
        }
        let mut out = u256::default();
        out.as_slice_mut().copy_from_slice(v);
        out
    }
}

impl From<u8> for u256 {
    fn from(v: u8) -> Self {
        let mut out = u256::default();
        out.as_slice_mut()[0] = v;
        out
    }
}

impl From<*const u8> for u256 {
    #[inline]
    fn from(str_ptr: *const u8) -> Self {
        if str_ptr.is_null() {
            return u256::default();
        }
        let mut out = u256::default();
        out.blob.set_hex(str_ptr);
        out
    }
}

impl From<&String> for u256 {
    #[inline]
    fn from(str_: &String) -> Self {
        let mut out = u256::default();
        out.blob.set_hex_from_str(str_);
        out
    }
}
