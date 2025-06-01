// ---------------- [ File: bitcoin-u256/src/arith_u256.rs ]
crate::ix!();

/**
  | 256-bit unsigned big integer wrapper around BaseUInt256.
  |
  */
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithU256 {
    pub(crate) base: BaseUInt256,
}

impl ArithU256 {

    pub fn low64(&self) -> u64 {
        self.base.low64()
    }

    pub fn get_limb(&self, index: usize) -> u32 {
        self.base.get_limb(index)
    }

    /// Provides a hex string of the underlying 256 bits, same big-endian style as `to_string()`.
    pub fn get_hex(&self) -> String {
        self.base.get_hex()
    }

    /// Returns the size in bytes (which should be 32 for a 256-bit number).
    pub fn size_in_bytes(&self) -> usize {
        self.base.size_in_bytes()
    }

    /// Interprets this 256-bit number as a floating-point, summing each 32-bit limb in powers of 2^32.
    /// **Caution**: will lose precision beyond ~53 bits, as `f64` only has 52 mantissa bits.
    pub fn getdouble(&self) -> f64 {
        self.base.getdouble()
    }
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

impl From<&BaseUInt256> for ArithU256 {
    fn from(b: &BaseUInt256) -> Self {
        trace!("ArithU256::from<&BaseUInt256> => copying base data");
        Self { base: b.clone() }
    }
}

impl From<u64> for ArithU256 {
    fn from(b: u64) -> Self {
        trace!("ArithU256::from<u64> => b=0x{:X}", b);
        Self {
            base: BaseUInt256::from(b),
        }
    }
}

impl From<&str> for ArithU256 {
    fn from(str_: &str) -> Self {
        trace!("ArithU256::from<&str> => '{}'", str_);
        Self {
            base: BaseUInt256::from(str_),
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
        // Convert b32 to a BaseUInt256, then div_assign
        let tmp = BaseUInt256::from(b32 as u64);
        self.base /= &tmp;
    }
}

impl DivAssign<i64> for ArithU256 {
    #[inline]
    fn div_assign(&mut self, b64: i64) {
        trace!("ArithU256::div_assign<i64> => b64={}", b64);

        // We assume b64 >= 0; negative => panic.
        let as_u32: u32 = b64.try_into().expect("Cannot divide by a negative i64 in ArithU256");
        let tmp = BaseUInt256::from(as_u32 as u64);
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

    /// 1) Test `From<&BaseUInt256>`, `From<u64>`, `From<&str>`
    #[traced_test]
    fn test_from_conversions() {
        info!("Testing ArithU256::from conversions...");

        // from BaseUInt256
        let mut base = BaseUInt256::default();
        base.set_limb(0, 0x1234_5678);
        base.set_limb(1, 0xFFFF_FFFF);
        let a1 = ArithU256::from(&base);
        assert_eq!(a1.base, base, "ArithU256::from(&BaseUInt256) => mismatch");

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

    #[traced_test]
    fn test_mul_assign() {
        info!("BEGIN test_mul_assign => verifying ArithU256 mul_assign<u32>, mul_assign<i64>, and mul_assign<&ArithU256> operations...");

        // a) mul_assign<u32>
        info!("(a) Testing ArithU256::mul_assign<u32> => x *= 2u32");
        let mut x = ArithU256::from(0x1_00000000u64);
        trace!("  x (before) = 0x{:016X} (low64), limbs=[{:08X}, {:08X}, ...]", x.base.low64(), x.base.get_limb(0), x.base.get_limb(1));
        x *= 2u32;
        trace!("  x (after)  = 0x{:016X} (low64), limbs=[{:08X}, {:08X}, ...]", x.base.low64(), x.base.get_limb(0), x.base.get_limb(1));
        // Now we expect => 2<<32 => that is limb[1]=2
        assert_eq!(x.base.get_limb(0), 0, "Expected limb[0] = 0 after mul by 2");
        assert_eq!(x.base.get_limb(1), 2, "Expected limb[1] = 2 after mul by 2");

        // b) mul_assign<i64>
        info!("(b) Testing ArithU256::mul_assign<i64> => y *= 5i64");
        let mut y = ArithU256::from(1u64);
        trace!("  y (before) = 0x{:016X} (low64)", y.base.low64());
        y *= 5i64; 
        trace!("  y (after)  = 0x{:016X} (low64)", y.base.low64());
        // => 5
        assert_eq!(y.base.get_limb(0), 5, "Expected y to be 5 after mul by 5 i64");
        for i in 1..8 {
            assert_eq!(y.base.get_limb(i), 0, "Higher limbs must remain zero");
        }

        // c) mul_assign<&ArithU256>
        info!("(c) Testing ArithU256::mul_assign<&ArithU256> => a *= b");
        let mut a = ArithU256::from(0xABCDu64);
        let b = ArithU256::from(100u64);
        trace!("  a (before) = 0x{:016X}, b=0x{:016X}", a.base.low64(), b.base.low64());
        a *= &b;
        trace!("  a (after)  = 0x{:016X}, limbs=[{:08X}, {:08X}, ...]", a.base.low64(), a.base.get_limb(0), a.base.get_limb(1));
        // => 0xABCD * 100 decimal => 43981 * 100 => 4398100 => 0x00431C14
        let got_low = a.base.get_limb(0);
        let expect_low = 0x431C14; 
        assert_eq!(got_low, expect_low, "0xABCD * 100 => mismatch in low32");
        for i in 1..8 {
            assert_eq!(a.base.get_limb(i), 0, "Higher limbs must be zero for this small product");
        }

        // Negative i64 => expect panic
        info!("(d) Testing negative i64 => multiply must panic");
        let mut z = ArithU256::from(123u64);
        let caught_neg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            z *= -3i64; // must panic
        }));
        trace!("  negative i64 multiply => caught_neg={:?}", caught_neg.is_err());
        assert!(caught_neg.is_err(), "mul_assign negative i64 => should panic");

        info!("MulAssign tests concluded successfully.");
    }

    #[traced_test]
    fn test_div_assign() {
        info!("Testing ArithU256 DivAssign with u32, i64, &ArithU256...");

        // (a) div_assign<u32>
        {
            let mut x = ArithU256::from(0x1234_5678u64);
            let divisor = 0x1234u32;

            trace!(
                "(a) Before: x=0x{:08X}, divisor=0x{:04X}, expecting ~65540 in low-limb",
                x.base.low64(),
                divisor
            );

            x /= divisor; // This eventually yields 65540 for the low limb.

            let got = x.base.get_limb(0);
            trace!(
                "(a) After: x.low64()=0x{:08X}, got limb[0]=0x{:X} ({})",
                x.base.low64(),
                got,
                got
            );

            // OLD line was: assert_eq!(got, 0x5678);  // now we fix it:
            assert_eq!(
                got, 
                0x10004, // decimal 65540
                "Div: 0x12345678 / 0x1234 => mismatch in low32"
            );
        }

        // (b) div_assign<i64>
        {
            let mut y = ArithU256::from(1000u64);
            let divisor_i64 = 10i64;

            trace!(
                "(b) Before: y=0x{:08X}, divisor_i64={}, expecting result=100 decimal in low-limb",
                y.base.low64(),
                divisor_i64
            );

            y /= divisor_i64;

            let got = y.base.get_limb(0);
            trace!(
                "(b) After: y.low64()=0x{:08X}, got limb[0]={}, expected=100 decimal",
                y.base.low64(),
                got
            );

            assert_eq!(got, 100);
        }

        // (b2) negative => panic check
        {
            let mut z = ArithU256::from(12345u64);
            let caught_neg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Tracing around the call
                trace!("(b2) about to do z /= -5i64 => should panic");
                z /= -5i64;
            }));
            assert!(caught_neg.is_err(), "div_assign negative i64 => should panic");
        }

        // (c) div_assign<&ArithU256>
        {
            let mut a = ArithU256::from(12345u64);
            let b = ArithU256::from(5u64);

            trace!(
                "(c) Before: a=0x{:08X}, b=0x{:08X}, expecting a=2469 decimal in low-limb afterward",
                a.base.low64(),
                b.base.low64(),
            );

            a /= &b;

            let got = a.base.get_limb(0);
            trace!(
                "(c) After: a=0x{:08X}, limb[0]={}, expected=2469",
                a.base.low64(),
                got
            );

            assert_eq!(got, 2469, "12345 / 5 => 2469 decimal in the low limb");
        }

        // division by zero => confirm panic
        {
            let mut zero_div = ArithU256::from(999u64);
            let zero_base = ArithU256::from(0u64);

            trace!(
                "About to divide 0x{:08X} by 0 => expecting panic",
                zero_div.base.low64()
            );

            let caught_zd = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                zero_div /= &zero_base; // => panic
            }));
            assert!(caught_zd.is_err(), "divide by zero => must panic for ArithU256");
        }

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
