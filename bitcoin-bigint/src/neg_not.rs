crate::ix!();

impl<const BITS: usize> Not for BaseUInt<BITS>
where
    [(); BITS / 32]:
{
    type Output = BaseUInt<BITS>;

    #[inline]
    fn not(self) -> Self::Output {
        // Bitwise NOT (~) each 32-bit limb
        let mut ret = Self::default();
        for (i, &val) in self.pn.iter().enumerate() {
            ret.pn[i] = !val;
        }
        ret
    }
}

impl<const BITS: usize> Neg for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = Self;

    /// Two's complement negation.
    /// In C++: `ret = ~self; ++ret;`
    /// This is effectively  `-x = (~x + 1)`
    #[inline]
    fn neg(self) -> Self::Output {
        let mut ret = Self::default();
        // Invert bits
        for (i, &val) in self.pn.iter().enumerate() {
            ret.pn[i] = !val;
        }
        // Now add 1 to complete two's complement
        let mut carry = 1u64;
        for limb in ret.pn.iter_mut() {
            let sum = *limb as u64 + carry;
            *limb = (sum & 0xffffffff) as u32;
            carry = sum >> 32;
            if carry == 0 {
                break; // No more carry, done.
            }
        }
        ret
    }
}

#[cfg(test)]
mod not_neg_exhaustive_tests {
    use super::*;

    /// Returns a random `BaseUInt<64>` using the given `SimpleLCG`.
    /// This is just for demonstration; it packs one `u64` into the two limbs for 64 bits.
    fn random_u64_baseuint(rng: &mut SimpleLCG) -> BaseUInt<64> {
        let r = rng.next_u64();
        BaseUInt::<64>::from(r)
    }

    /// Creates a `BaseUInt<64>` whose bits are all set to `1`.
    /// This is `0xFFFF_FFFF_FFFF_FFFF` in 64-bit representation.
    fn all_ones_64() -> BaseUInt<64> {
        BaseUInt::<64>::from(0xFFFF_FFFF_FFFF_FFFFu64)
    }

    /// Creates a `BaseUInt<256>` whose bits are all set to `1`.
    /// This is `0xFFFF...FFFF` (256 bits total).
    fn all_ones_256() -> BaseUInt<256> {
        // 256 bits => 8 limbs of 32 bits each, each = 0xFFFF_FFFF
        let mut x = BaseUInt::<256>::default();
        for limb in x.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        x
    }

    /// Show that applying `!x` flips all bits, and that double-not `!!x` returns `x`.
    #[traced_test]
    fn test_not_basic_64() {
        trace!("Testing bitwise NOT for BaseUInt<64> on basic edge cases.");

        let zero = BaseUInt::<64>::default();
        let not_zero = !zero.clone();
        debug!("not(0) => {:?}", not_zero);
        // For 64 bits, !0 should be all ones:
        assert_eq!(not_zero, all_ones_64(), "!0 should be all bits set.");

        let ones = all_ones_64();
        let not_ones = !ones.clone();
        debug!("not(all_ones) => {:?}", not_ones);
        // Should give 0 back
        assert_eq!(not_ones, zero, "!all_ones should be 0.");

        // Check double NOT
        let x = BaseUInt::<64>::from(0x1234_ABCD_0000_4444u64);
        let double_not = !!x.clone();
        debug!("x={:?}, !!x={:?}", x, double_not);
        assert_eq!(x, double_not, "Double NOT should return original value.");
    }

    /// Test random values for bitwise NOT in 64 bits.
    /// We'll confirm that `!x` == `x ^ 0xFFFF_FFFF_FFFF_FFFF`.
    #[traced_test]
    fn test_not_random_64() {
        trace!("Testing bitwise NOT for BaseUInt<64> on random values.");

        let mut rng = SimpleLCG::new(0xDEAD_BEEF_CAFE_F00D);
        for _ in 0..20 {
            let x = random_u64_baseuint(&mut rng);
            let not_x = !x.clone();
            let expected = x.clone() ^ &BaseUInt::<64>::from(0xFFFF_FFFF_FFFF_FFFFu64);
            debug!("x={:?}, !x={:?}, x ^ all_ones={:?}", x, not_x, expected);
            assert_eq!(
                not_x, expected,
                "Bitwise NOT should match XOR with all bits set."
            );
        }
    }

    /// Test negation on zero and all-ones for 64 bits.
    /// In two's complement, `-x` is `!x + 1`.
    #[traced_test]
    fn test_neg_basic_64() {
        trace!("Testing negation for BaseUInt<64> on basic edge cases.");

        let zero = BaseUInt::<64>::default();
        let neg_zero = -zero.clone();
        debug!("-(0) => {:?}", neg_zero);
        assert_eq!(neg_zero, zero, "-0 should be 0 in two's complement.");

        let ones = all_ones_64();
        let neg_ones = -ones.clone();
        debug!("-(all_ones_64) => {:?}", neg_ones);
        // - (0xFFFF_FFFF_FFFF_FFFF) = 1? Let's see:
        //   ~ones = 0, then +1 => 1 in 64-bit representation
        assert_eq!(neg_ones, BaseUInt::<64>::from(1u64), "-all_ones should yield 1.");
    }

    /// Check random negation properties in 64 bits:
    /// (1) `-x` should satisfy `x + (-x) == 0` for all x.
    /// (2) `-x` should be `!x + 1`.
    #[traced_test]
    fn test_neg_random_64() {
        trace!("Testing negation for BaseUInt<64> on random values.");

        let mut rng = SimpleLCG::new(0x1234_5678_9ABC_DEF0);
        for _ in 0..20 {
            let x = random_u64_baseuint(&mut rng);
            let minus_x = -x.clone();
            let sum = x.clone() + &minus_x;
            debug!("x={:?}, -x={:?}, (x + -x)={:?}", x, minus_x, sum);
            assert_eq!(
                sum,
                BaseUInt::<64>::default(),
                "x + (-x) should be 0 for all x in two's complement."
            );

            // Also check that -x == (!x + 1).
            let not_x_plus_one = (!x.clone()) + &BaseUInt::<64>::from(1u64);
            debug!("not(x) + 1 = {:?}", not_x_plus_one);
            assert_eq!(minus_x, not_x_plus_one, "Negation must match ~x + 1.");
        }
    }

    /// Check some random tests for bitwise NOT and negation in 256 bits.
    /// We'll rely on the existing `random_u256` function in the crate.
    #[traced_test]
    fn test_not_neg_random_256() {
        trace!("Testing NOT/NEG for BaseUInt<256> on random values.");

        let mut rng = SimpleLCG::new(0x0FF1CE_BADC0FFE_u64);
        for _ in 0..20 {
            let x = random_u256(&mut rng);
            let not_x = !x.clone();
            let neg_x = -x.clone();
            // Check ~x + 1 = -x
            let check_neg = (!x.clone()) + &BaseUInt::<256>::from(1u64);
            debug!(
                "x={:?}\n!x={:?}\n-x={:?}\n(!x+1)={:?}",
                x, not_x, neg_x, check_neg
            );
            assert_eq!(neg_x, check_neg, "Negation must match ~x + 1 in 256 bits.");

            // Check that x + (-x) == 0
            let sum = x.clone() + &neg_x;
            assert_eq!(
                sum,
                BaseUInt::<256>::default(),
                "x + (-x) should be 0 for 256-bit as well."
            );

            // Also check that !x == x ^ (all_ones_256).
            let expected_not = x.clone() ^ &all_ones_256();
            assert_eq!(not_x, expected_not, "bitwise NOT must match XOR with all bits set (256).");
        }
    }
}
