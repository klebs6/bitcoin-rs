// ---------------- [ File: bitcoin-bigint/src/set_hex.rs ]
crate::ix!();

// ---------------------------------------------------------------------------
// 2) Macro for From<u64>, From<&str>, set_hex, plus random other conversions
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! define_base_uint_conversions {
    ($name:ident, $bits:expr, $limbs:expr) => {

        impl $name {
            /// set_hex(*const u8)
            pub fn set_hex(&mut self, psz: *const u8) {
                if psz.is_null() {
                    // interpret null as empty => 0
                    *self = Self::default();
                } else {
                    // build a &str
                    let cstr_len = unsafe {
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

            pub fn set_hex_with_str(&mut self, str_: &str) {
                *self = Self::from(str_);
            }
        }
    }
}

#[cfg(test)]
mod base_uint_set_hex_exhaustive_tests {
    use super::*;
    use std::ptr;
    use std::ffi::CString;
    use tracing::{info, debug};

    #[traced_test]
    fn test_set_hex_64_bits_edge_cases() {
        info!("Testing set_hex() for 64-bit BaseUInt with edge cases.");

        type U64B = BaseUInt64;

        // 1) Null pointer => sets to zero
        {
            let mut x = U64B::default();
            x.set_hex(ptr::null() as *const u8);
            assert_eq!(x, U64B::default());
        }

        // 2) A normal c-string with "  0x1234ABCD  "
        {
            let mut x = U64B::default();
            let cstr = CString::new("  0x1234ABCD  ").unwrap();
            x.set_hex(cstr.as_ptr() as *const u8);
            assert_eq!(x.pn[0], 0x1234_ABCD);
            assert_eq!(x.pn[1], 0);
        }

        // 3) partial => "FFFXYZ" => parse "FFF"
        {
            let mut y = U64B::default();
            let cstr2 = CString::new("FFFXYZ").unwrap();
            y.set_hex(cstr2.as_ptr() as *const u8);
            assert_eq!(y.pn[0], 0x0FFF);
            assert_eq!(y.pn[1], 0);
        }

        // 4) set_hex_with_str => "0xDEAD_BEEF_1111_2222"
        {
            let mut z = U64B::default();
            z.set_hex_with_str("0xDEAD_BEEF_1111_2222");
            // => lower 32 => 0x1111_2222, upper 32 => 0xDEAD_BEEF
            assert_eq!(z.pn[0], 0x1111_2222);
            assert_eq!(z.pn[1], 0xDEAD_BEEF);
        }

        info!("64-bit set_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_set_hex_32_bits_edge_cases() {
        info!("Testing set_hex() for 32-bit BaseUInt with edge cases.");

        type U32 = BaseUInt32;

        // 1) Null => 0
        let mut x = U32::default();
        x.set_hex(ptr::null() as *const u8);
        assert_eq!(x, U32::default());

        // 2) "0x12345678" => store => 0x12345678
        let cstr = CString::new("0x12345678").unwrap();
        x.set_hex(cstr.as_ptr() as *const u8);
        assert_eq!(x.pn[0], 0x1234_5678);

        // 3) Overflow => "0xFFFF_FFFF_FFFF" => only keep 0xFFFF_FFFF
        let cstr2 = CString::new("0xFFFF_FFFF_FFFF").unwrap();
        x.set_hex(cstr2.as_ptr() as *const u8);
        assert_eq!(x.pn[0], 0xFFFF_FFFF);

        // 4) set_hex_with_str => "1234" => => 0x1234
        let mut y = U32::default();
        y.set_hex_with_str("1234");
        assert_eq!(y.pn[0], 0x1234);

        info!("32-bit set_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_set_hex_256_bits_edge_cases() {
        info!("Testing set_hex() for 256-bit BaseUInt with edge cases.");
        type U256 = BaseUInt256;

        // 1) null => zero
        let mut x = U256::default();
        x.set_hex(ptr::null() as *const u8);
        assert!(x.pn.iter().all(|&l| l == 0));

        // 2) normal c-string => "  0xDEAF_BEEF_DEAD_BEEF_CAFE_BABE   "
        let cstr = CString::new("  0xDEAF_BEEF_DEAD_BEEF_CAFE_BABE   ").unwrap();
        let mut y = U256::default();
        y.set_hex(cstr.as_ptr() as *const u8);
        let hex_y = y.get_hex();
        let rt = U256::from(hex_y.as_str());
        assert_eq!(rt, y, "Round-trip from set_hex => get_hex => from_str must match.");

        // 3) partial "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        let cstr2 = CString::new("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let mut z = U256::default();
        z.set_hex(cstr2.as_ptr() as *const u8);
        for limb in z.pn.iter() {
            assert_eq!(*limb, 0xFFFF_FFFF);
        }

        info!("256-bit set_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_set_hex_random_32_64_256() {
        info!("Testing set_hex() with random hex for 32,64,256 bits => compare with direct parse.");
        let mut rng = super::super::simple_lcg::SimpleLCG::new(0x1234_5678_9999_8888);

        fn random_hex_string(rng: &mut super::super::simple_lcg::SimpleLCG) -> String {
            let len = (rng.next_u64() % 80 + 1) as usize;
            let mut s = String::new();
            if (rng.next_u64() & 1) == 1 {
                s.push_str("0x");
            }
            for _ in 0..len {
                let nib = (rng.next_u64() & 0xF) as u8;
                let c = core::char::from_digit(nib as u32, 16).unwrap();
                if (rng.next_u64() & 1) == 1 {
                    s.push(c.to_ascii_uppercase());
                } else {
                    s.push(c.to_ascii_lowercase());
                }
            }
            s
        }

        fn check_random_hex_32(rng: &mut super::super::simple_lcg::SimpleLCG) {
            let hex_str = random_hex_string(rng);
            let cstr = CString::new(hex_str.clone()).unwrap();
            let mut x = BaseUInt32::default();
            x.set_hex(cstr.as_ptr() as *const u8);
            let y = BaseUInt32::from(hex_str.as_str());
            assert_eq!(x, y);
        }
        fn check_random_hex_64(rng: &mut super::super::simple_lcg::SimpleLCG) {
            let hex_str = random_hex_string(rng);
            let cstr = CString::new(hex_str.clone()).unwrap();
            let mut x = BaseUInt64::default();
            x.set_hex(cstr.as_ptr() as *const u8);
            let y = BaseUInt64::from(hex_str.as_str());
            assert_eq!(x, y);
        }
        fn check_random_hex_256(rng: &mut super::super::simple_lcg::SimpleLCG) {
            let hex_str = random_hex_string(rng);
            let cstr = CString::new(hex_str.clone()).unwrap();
            let mut x = BaseUInt256::default();
            x.set_hex(cstr.as_ptr() as *const u8);
            let y = BaseUInt256::from(hex_str.as_str());
            assert_eq!(x, y);
        }

        for _ in 0..30 {
            check_random_hex_32(&mut rng);
            check_random_hex_64(&mut rng);
            check_random_hex_256(&mut rng);
        }

        info!("Random set_hex tests for 32,64,256 bits succeeded => matches direct parse logic.");
    }
}
