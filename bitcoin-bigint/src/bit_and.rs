// ---------------- [ File: bitcoin-bigint/src/bit_and.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_baseuint_bitand {

    ($uint_type:ident, $bits:expr, $limbs:expr) => {

        impl core::ops::BitAnd<&$uint_type> for $uint_type {
            type Output = $uint_type;
            fn bitand(self, other: &$uint_type) -> Self::Output {
                let mut ret = self.clone();
                ret &= other;
                ret
            }
        }

        impl core::ops::BitAndAssign<&$uint_type> for $uint_type {
            #[inline]
            fn bitand_assign(&mut self, b: &$uint_type) {
                for i in 0..$limbs {
                    self.pn[i] &= b.pn[i];
                }
            }
        }

        impl core::ops::BitAndAssign<u64> for $uint_type {
            fn bitand_assign(&mut self, b: u64) {
                self.pn[0] &= (b & 0xffff_ffff) as u32;
                if $limbs > 1 {
                    self.pn[1] &= ((b >> 32) & 0xffff_ffff) as u32;
                }
                for i in 2..$limbs {
                    self.pn[i] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod bitwise_and_exhaustive_tests {
    use super::*;
    use tracing::{debug, error, info, trace};
    use std::cmp::Ordering;

    #[traced_test]
    fn test_bitand_assign_basic_64() {
        info!("Verifying basic AND assignment (self &= other) with 64-bit BaseUInt.");

        type U64 = BaseUInt64;

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
        type U64 = BaseUInt64;

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

        type U256 = BaseUInt256;

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

        type U256 = BaseUInt256;
        let mut rng = SimpleLCG::new(0x1234_5678_DEAD_BEEF);

        for i in 0..10 {
            let mut limbs_a = [0u32; 8];
            let mut limbs_b = [0u32; 8];
            for j in 0..8 {
                limbs_a[j] = rng.next_u64() as u32;
                limbs_b[j] = rng.next_u64() as u32;
            }
            let a = from_limbs_256(&limbs_a);
            let b = from_limbs_256(&limbs_b);

            trace!("Test iteration {} => a={:?}, b={:?}", i, a, b);

            let c = a.clone() & &b;
            let c_limbs = to_limbs_256(&c);

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

        type U64 = BaseUInt64;

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

    /// A trivial LCG for reproducible pseudo-random tests.
    struct SimpleRng(u64);

    impl SimpleRng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next_u64(&mut self) -> u64 {
            // linear congruential generator step
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
    }

    /// Check that `BaseUInt<BITS>::bitand_assign(u64)` behaves as expected in basic edge cases.
    #[test]
    fn test_bitand_assign_u64_basics() {
        // 1) 0 & <any> => 0
        {
            let mut x = BaseUInt64::default(); // x=0
            x &= 0xFFFF_FFFF_FFFF_FFFFu64; // all ones
            assert_eq!(x.pn[0], 0, "0 & allones => 0, lower-limb");
            assert_eq!(x.pn[1], 0, "0 & allones => 0, upper-limb");
        }

        // 2) all-ones & <some mask> => partial
        {
            // all-ones in 64 bits => 0xFFFF_FFFF_FFFF_FFFF
            let mut x = BaseUInt64::default();
            x.pn[0] = 0xFFFF_FFFF;
            x.pn[1] = 0xFFFF_FFFF;

            // choose a mask
            let mask = 0x0000_FFFF_1234_5678u64;
            x &= mask;

            // let's do the AND in normal 64-bit rust
            let expected = 0xFFFF_FFFF_FFFF_FFFFu64 & mask;

            // check each limb
            let low32 = (expected & 0xFFFF_FFFF) as u32;
            let high32 = ((expected >> 32) & 0xFFFF_FFFF) as u32;
            assert_eq!(x.pn[0], low32);
            assert_eq!(x.pn[1], high32);
        }

        // 3) partial usage in bigger BITS, e.g. 256 bits: (the extra limbs must be zeroed)
        {
            let mut x = BaseUInt256::default();
            // Fill first 3 limbs with random data
            x.pn[0] = 0xDEAD_BEEF;
            x.pn[1] = 0xAAAA_5555;
            x.pn[2] = 0xFF00_FF00;
            x.pn[3] = 0x1234_5678; // etc

            x &= 0xFFFF_FFFF_0000_1111u64;
            // This zeroes out limbs[2..], so x.pn[2..] => 0
            // low limb => 0x0000_1111 & 0xDEAD_BEEF => 0x0000_0???
            // second limb => 0xFFFF_FFFF & 0xAAAA_5555 => same => 0xAAAA_5555
            // actually wait, 64 bits => lower 32 => pn[0], next 32 => pn[1], the rest => 0
            // let's do a direct check:

            let expected64 = (0xFFFF_FFFF_0000_1111u64) & (
                (x.pn[0] as u64)
                | ((x.pn[1] as u64) << 32)
            );
            let ex_low = (expected64 & 0xFFFF_FFFF) as u32;
            let ex_high = ((expected64 >> 32) & 0xFFFF_FFFF) as u32;

            // Now x must match those two limbs, plus zeros above
            assert_eq!(x.pn[0], ex_low);
            assert_eq!(x.pn[1], ex_high);
            for i in 2..8 {
                assert_eq!(x.pn[i], 0);
            }
        }
    }

    /// Random test for `BaseUInt<BITS>::bitand_assign(u64)`.
    /// We ensure it matches the “mod 2^BITS”  operation
    /// on each limb for up to 64 bits.
    #[test]
    fn test_bitand_assign_u64_random() {
        let mut rng = SimpleRng::new(0xDEAD_BEEF_1234_5678);

        // We'll do, say, 100 random checks for 64-bit size:
        for _ in 0..100 {
            let random_a = rng.next_u64(); 
            let random_mask = rng.next_u64();

            let mut x = BaseUInt64::from(random_a);
            // The “expected” is just random_a & random_mask in normal Rust:
            let expected_64 = random_a & random_mask;

            x &= random_mask; // calls the new bitand_assign(u64)
            
            // Now check each limb
            let low32 = (expected_64 & 0xffff_ffff) as u32;
            let high32 = ((expected_64 >> 32) & 0xffff_ffff) as u32;
            assert_eq!(x.pn[0], low32);
            assert_eq!(x.pn[1], high32);
        }

        // We can do the same for 256 bits, or 128, etc.
        // Let's do 20 random checks for 256 bits:
        for _ in 0..20 {
            let mut big = BaseUInt256::default();
            // fill 8 limbs with random
            for i in 0..8 {
                big.pn[i] = rng.next_u64() as u32;
            }

            let mask64 = rng.next_u64();

            // Let's do it manually: the lower 64 bits of `big` get &'ed with mask64,
            // the rest become zero. We'll do that logic in a local "expected" copy:
            let mut expected = big.clone();

            // interpret the lower 64 bits from pn[0..2]
            let orig_low64 = (expected.pn[0] as u64)
                | ((expected.pn[1] as u64) << 32);
            let new_low64 = orig_low64 & mask64;
            expected.pn[0] = (new_low64 & 0xffff_ffff) as u32;
            expected.pn[1] = ((new_low64 >> 32) & 0xffff_ffff) as u32;
            // higher limbs => 0
            for i in 2..8 {
                expected.pn[i] = 0;
            }

            // Now do the actual operation
            let mut actual = big.clone();
            actual &= mask64;

            // Compare
            assert_eq!(actual.get_hex(), expected.get_hex(),
                       "bitand_assign<u64> mismatch in 256-bit random test.\n  big was {},\n  mask64=0x{:016X}",
                       big.get_hex(),
                       mask64);
        }
    }
}
