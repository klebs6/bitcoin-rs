crate::ix!();

// Bitwise AND (self & &other => new BaseUInt)
impl<const BITS: usize> BitAnd<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn bitand(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        ret &= other;
        ret
    }
}

impl<const BITS: usize> BitAndAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Bitwise AND assignment: `self &= other`
    #[inline]
    fn bitand_assign(&mut self, b: &BaseUInt<BITS>) {
        for i in 0..(BITS / 32) {
            self.pn[i] &= b.pn[i];
        }
    }
}

#[cfg(test)]
mod bitwise_and_exhaustive_tests {
    use super::*;
    use tracing::{debug, error, info, trace};
    use std::cmp::Ordering;

    /// A very simple pseudo-random generator for repeatable tests.
    /// We'll use something like an LCG for demonstration.
    struct SimpleLCG {
        state: u64,
    }

    impl SimpleLCG {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next_u64(&mut self) -> u64 {
            // standard LCG step
            self.state = self
                .state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.state
        }
    }

    /// Helper to create a `BaseUInt<BITS>` from an array of 32-bit limbs (LE order).
    fn from_limbs<const BITS: usize>(limbs: &[u32]) -> BaseUInt<BITS>
    where
        [(); BITS / 32]:,
    {
        let mut x = BaseUInt::<BITS>::default();
        let count = BITS / 32;
        for (i, &val) in limbs.iter().take(count).enumerate() {
            x.pn[i] = val;
        }
        x
    }

    /// Helper to view the limbs of a `BaseUInt<BITS>` as a Vec<u32>.
    fn to_limbs<const BITS: usize>(v: &BaseUInt<BITS>) -> Vec<u32>
    where
        [(); BITS / 32]:,
    {
        v.pn.to_vec()
    }

    #[traced_test]
    fn test_bitand_assign_basic_64() {
        info!("Verifying basic AND assignment (self &= other) with 64-bit BaseUInt.");

        type U64 = BaseUInt<64>;

        // 1) Zero AND anything => zero
        let mut x = U64::default(); // 0
        let y = U64::from(0xFFFF_FFFF_1234_5678u64);
        trace!("x = 0x{:016X}, y = 0x{:016X}", x.low64(), y.low64());
        x &= &y;
        assert_eq!(x.low64(), 0, "0 & anything => 0");

        // 2) Full-ones AND partial
        let mut all_ones = U64::default();
        all_ones.pn[0] = 0xFFFF_FFFF;
        all_ones.pn[1] = 0xFFFF_FFFF;
        trace!("all_ones = {:?}", all_ones);

        let partial = U64::from(0xAABB_CCDD_1122_3344u64);
        trace!("partial = {:?}", partial);
        let expected_and = 0xAABB_CCDD_1122_3344u64; // identical to partial
        all_ones &= &partial;
        trace!("Result after &= partial => {:?}", all_ones);
        assert_eq!(all_ones.low64(), expected_and);

        // 3) Small example: 0xFFFF0000 & 0x1234FFFF = 0x12340000 (lower 32 bits for demonstration)
        //    We'll store that in 64 bits but effectively it's about the low limb.
        let mut a = U64::from(0xFFFF_0000u64);
        let b = U64::from(0x1234_FFFFu64);
        a &= &b;
        let res = a.low64();
        trace!("0xFFFF0000 & 0x1234FFFF => 0x{:08X}", res as u32);
        assert_eq!(res, 0x1234_0000);

        info!("Basic AND assignment test (64-bit) passed.");
    }

    #[traced_test]
    fn test_bitand_assign_random_64() {
        info!("Testing random AND assignment for 64-bit BaseUInt.");
        type U64 = BaseUInt<64>;

        let mut rng = SimpleLCG::new(0xBEEF_0000_DEAD_0000);

        for i in 0..20 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();
            let mut a = U64::from(a_val);
            let b = U64::from(b_val);

            let expected = a_val & b_val; // standard bitwise AND
            trace!("Iter {}: a=0x{:016X}, b=0x{:016X}, a&b=0x{:016X}", i, a_val, b_val, expected);

            a &= &b;
            let got = a.low64();
            assert_eq!(got, expected, "Mismatch in random 64-bit &=");
        }

        info!("Random AND assignment test (64-bit) succeeded.");
    }

    #[traced_test]
    fn test_bitand_assign_256_exhaustive_edges() {
        info!("Testing bitwise AND assignment for a few edge cases in 256-bit BaseUInt.");

        type U256 = BaseUInt<256>;

        // 1) All zero with all zero => zero
        let mut z1 = U256::default();
        let z2 = U256::default();
        z1 &= &z2;
        for &limb in z1.pn.iter() {
            assert_eq!(limb, 0, "0 & 0 => 0 for each limb");
        }

        // 2) Zero & full-ones => zero
        let mut zero = U256::default();
        let mut full = U256::default();
        for i in 0..8 {
            full.pn[i] = 0xFFFF_FFFF;
        }
        zero &= &full;
        for &limb in zero.pn.iter() {
            assert_eq!(limb, 0);
        }

        // 3) partial pattern vs partial pattern in multiple limbs
        let mut a = U256::default();
        let mut b = U256::default();
        // Let's fill a with 0x1111_2222 in limb0, 0x3333_4444 in limb1, ...
        // b with 0xFFFF_0000 in each limb, see how it lines up.
        a.pn[0] = 0x1111_2222;
        a.pn[1] = 0x3333_4444;
        a.pn[2] = 0x5555_6666;
        a.pn[3] = 0x7777_8888;
        a.pn[4] = 0x9999_AAAA;
        a.pn[5] = 0xBBBB_CCCC;
        a.pn[6] = 0xDDDD_EEEE;
        a.pn[7] = 0xFFFF_0000;

        for i in 0..8 {
            b.pn[i] = 0xFFFF_0000;
        }

        let mut copy_a = a.clone();
        copy_a &= &b;
        trace!("After &= b => copy_a = {:?}", copy_a);

        // We expect each limb to become (original & 0xFFFF_0000).
        // So let's do that bitwise for each limb of a:
        for i in 0..8 {
            let expected = a.pn[i] & 0xFFFF_0000;
            assert_eq!(copy_a.pn[i], expected, "Mismatch limb {i} in 256-bit AND");
        }

        info!("Edge-case coverage for 256-bit AND assignment passed.");
    }

    #[traced_test]
    fn test_bitand_operator_256_random() {
        info!("Random test of bitwise AND operator for 256-bit BaseUInt.");

        type U256 = BaseUInt<256>;
        let mut rng = SimpleLCG::new(0x1234_5678_DEAD_BEEF);

        for i in 0..10 {
            let mut limbs_a = [0u32; 8];
            let mut limbs_b = [0u32; 8];
            for j in 0..8 {
                limbs_a[j] = rng.next_u64() as u32;
                limbs_b[j] = rng.next_u64() as u32;
            }
            let a = from_limbs::<256>(&limbs_a);
            let b = from_limbs::<256>(&limbs_b);

            trace!("Test iteration {} => a={:?}, b={:?}", i, a, b);

            let c = a.clone() & &b;
            let c_limbs = to_limbs(&c);

            for (idx, &val_c) in c_limbs.iter().enumerate() {
                let expected = limbs_a[idx] & limbs_b[idx];
                assert_eq!(val_c, expected, "Mismatch in random 256-bit & for limb {idx}");
            }
        }

        info!("Random operator (&) checks passed for 256-bit BaseUInt.");
    }

    #[traced_test]
    fn test_bitand_operator_64() {
        info!("Testing the bitand operator (self & &other) in 64-bit BaseUInt.");

        type U64 = BaseUInt<64>;

        // 1) Simple demonstration
        let x = U64::from(0xDEAD_BEEF_1234_5678u64);
        let y = U64::from(0xFFFF_0000_FFFF_0000u64);
        let result = x.clone() & &y;
        // => (DEAD_BEEF_1234_5678 & FFFF_0000_FFFF_0000) => DEAD_0000_1234_0000
        let expected = 0xDEAD_0000_1234_0000u64;
        trace!("x={:?}, y={:?}, result={:?}, expected=0x{:016X}", x, y, result, expected);
        assert_eq!(result.low64(), expected);

        // 2) Another approach: partial vs. partial
        let a = U64::from(0x0123_4567_89AB_CDEFu64);
        let b = U64::from(0xF0F0_F0F0_F0F0_F0F0u64);
        let c = a.clone() & &b;

        // Corrected expected value (bitwise AND each nibble/byte shows that 0x45 & 0xF0 -> 0x40)
        let exp = 0x0020_4060_80A0_C0E0u64;
        trace!(
            "a=0x{:016X}, b=0x{:016X}, a&b=0x{:016X}, exp=0x{:016X}",
            a.low64(),
            b.low64(),
            c.low64(),
            exp
        );
        assert_eq!(c.low64(), exp);

        info!("Bitwise AND operator test (64-bit) passed.");
    }
}
