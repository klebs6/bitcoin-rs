// ---------------- [ File: bitcoin-bigint/src/set_hex.rs ]
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

#[cfg(test)]
mod base_uint_set_hex_exhaustive_tests {
    use super::*;
    use std::ptr;
    use std::ffi::CString;

    /// Exhaustive tests for:
    /// - `BaseUInt<BITS>::set_hex(*const u8)`
    /// - `BaseUInt<BITS>::set_hex_with_str(&str)`
    ///
    /// Both eventually call the `From<&str>` logic, but `set_hex` handles a raw C-string pointer
    /// (potentially null).
    #[traced_test]
    fn test_set_hex_64_bits_edge_cases() {
        info!("Testing set_hex() and set_hex_with_str() for 64-bit BaseUInt with edge cases.");

        type U64B = BaseUInt<64>;

        // 1) Null pointer => sets to zero
        {
            let mut x = U64B::default();
            let null_ptr: *const u8 = ptr::null();
            x.set_hex(null_ptr);
            assert_eq!(x, U64B::default(), "null pointer => should set to zero");
        }

        // 2) A normal c-string with "0x1234ABCD"
        {
            let mut x = U64B::default();
            let cstr = CString::new("  0x1234ABCD  ").unwrap();
            let raw_ptr = cstr.as_ptr();
            x.set_hex(raw_ptr as *const u8);
            debug!("After set_hex(raw_ptr with '0x1234ABCD'), x={:?}", x);
            // parse => 0x1234ABCD => => lower limb => 0x1234ABCD, upper => 0
            assert_eq!(x.pn[0], 0x1234_ABCD);
            assert_eq!(x.pn[1], 0);
        }

        // 3) Some random partial hex => "FFFXYZ" => stops at 'X'
        //    e.g. => "FFF" => => 0xFFF => => 0x00000FFF
        {
            let mut y = U64B::default();
            let cstr2 = CString::new("FFFXYZ").unwrap();
            y.set_hex(cstr2.as_ptr() as *const u8);
            // "FFF" => => 0x0FFF in the lower limb
            assert_eq!(y.pn[0], 0x0FFF);
            assert_eq!(y.pn[1], 0);
        }

        // 4) set_hex_with_str => do the same checks
        {
            let mut z = U64B::default();
            z.set_hex_with_str("0xDEAD_BEEF_1111_2222");
            // => parse => "DEAD_BEEF_1111_2222" ignoring underscores
            //    => = 0xDEAD_BEEF_1111_2222 (64 bits).
            // Lower 32 => 0x1111_2222, upper 32 => 0xDEAD_BEEF
            assert_eq!(z.pn[0], 0x1111_2222);
            assert_eq!(z.pn[1], 0xDEAD_BEEF);
        }

        info!("64-bit set_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_set_hex_32_bits_edge_cases() {
        info!("Testing set_hex() and set_hex_with_str() for 32-bit BaseUInt with edge cases.");

        type U32 = BaseUInt<32>;

        // 1) Null => 0
        let mut x = U32::default();
        x.set_hex(ptr::null() as *const u8);
        assert_eq!(x, U32::default());

        // 2) "0x12345678" => store => 0x12345678 in the only limb
        let cstr = CString::new("0x12345678").unwrap();
        x.set_hex(cstr.as_ptr() as *const u8);
        assert_eq!(x.pn[0], 0x1234_5678);

        // 3) Overflow => e.g. "0xFFFF_FFFF_FFFF" => only keep lower 32 => 0xFFFF_FFFF
        let cstr2 = CString::new("0xFFFF_FFFF_FFFF").unwrap();
        x.set_hex(cstr2.as_ptr() as *const u8);
        assert_eq!(x.pn[0], 0xFFFF_FFFF);

        // 4) set_hex_with_str => e.g. "1234" => => 0x1234
        let mut y = U32::default();
        y.set_hex_with_str("1234");
        assert_eq!(y.pn[0], 0x1234);

        info!("32-bit set_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_set_hex_256_bits_edge_cases() {
        info!("Testing set_hex() and set_hex_with_str() for 256-bit BaseUInt with edge cases.");

        type U256 = BaseUInt<256>;

        // 1) null => zero
        let mut x = U256::default();
        x.set_hex(ptr::null() as *const u8);
        assert!(x.pn.iter().all(|&l| l == 0));

        // 2) normal c-string => e.g. "  0xDEAF_BEEF_DEAD_BEEF_CAFE_BABE   "
        //    parse => store in the first few limbs
        let cstr = CString::new("  0xDEAF_BEEF_DEAD_BEEF_CAFE_BABE   ").unwrap();
        let mut y = U256::default();
        y.set_hex(cstr.as_ptr() as *const u8);
        debug!("y={:?}", y);
        // we won't do exact limb check here, but let's get_hex and parse back
        let hex_y = y.get_hex();
        debug!("hex_y='{}'", hex_y);
        // parse again => compare limbs => round trip
        let rt = U256::from(hex_y.as_str());
        assert_eq!(rt, y, "Round-trip from set_hex => get_hex => from_str must match.");

        // 3) partial "0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF" => bigger than 256 => keep only 256
        let cstr2 = CString::new("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let mut z = U256::default();
        z.set_hex(cstr2.as_ptr() as *const u8);
        // => effectively 288 bits => we only keep the lower 256 => 0xFFFF_FFFF... repeated for 8 limbs
        for limb in z.pn.iter() {
            assert_eq!(*limb, 0xFFFF_FFFF);
        }

        info!("256-bit set_hex edge-case tests passed.");
    }

    /// We'll also do a random approach. We'll build random hex strings, parse them with set_hex, 
    /// then compare to a direct parse via BaseUInt::from(&str).
    #[traced_test]
    fn test_set_hex_random_32_64_256() {
        info!("Testing set_hex() with random hex for 32,64,256 bits => compare with direct parse.");

        let mut rng = SimpleLCG::new(0x1234_5678_9999_8888);

        // We'll define a small helper: generate random hex up to e.g. 80 nibbles, then pass to set_hex.
        fn random_hex_string(rng: &mut SimpleLCG) -> String {
            // length from 1..=80 nibble chars
            let len = (rng.next_u64() % 80 + 1) as usize;
            let mut s = String::new();
            // optionally 50% chance add '0x'
            if (rng.next_u64() & 1) == 1 {
                s.push_str("0x");
            }
            for _ in 0..len {
                let nib = (rng.next_u64() & 0xF) as u8;
                let c = core::char::from_digit(nib as u32, 16).unwrap();
                // randomly uppercase?
                if (rng.next_u64() & 1) == 1 {
                    s.push(c.to_ascii_uppercase());
                } else {
                    s.push(c.to_ascii_lowercase());
                }
            }
            s
        }

        // For each BITS, we'll do random tests
        fn check_random_hex<const B: usize>(rng: &mut SimpleLCG)
        where
            [(); B / 32]:,
        {
            let hex_str = random_hex_string(rng);
            debug!("Random hex => '{}'", hex_str);

            let cstr = CString::new(hex_str.clone()).unwrap();
            let mut x = BaseUInt::<B>::default();
            x.set_hex(cstr.as_ptr() as *const u8);

            // Compare with direct parse: y=BaseUInt::<B>::from(&hex_str)
            let y = BaseUInt::<B>::from(hex_str.as_str());
            assert_eq!(x, y, "Random set_hex vs direct parse mismatch for B={}", B);
        }

        // We'll do 30 random tries for 32,64,256
        for _ in 0..30 {
            check_random_hex::<32>(&mut rng);
            check_random_hex::<64>(&mut rng);
            check_random_hex::<256>(&mut rng);
        }

        info!("Random set_hex tests for 32,64,256 bits succeeded => matches direct parse logic.");
    }
}
