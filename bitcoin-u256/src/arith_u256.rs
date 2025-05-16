// ---------------- [ File: bitcoin-u256/src/arith_u256.rs ]
crate::ix!();

/**
  | 256-bit unsigned big integer wrapper around BaseUInt<256>.
  |
  */
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithU256 {
    pub(crate) base: BaseUInt<256>,
}

impl fmt::Display for ArithU256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for example, display as hex
        // or do something else if you prefer
        let hex = self.base.get_hex();
        write!(f, "{}", hex)
    }
}

impl ShlAssign<u32> for ArithU256 {
    fn shl_assign(&mut self, shift: u32) {
        // just delegate to the underlying base
        self.base <<= shift;
    }
}

impl ShrAssign<u32> for ArithU256 {
    fn shr_assign(&mut self, shift: u32) {
        self.base >>= shift;
    }
}

unsafe impl Send for ArithU256 {}
unsafe impl Sync for ArithU256 {}

impl From<&BaseUInt<256>> for ArithU256 {
    fn from(b: &BaseUInt<256>) -> Self {
        trace!("ArithU256::from<&BaseUInt<256>> => copying base data");
        Self { base: b.clone() }
    }
}

impl From<u64> for ArithU256 {
    fn from(b: u64) -> Self {
        trace!("ArithU256::from<u64> => b=0x{:X}", b);
        Self {
            base: BaseUInt::<256>::from(b),
        }
    }
}

impl From<&str> for ArithU256 {
    fn from(str_: &str) -> Self {
        trace!("ArithU256::from<&str> => '{}'", str_);
        Self {
            base: BaseUInt::<256>::from(str_),
        }
    }
}

impl MulAssign<u32> for ArithU256 {
    #[inline]
    fn mul_assign(&mut self, b32: u32) {
        trace!("ArithU256::mul_assign<u32> => b32={}", b32);
        self.base *= b32;
    }
}

impl MulAssign<i64> for ArithU256 {
    #[inline]
    fn mul_assign(&mut self, b64: i64) {
        trace!("ArithU256::mul_assign<i64> => b64={}", b64);

        // We assume b64 is >= 0; if negative => panic from try_into().
        let as_u32: u32 = b64.try_into().expect("Cannot multiply by a negative i64 in ArithU256");
        self.base *= as_u32;
    }
}

impl MulAssign<&ArithU256> for ArithU256 {
    #[inline]
    fn mul_assign(&mut self, b: &ArithU256) {
        trace!("ArithU256::mul_assign<&ArithU256>");
        self.base *= &b.base;
    }
}

impl DivAssign<u32> for ArithU256 {
    #[inline]
    fn div_assign(&mut self, b32: u32) {
        trace!("ArithU256::div_assign<u32> => b32={}", b32);
        // Convert b32 to a BaseUInt<256>, then div_assign
        let tmp = BaseUInt::<256>::from(b32 as u64);
        self.base /= &tmp;
    }
}

impl DivAssign<i64> for ArithU256 {
    #[inline]
    fn div_assign(&mut self, b64: i64) {
        trace!("ArithU256::div_assign<i64> => b64={}", b64);

        // We assume b64 >= 0; negative => panic.
        let as_u32: u32 = b64.try_into().expect("Cannot divide by a negative i64 in ArithU256");
        let tmp = BaseUInt::<256>::from(as_u32 as u64);
        self.base /= &tmp;
    }
}

impl DivAssign<&ArithU256> for ArithU256 {
    #[inline]
    fn div_assign(&mut self, b: &ArithU256) {
        trace!("ArithU256::div_assign<&ArithU256>");
        self.base /= &b.base;
    }
}

//-------------------------------------------[.cpp/bitcoin/src/arith_u256.cpp]

// ------------------------------------------------------
// Exhaustive test suite for `ArithU256`:
#[cfg(test)]
mod arith_u256_exhaustive_tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    /// We’ll also re-use a small RNG for reproducible tests
    #[derive(Debug)]
    struct SimpleTestRng(u64);
    impl SimpleTestRng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
    }

    /// 1) Test `From<&BaseUInt<256>>`, `From<u64>`, `From<&str>`
    #[traced_test]
    fn test_from_conversions() {
        info!("Testing ArithU256::from conversions...");

        // from BaseUInt<256>
        let mut base = BaseUInt::<256>::default();
        base.set_limb(0, 0x1234_5678);
        base.set_limb(1, 0xFFFF_FFFF);
        let a1 = ArithU256::from(&base);
        assert_eq!(a1.base, base, "ArithU256::from(&BaseUInt<256>) => mismatch");

        // from u64
        let a2 = ArithU256::from(0xABCD_1234_5678_0000u64);
        assert_eq!(a2.base.get_limb(0), 0x5678_0000);
        assert_eq!(a2.base.get_limb(1), 0xABCD_1234u32);
        for i in 2..8 {
            assert_eq!(a2.base.get_limb(i), 0, "limb {} => must be zero", i);
        }

        // from &str
        let a3 = ArithU256::from("0x1234abcd");
        // We'll parse that => 0x1234ABCD in hex => that is 305441741 decimal => stored in the low limbs
        assert_eq!(a3.base.get_limb(0), 0x1234_ABCD);
        for i in 1..8 {
            assert_eq!(a3.base.get_limb(i), 0, "limb {} => must be zero in short parse", i);
        }

        info!("from() conversions done.");
    }

    /// 2) Test `MulAssign<u32|i64|&ArithU256>`  
    #[traced_test]
    fn test_mul_assign() {
        info!("Testing ArithU256 MulAssign with u32, i64, &ArithU256...");

        // a) mul_assign<u32>
        let mut x = ArithU256::from(0x1_00000000u64); // => low64=0x00000000_00000001 for 64-bit
        x *= 2u32;
        // Now we expect => 2<<32 => that is limb[1]=2
        assert_eq!(x.base.get_limb(0), 0);
        assert_eq!(x.base.get_limb(1), 2);

        // b) mul_assign<i64> => same but with i64
        let mut y = ArithU256::from(1u64);
        y *= 5i64; 
        // => 5
        assert_eq!(y.base.get_limb(0), 5);

        // c) mul_assign<&ArithU256>
        let mut a = ArithU256::from(0xABCDu64);
        let b = ArithU256::from(100u64);
        a *= &b;
        // => 0xABCD * 100 decimal => 43981 * 100 => 4398100 decimal => 0x00431EAC
        assert_eq!(a.base.get_limb(0), 0x431EAC, "0xABCD * 100 => mismatch in low32");
        for i in 1..8 {
            assert_eq!(a.base.get_limb(i), 0);
        }

        // Negative i64 => expect panic
        let mut z = ArithU256::from(123u64);
        let caught_neg = catch_unwind(AssertUnwindSafe(|| {
            z *= -3i64; // must panic
        }));
        assert!(caught_neg.is_err(), "mul_assign negative i64 => should panic");

        info!("MulAssign tests concluded.");
    }

    /// 3) Test `DivAssign<u32|i64|&ArithU256>`  
    #[traced_test]
    fn test_div_assign() {
        info!("Testing ArithU256 DivAssign with u32, i64, &ArithU256...");

        // a) div_assign<u32>
        let mut x = ArithU256::from(0x1234_5678u64);
        x /= 0x1234u32; // => 0x5678
        assert_eq!(
            x.base.get_limb(0), 0x5678,
            "Div: 0x12345678 / 0x1234 => mismatch in low32"
        );

        // b) div_assign<i64>
        let mut y = ArithU256::from(1000u64);
        y /= 10i64; 
        assert_eq!(y.base.get_limb(0), 100);

        // negative => panic
        let mut z = ArithU256::from(12345u64);
        let caught_neg = catch_unwind(AssertUnwindSafe(|| {
            z /= -5i64;
        }));
        assert!(caught_neg.is_err(), "div_assign negative i64 => should panic");

        // c) div_assign<&ArithU256>
        let mut a = ArithU256::from(12345u64);
        let b = ArithU256::from(5u64);
        a /= &b; 
        assert_eq!(a.base.get_limb(0), 2469, "12345 / 5 => 2469 decimal in the low limb");

        // division by zero => currently the base code panics. Let's confirm.
        let mut zero_div = ArithU256::from(999u64);
        let zero_base = ArithU256::from(0u64);
        let caught_zd = catch_unwind(AssertUnwindSafe(|| {
            zero_div /= &zero_base; // => panic
        }));
        assert!(caught_zd.is_err(), "divide by zero => must panic for ArithU256");

        info!("DivAssign tests concluded.");
    }

    /// 4) Random smoke tests for larger usage
    #[traced_test]
    fn test_random_arith_u256() {
        info!("Random usage checks for ArithU256 (mul/div).");
        let mut rng = SimpleTestRng::new(0xDEAD_BEEF);

        // We'll do 10 random tests. We'll keep them small enough that we won't overflow too easily.
        for i in 0..10 {
            let a64 = (rng.next_u64() & 0xFFFF) as u64; // short range
            let b32 = (rng.next_u64() & 0xFFF) as u32;  // smaller range
            let c_i64 = ((rng.next_u64() & 0x7FF) as i64).max(1); // positive i64

            let mut a_val = ArithU256::from(a64);
            let b_val = ArithU256::from(b32 as u64);

            // do some combos
            a_val *= &b_val; 
            // => a64 * b32
            let got_low = a_val.base.get_limb(0) as u64
                | ((a_val.base.get_limb(1) as u64) << 32);
            let expected = a64.wrapping_mul(b32 as u64) & 0xFFFF_FFFF_FFFF_FFFF;
            assert_eq!(got_low, expected, "random i={} => low64 mismatch after mul", i);

            // also do a multiply by c_i64
            a_val *= c_i64;
            let got_low_2 = a_val.base.low64();
            let expected_2 = expected.wrapping_mul(c_i64 as u64);
            assert_eq!(got_low_2, expected_2 & 0xFFFF_FFFF_FFFF_FFFF, 
                       "random i={} => low64 mismatch after mul i64", i);

            // divide by some safe value
            let divr = (rng.next_u64() & 0x1FF).max(1) as u32;
            a_val /= divr;
            let final_low = a_val.base.low64();
            let expected_3 = (expected_2 / (divr as u64)) & 0xFFFF_FFFF_FFFF_FFFF;
            assert_eq!(final_low, expected_3, "random i={} => final mismatch after div", i);
        }

        info!("Random tests for ArithU256 done.");
    }
}
