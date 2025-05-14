crate::ix!();

impl<const BITS: usize> BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Set self from a hex string. In C++ code: `*this = UintToArith256(uint256S(psz))`.
    /// We'll just reuse our From<&str> logic:
    pub fn set_hex(&mut self, psz: *const u8) {
        if psz.is_null() {
            // interpret null as empty => 0
            *self = Self::default();
        } else {
            // Convert *const u8 to &str (unsafe). We'll do a quick conversion or fallback to empty if invalid.
            let cstr_len = unsafe {
                // find length up to a null terminator
                let mut len = 0;
                let mut ptr = psz;
                while !ptr.is_null() && *ptr != 0 {
                    ptr = ptr.add(1);
                    len += 1;
                }
                len
            };
            let slice = unsafe { std::slice::from_raw_parts(psz, cstr_len) };
            let as_str = std::str::from_utf8(slice).unwrap_or("");
            *self = Self::from(as_str);
        }
    }

    /// Overload that takes a &str
    pub fn set_hex_with_str(&mut self, str_: &str) {
        *self = Self::from(str_);
    }
}
