// ---------------- [ File: bitcoin-bigint/src/sub_assign.rs ]
crate::ix!();

impl<const BITS: usize> SubAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self -= other  =>  self = self + (-other)
    #[inline]
    fn sub_assign(&mut self, other: &BaseUInt<BITS>) {
        *self += &(-other.clone()); // We rely on our `Neg` impl.
    }
}

impl<const BITS: usize> SubAssign<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self -= u64  =>  self += -u64
    #[inline]
    fn sub_assign(&mut self, other: u64) {

        let mut b = Self::default();

        b.pn[0] = (other & 0xffff_ffff) as u32;

        if BITS / 32 > 1 {
            b.pn[1] = ((other >> 32) & 0xffff_ffff) as u32;
        }

        *self += &(-b);
    }
}

#[cfg(test)]
mod test_sub_assign {
    use super::*;

    /// Verify subtracting two 64-bit BaseUInts in basic edge cases (BITS=64).
    #[traced_test]
    fn basic_sub_assign_ref_64() {
        info!("Starting basic_sub_assign_ref_64 tests...");
        // 1) 0 - 0 => 0
        let mut a = BaseUInt::<64>::default();
        let b = BaseUInt::<64>::default();
        debug!("Initial: a=0, b=0");
        a -= &b;
        trace!("After sub: a={:?}", a);
        assert_eq!(a.low64(), 0, "0 - 0 should yield 0");

        // 2) 1 - 1 => 0
        let mut a = BaseUInt::<64>::from(1u64);
        let b = BaseUInt::<64>::from(1u64);
        debug!("Initial: a=1, b=1");
        a -= &b;
        trace!("After sub: a={:?}", a);
        assert_eq!(a.low64(), 0, "1 - 1 should yield 0");

        // 3) 1 - 2 => 2^64 - 1 (wrap-around in 64-bit sense)
        let mut a = BaseUInt::<64>::from(1u64);
        let b = BaseUInt::<64>::from(2u64);
        debug!("Initial: a=1, b=2");
        a -= &b;
        trace!("After sub: a={:?}", a);
        // 2^64 - 1 is 0xFFFF_FFFF_FFFF_FFFF
        let expected = u64::MAX;
        assert_eq!(a.low64(), expected, "1 - 2 should wrap to 2^64 - 1");

        info!("basic_sub_assign_ref_64 tests passed.");
    }

    /// Randomized tests for subtracting two 64-bit BaseUInts (BITS=64).
    /// We confirm (a - b) mod 2^64 by comparing to a 128-bit reference calculation.
    #[traced_test]
    fn random_sub_assign_ref_64() {
        info!("Starting random_sub_assign_ref_64 tests...");
        let mut rng = SimpleLCG::new(0xACE0FF1CE);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();

            let mut a = BaseUInt::<64>::from(x_64);
            let b = BaseUInt::<64>::from(y_64);

            trace!("a={:#x}, b={:#x}", x_64, y_64);
            a -= &b;

            // Expected is (x_64 - y_64) mod 2^64.
            let expected = x_64.wrapping_sub(y_64);
            debug!("Expected result = 0x{:016X}", expected);
            debug!("Actual BaseUInt<64> after sub = {:?}", a);

            assert_eq!(
                a.low64(),
                expected,
                "Mismatch in random_sub_assign_ref_64 for x={:#x}, y={:#x}",
                x_64,
                y_64
            );
        }

        info!("random_sub_assign_ref_64 tests passed.");
    }

    /// Verify subtracting a u64 from a 64-bit BaseUInt (BITS=64) in basic edge cases.
    #[traced_test]
    fn basic_sub_assign_u64_64() {
        info!("Starting basic_sub_assign_u64_64 tests...");
        // 1) 0 - 0 => 0
        let mut a = BaseUInt::<64>::default();
        let b: u64 = 0;
        debug!("Initial: a=0, b=0");
        a -= b;
        trace!("After sub: a={:?}", a);
        assert_eq!(a.low64(), 0, "0 - 0 should yield 0");

        // 2) 5 - 5 => 0
        let mut a = BaseUInt::<64>::from(5u64);
        let b: u64 = 5;
        debug!("Initial: a=5, b=5");
        a -= b;
        trace!("After sub: a={:?}", a);
        assert_eq!(a.low64(), 0, "5 - 5 should yield 0");

        // 3) 0 - 1 => 2^64 - 1
        let mut a = BaseUInt::<64>::default();
        let b: u64 = 1;
        debug!("Initial: a=0, b=1");
        a -= b;
        trace!("After sub: a={:?}", a);
        let expected = u64::MAX;
        assert_eq!(a.low64(), expected, "0 - 1 should wrap to 2^64 - 1");

        info!("basic_sub_assign_u64_64 tests passed.");
    }

    /// Randomized tests for subtracting a u64 from a 64-bit BaseUInt (BITS=64).
    /// We confirm (a - b) mod 2^64 by comparing to a 128-bit reference calculation.
    #[traced_test]
    fn random_sub_assign_u64_64() {
        info!("Starting random_sub_assign_u64_64 tests...");
        let mut rng = SimpleLCG::new(0xF00D_BEEF);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();

            let mut a = BaseUInt::<64>::from(x_64);
            trace!("a={:#x}, b={:#x}", x_64, y_64);
            a -= y_64;

            let expected = x_64.wrapping_sub(y_64);
            debug!("Expected result = 0x{:016X}", expected);
            debug!("Actual BaseUInt<64> after sub = {:?}", a);

            assert_eq!(
                a.low64(),
                expected,
                "Mismatch in random_sub_assign_u64_64 for x={:#x}, y={:#x}",
                x_64,
                y_64
            );
        }

        info!("random_sub_assign_u64_64 tests passed.");
    }

    /// Verify subtracting two 256-bit BaseUInts in basic edge cases (BITS=256).
    #[traced_test]
    fn basic_sub_assign_ref_256() {
        info!("Starting basic_sub_assign_ref_256 tests...");
        // 1) 0 - 0 => 0
        let mut a = BaseUInt::<256>::default();
        let b = BaseUInt::<256>::default();
        debug!("Initial: a=0, b=0");
        a -= &b;
        trace!("After sub: a={:?}", a);
        assert_eq!(a.compare_to(&b), 0, "0 - 0 should yield 0");

        // 2) 1 - 1 => 0
        let mut a = BaseUInt::<256>::from(1u64);
        let b = BaseUInt::<256>::from(1u64);
        debug!("Initial: a=1, b=1");
        a -= &b;
        trace!("After sub: a={:?}", a);
        let zero = BaseUInt::<256>::default();
        assert_eq!(a.compare_to(&zero), 0, "1 - 1 should yield 0");

        // 3) Large edge: (2^256 - 1) - 1 => 2^256 - 2
        // Construct (2^256 - 1)
        let mut all_ones = BaseUInt::<256>::default();
        for limb in all_ones.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        let mut a = all_ones.clone();
        let b = BaseUInt::<256>::from(1u64);
        debug!("Initial: a=(2^256 - 1), b=1");
        a -= &b;
        trace!("After sub: a={:?}", a);
        // Expected => (2^256 - 2) => that means all_ones except the lowest limb is 0xFFFF_FFFE
        let mut expected_all_ones = all_ones.clone();
        expected_all_ones.pn[0] = 0xFFFF_FFFE;
        assert_eq!(
            a.pn, expected_all_ones.pn,
            "(2^256 - 1) - 1 should yield (2^256 - 2)."
        );

        info!("basic_sub_assign_ref_256 tests passed.");
    }

    /// Randomized tests for subtracting two 256-bit BaseUInts (BITS=256).
    /// We verify correctness by artificially limiting the random values so they fit in 128 bits,
    /// then do a 128-bit reference calculation (wrap mod 2^256 is the same if each operand < 2^128).
    #[traced_test]
    fn random_sub_assign_ref_256() {
        info!("Starting random_sub_assign_ref_256 tests...");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF);

        for _ in 0..100 {
            // artificially limit to 128-bit by ignoring the upper 64 bits
            let x_128 = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
            let y_128 = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);

            let mut a = BaseUInt::<256>::default();
            a.pn[0] = (x_128 & 0xFFFF_FFFF) as u32;
            a.pn[1] = ((x_128 >> 32) & 0xFFFF_FFFF) as u32;
            a.pn[2] = ((x_128 >> 64) & 0xFFFF_FFFF) as u32;
            a.pn[3] = ((x_128 >> 96) & 0xFFFF_FFFF) as u32;

            let mut b = BaseUInt::<256>::default();
            b.pn[0] = (y_128 & 0xFFFF_FFFF) as u32;
            b.pn[1] = ((y_128 >> 32) & 0xFFFF_FFFF) as u32;
            b.pn[2] = ((y_128 >> 64) & 0xFFFF_FFFF) as u32;
            b.pn[3] = ((y_128 >> 96) & 0xFFFF_FFFF) as u32;

            trace!("a(pn[0..4])={:?}, b(pn[0..4])={:?}", &a.pn[0..4], &b.pn[0..4]);
            a -= &b;

            // reference => (x_128 - y_128) mod 2^128
            // but since each is < 2^128, the difference mod 2^128 is just x_128.wrapping_sub(y_128).
            let expected_128 = x_128.wrapping_sub(y_128);
            let mut expected = BaseUInt::<256>::default();
            expected.pn[0] = (expected_128 & 0xFFFF_FFFF) as u32;
            expected.pn[1] = ((expected_128 >> 32) & 0xFFFF_FFFF) as u32;
            expected.pn[2] = ((expected_128 >> 64) & 0xFFFF_FFFF) as u32;
            expected.pn[3] = ((expected_128 >> 96) & 0xFFFF_FFFF) as u32;

            debug!(
                "Expected pn[0..4]={:08X?}, Actual pn[0..4]={:08X?}",
                &expected.pn[0..4], &a.pn[0..4]
            );
            assert_eq!(
                a.pn[0..4],
                expected.pn[0..4],
                "Mismatch in random_sub_assign_ref_256 for x_128={:#x}, y_128={:#x}",
                x_128,
                y_128
            );
        }

        info!("random_sub_assign_ref_256 tests passed.");
    }

    /// Verify subtracting a u64 from a 256-bit BaseUInt (BITS=256) in basic edge cases.
    #[traced_test]
    fn basic_sub_assign_u64_256() {
        info!("Starting basic_sub_assign_u64_256 tests...");
        // 1) 0 - 0 => 0
        let mut a = BaseUInt::<256>::default();
        let b: u64 = 0;
        debug!("Initial: a=0, b=0");
        a -= b;
        trace!("After sub: a={:?}", a);
        assert!(a.equal_to(0), "0 - 0 should yield 0");

        // 2) 100 - 100 => 0
        let mut a = BaseUInt::<256>::from(100u64);
        let b: u64 = 100;
        debug!("Initial: a=100, b=100");
        a -= b;
        trace!("After sub: a={:?}", a);
        assert!(a.equal_to(0), "100 - 100 should yield 0");

        // 3) 0 - 1 => wrap-around => 2^256 - 1
        let mut a = BaseUInt::<256>::default();
        let b: u64 = 1;
        debug!("Initial: a=0, b=1");
        a -= b;
        trace!("After sub: a={:?}", a);
        // Now a should be (2^256 - 1)
        let mut all_ones = BaseUInt::<256>::default();
        for limb in all_ones.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        assert_eq!(
            a.pn, all_ones.pn,
            "0 - 1 should wrap to 2^256 - 1"
        );

        info!("basic_sub_assign_u64_256 tests passed.");
    }

    /// Randomized tests for subtracting a u64 from a 256-bit BaseUInt (BITS=256).
    /// We only generate 64-bit random values for the big integer so we can do 128-bit reference arithmetic easily.
    #[traced_test]
    fn random_sub_assign_u64_256() {
        info!("Starting random_sub_assign_u64_256 tests...");
        let mut rng = SimpleLCG::new(0xFACE_FEED);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();

            let mut a = BaseUInt::<256>::from(x_64);
            trace!("a={:#x}, b={:#x}", x_64, y_64);
            a -= y_64;

            // Expect (x_64 - y_64) mod 2^64 (and stored in the low bits of the 256-bit).
            let expected_64 = x_64.wrapping_sub(y_64);
            debug!("Expected low 64 bits = 0x{:016X}", expected_64);
            debug!("Actual BaseUInt<256> after sub = {:?}", a);

            // The low 64 bits of `a` must match `expected_64`.
            assert_eq!(
                a.low64(),
                expected_64,
                "Mismatch in random_sub_assign_u64_256 for x={:#x}, y={:#x}",
                x_64,
                y_64
            );
        }

        info!("random_sub_assign_u64_256 tests passed.");
    }
}
