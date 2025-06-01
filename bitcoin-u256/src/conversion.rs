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

impl From<&str> for u256 {
    #[inline]
    fn from(str_: &str) -> Self {
        let mut out = u256::default();
        out.blob.set_hex_from_str(str_);
        out
    }
}

#[cfg(test)]
mod u256_from_exhaustive_tests {
    use super::*;
    use std::ffi::CString;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    /// We’ll define a small pseudo-random generator for stable random sequences.
    struct MiniRng(u64);
    impl MiniRng {
        fn new(seed: u64) -> Self { Self(seed) }
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
    }

    #[traced_test]
    fn test_from_vec_u8() {
        info!("Testing `u256::from(&Vec<u8>)` for correct length and usage...");

        // (1) Exactly 32 bytes => no panic
        let good_32 = vec![0u8; 32];
        let x = u256::from(&good_32);
        assert!(x.is_null(), "We used all-zero => expect is_null()=true.");

        // (2) If length < 32 => must panic
        let short_vec = vec![1u8; 16];
        let caught_short = catch_unwind(AssertUnwindSafe(|| {
            let _ = u256::from(&short_vec);
        }));
        assert!(caught_short.is_err(), "from(&Vec<u8> with length <32) => panic expected.");

        // (3) If length > 32 => also panic
        let long_vec = vec![2u8; 64];
        let caught_long = catch_unwind(AssertUnwindSafe(|| {
            let _ = u256::from(&long_vec);
        }));
        assert!(caught_long.is_err(), "from(&Vec<u8> with length>32) => panic expected.");

        // (4) Nonzero check
        let mut r = MiniRng::new(0xAABB_0011_2233_4455);
        let mut random_32 = vec![0u8; 32];
        for b in random_32.iter_mut() {
            *b = (r.next_u64() & 0xFF) as u8;
        }
        let y = u256::from(&random_32);
        assert!(!y.is_null(), "Random 32 bytes => not null if any byte is nonzero.");
        trace!("(4) random => y={:?}", y);

        info!("test_from_vec_u8 => all sub-checks passed.");
    }

    #[traced_test]
    fn test_from_u8() {
        info!("Testing `u256::from(u8)` => only low byte is set, rest zero.");

        // We'll do some random values plus boundary: 0, 255
        let values = [0u8, 1, 123, 255];
        for &val in &values {
            let num = u256::from(val);
            let s = num.as_slice();
            assert_eq!(s[0], val, "lowest byte must match the input u8");
            for i in 1..32 {
                assert_eq!(s[i], 0, "other bytes must be zero");
            }
        }
        info!("test_from_u8 => confirmed that from(u8) sets only the low byte.");
    }

    #[traced_test]
    fn test_from_ptr_const_u8() {
        info!("Testing `u256::from(*const u8)` => parse a hex c-string (or null).");

        // (1) Null => default => zero
        let null_ptr: *const u8 = std::ptr::null();
        let z = u256::from(null_ptr);
        assert!(z.is_null(), "from(null_ptr) => default => zero");

        // (2) A typical c-string => "0xABcDeF"
        // We simulate it with `CString`.
        let cstr = CString::new("  0XAbCDeF  ").unwrap(); // e.g. with whitespace
        let raw_ptr = cstr.as_ptr() as *const u8;
        let parsed = u256::from(raw_ptr);
        // That hex => big-endian nibs [0xAB, 0xCD, 0xEF], stored LE in the low bytes => check the first 3
        let sl = parsed.as_slice();
        assert!(sl[0] != 0 || sl[1] != 0 || sl[2] != 0, 
            "some nonzero bytes after parse from '0xABcDeF'");
        assert!(!parsed.is_null(), "should parse nonzero from cstr=0xABcDeF");
        trace!("(2) from('0xABcDeF') => parsed={:?}", parsed);

        // (3) partial => "DEADbeefXYZ" => parse up to non-hex => 
        // We'll just check we get something nonzero.
        let cstr2 = CString::new("  DEADbeefXYZ").unwrap();
        let parsed2 = u256::from(cstr2.as_ptr() as *const u8);
        assert!(!parsed2.is_null(), "parsed2 => must be nonnull from 'DEADbeefXYZ' partial parse.");

        info!("test_from_ptr_const_u8 => done with c-string => parse hex checks.");
    }

    #[traced_test]
    fn test_from_string_ref() {
        info!("Testing `u256::from(&String)` => parse hex, ignoring prefix/spaces/etc.");

        // (1) empty => => 0
        let empty = "".to_string();
        let zero = u256::from(&empty);
        assert!(zero.is_null(), "empty => parse => zero");

        // (2) normal => "0x1234"
        let s1 = "0x1234".to_string();
        let x1 = u256::from(&s1);
        assert!(!x1.is_null(), "Should parse nonzero from '0x1234'");

        // (3) underscore => "beef__cafe"
        let s2 = "beef__cafe".to_string();
        let x2 = u256::from(&s2);
        assert!(!x2.is_null(), "Should parse ignoring underscores => 'BEEFCAFE' => nonzero");
        trace!("(3) => x2={:?}", x2);

        // (4) partial => "deadbeefXYZ" => parse => 0xDEADBEEF
        let s3 = "deadbeefXYZ".to_string();
        let x3 = u256::from(&s3);
        assert!(!x3.is_null(), "deadbeef => partial parse => not null");

        info!("test_from_string_ref => all sub-checks done.");
    }
}
