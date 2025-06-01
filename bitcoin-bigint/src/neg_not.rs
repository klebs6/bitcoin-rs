// ---------------- [ File: bitcoin-bigint/src/neg_not.rs ]
crate::ix!();

// ---------------------------------------------------------------------------
// 4) Macro for Not, Neg plus tests
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! define_base_uint_not_neg {
    ($name:ident, $bits:expr, $limbs:expr) => {

        impl core::ops::Not for $name {
            type Output = $name;
            #[inline]
            fn not(self) -> Self::Output {
                let mut ret = Self::default();
                for (i, &val) in self.pn.iter().enumerate() {
                    ret.pn[i] = !val;
                }
                ret
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output {
                let mut ret = !self; // bitwise not
                // plus 1
                let mut carry = 1u64;
                for limb in ret.pn.iter_mut() {
                    let sum = *limb as u64 + carry;
                    *limb = (sum & 0xFFFF_FFFF) as u32;
                    carry = sum >> 32;
                    if carry == 0 {
                        break;
                    }
                }
                ret
            }
        }
    }
}

#[cfg(test)]
mod not_neg_exhaustive_tests {
    use super::*;
    use crate::simple_lcg::SimpleLCG;
    use tracing::{info, debug, trace};

    // a helper to produce random 64-bit baseuint
    fn random_u64_baseuint(rng: &mut SimpleLCG) -> BaseUInt64 {
        let r = rng.next_u64();
        BaseUInt64::from(r)
    }

    fn all_ones_64() -> BaseUInt64 {
        BaseUInt64::from(u64::MAX)
    }

    fn all_ones_256() -> BaseUInt256 {
        let mut x = BaseUInt256::default();
        for limb in x.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        x
    }

    #[traced_test]
    fn test_not_basic_64() {
        trace!("Testing bitwise NOT for BaseUInt64 on basic edge cases.");

        let zero = BaseUInt64::default();
        let not_zero = !zero.clone();
        assert_eq!(not_zero, all_ones_64(), "!0 => all bits set in 64 bits");

        let ones = all_ones_64();
        let not_ones = !ones.clone();
        assert_eq!(not_ones, zero, "!all_ones => 0 in 64 bits");

        let x = BaseUInt64::from(0x1234_ABCD_0000_4444u64);
        let double_not = !!x.clone();
        assert_eq!(x, double_not);
    }

    #[traced_test]
    fn test_not_random_64() {
        trace!("Testing bitwise NOT for BaseUInt64 on random values.");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF_CAFE_F00D);
        for _ in 0..20 {
            let x = random_u64_baseuint(&mut rng);
            let not_x = !x.clone();
            let expected = x.clone() ^ &all_ones_64();
            assert_eq!(not_x, expected, "Bitwise NOT should match XOR with all bits set.");
        }
    }

    #[traced_test]
    fn test_neg_basic_64() {
        trace!("Testing negation for BaseUInt64 on basic edge cases.");

        let zero = BaseUInt64::default();
        let neg_zero = -zero.clone();
        assert_eq!(neg_zero, zero, "-0 => 0 for 64 bits.");

        let ones = all_ones_64();
        let neg_ones = -ones.clone();
        assert_eq!(neg_ones, BaseUInt64::from(1u64));
    }

    #[traced_test]
    fn test_neg_random_64() {
        trace!("Testing negation for BaseUInt64 on random values.");

        let mut rng = SimpleLCG::new(0x1234_5678_9ABC_DEF0);
        for _ in 0..20 {
            let x = random_u64_baseuint(&mut rng);
            let minus_x = -x.clone();
            let sum = x.clone() + &minus_x;
            assert_eq!(sum, BaseUInt64::default(), "x + (-x) => 0 in 64 bits.");

            let not_x_plus_one = (!x.clone()) + &BaseUInt64::from(1u64);
            assert_eq!(minus_x, not_x_plus_one, "Negation => ~x + 1.");
        }
    }

    #[traced_test]
    fn test_not_neg_random_256() {
        trace!("Testing NOT/NEG for BaseUInt256 on random values.");

        let mut rng = SimpleLCG::new(0x0FF1CE_BADC0FFE_u64);
        for _ in 0..20 {
            let x = super::super::simple_lcg::random_u256(&mut rng);
            let not_x = !x.clone();
            let neg_x = -x.clone();
            let check_neg = (!x.clone()) + &BaseUInt256::from(1u64);
            assert_eq!(neg_x, check_neg, "-x => ~x + 1 in 256 bits.");
            let sum = x.clone() + &neg_x;
            assert_eq!(sum, BaseUInt256::default(), "x + (-x) => 0 in 256 bits.");
            let expected_not = x.clone() ^ &all_ones_256();
            assert_eq!(not_x, expected_not, "!x => x ^ all_ones_256.");
        }
    }
}
