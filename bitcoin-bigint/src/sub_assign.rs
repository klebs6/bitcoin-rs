// ---------------- [ File: bitcoin-bigint/src/sub_assign.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_baseuint_subassign {
    ($name:ident, $bits:expr, $limbs:expr) => {

        // sub_assign(&BaseUInt)
        impl core::ops::SubAssign<&$name> for $name {
            fn sub_assign(&mut self, rhs: &Self) {
                // self = self + (-rhs)
                *self += &(-rhs.clone());
            }
        }

        // sub_assign(u64)
        impl core::ops::SubAssign<u64> for $name {
            fn sub_assign(&mut self, rhs: u64) {
                let mut b = Self::from(rhs);
                *self += &(-b);
            }
        }
    }
}

#[cfg(test)]
mod test_sub_assign {
    use super::*;
    use crate::simple_lcg::SimpleLCG;
    use tracing::{info, debug, trace};

    #[traced_test]
    fn basic_sub_assign_ref_64() {
        info!("Starting basic_sub_assign_ref_64 tests...");
        let mut a = BaseUInt64::default();
        let b = BaseUInt64::default();
        a -= &b;
        assert_eq!(a.low64(), 0);

        let mut a = BaseUInt64::from(1u64);
        let b = BaseUInt64::from(1u64);
        a -= &b;
        assert_eq!(a.low64(), 0);

        let mut a = BaseUInt64::from(1u64);
        let b = BaseUInt64::from(2u64);
        a -= &b;
        let expected = u64::MAX;
        assert_eq!(a.low64(), expected);

        info!("basic_sub_assign_ref_64 tests passed.");
    }

    #[traced_test]
    fn random_sub_assign_ref_64() {
        info!("Starting random_sub_assign_ref_64 tests...");
        let mut rng = SimpleLCG::new(0xACE0FF1CE);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();
            let mut a = BaseUInt64::from(x_64);
            let b = BaseUInt64::from(y_64);
            a -= &b;
            let expected = x_64.wrapping_sub(y_64);
            assert_eq!(a.low64(), expected);
        }

        info!("random_sub_assign_ref_64 tests passed.");
    }

    #[traced_test]
    fn basic_sub_assign_u64_64() {
        info!("Starting basic_sub_assign_u64_64 tests...");
        let mut a = BaseUInt64::default();
        a -= 0u64;
        assert_eq!(a.low64(), 0);

        let mut a = BaseUInt64::from(5u64);
        a -= 5u64;
        assert_eq!(a.low64(), 0);

        let mut a = BaseUInt64::default();
        a -= 1u64;
        let expected = u64::MAX;
        assert_eq!(a.low64(), expected);

        info!("basic_sub_assign_u64_64 tests passed.");
    }

    #[traced_test]
    fn random_sub_assign_u64_64() {
        info!("Starting random_sub_assign_u64_64 tests...");
        let mut rng = SimpleLCG::new(0xF00D_BEEF);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();
            let mut a = BaseUInt64::from(x_64);
            a -= y_64;
            let expected = x_64.wrapping_sub(y_64);
            assert_eq!(a.low64(), expected);
        }

        info!("random_sub_assign_u64_64 tests passed.");
    }

    #[traced_test]
    fn basic_sub_assign_ref_256() {
        info!("Starting basic_sub_assign_ref_256 tests...");
        let mut a = BaseUInt256::default();
        let b = BaseUInt256::default();
        a -= &b;
        assert_eq!(a.compare_to(&b), 0);

        let mut a = BaseUInt256::from(1u64);
        let b = BaseUInt256::from(1u64);
        a -= &b;
        let zero = BaseUInt256::default();
        assert_eq!(a.compare_to(&zero), 0);

        let mut all_ones = BaseUInt256::default();
        for limb in all_ones.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        let mut a = all_ones.clone();
        let b = BaseUInt256::from(1u64);
        a -= &b;
        let mut expected_all_ones = all_ones.clone();
        expected_all_ones.pn[0] = 0xFFFF_FFFE;
        assert_eq!(a.pn, expected_all_ones.pn);

        info!("basic_sub_assign_ref_256 tests passed.");
    }

    #[traced_test]
    fn random_sub_assign_ref_256() {
        info!("Starting random_sub_assign_ref_256 tests...");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF);

        for _ in 0..100 {
            let x_128 = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
            let y_128 = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);

            let mut a = BaseUInt256::default();
            a.pn[0] = (x_128 & 0xFFFF_FFFF) as u32;
            a.pn[1] = ((x_128 >> 32) & 0xFFFF_FFFF) as u32;
            a.pn[2] = ((x_128 >> 64) & 0xFFFF_FFFF) as u32;
            a.pn[3] = ((x_128 >> 96) & 0xFFFF_FFFF) as u32;

            let mut b = BaseUInt256::default();
            b.pn[0] = (y_128 & 0xFFFF_FFFF) as u32;
            b.pn[1] = ((y_128 >> 32) & 0xFFFF_FFFF) as u32;
            b.pn[2] = ((y_128 >> 64) & 0xFFFF_FFFF) as u32;
            b.pn[3] = ((y_128 >> 96) & 0xFFFF_FFFF) as u32;

            a -= &b;
            let expected_128 = x_128.wrapping_sub(y_128);
            let mut expected = BaseUInt256::default();
            expected.pn[0] = (expected_128 & 0xFFFF_FFFF) as u32;
            expected.pn[1] = ((expected_128 >> 32) & 0xFFFF_FFFF) as u32;
            expected.pn[2] = ((expected_128 >> 64) & 0xFFFF_FFFF) as u32;
            expected.pn[3] = ((expected_128 >> 96) & 0xFFFF_FFFF) as u32;
            assert_eq!(a.pn[0..4], expected.pn[0..4]);
        }

        info!("random_sub_assign_ref_256 tests passed.");
    }

    #[traced_test]
    fn basic_sub_assign_u64_256() {
        info!("Starting basic_sub_assign_u64_256 tests...");
        let mut a = BaseUInt256::default();
        a -= 0u64;
        assert!(a.equal_to(0));

        let mut a = BaseUInt256::from(100u64);
        a -= 100u64;
        assert!(a.equal_to(0));

        let mut a = BaseUInt256::default();
        a -= 1u64;
        let mut all_ones = BaseUInt256::default();
        for limb in all_ones.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        assert_eq!(a.pn, all_ones.pn);

        info!("basic_sub_assign_u64_256 tests passed.");
    }

    #[traced_test]
    fn random_sub_assign_u64_256() {
        info!("Starting random_sub_assign_u64_256 tests...");
        let mut rng = SimpleLCG::new(0xFACE_FEED);

        for _ in 0..100 {
            let x_64 = rng.next_u64();
            let y_64 = rng.next_u64();
            let mut a = BaseUInt256::from(x_64);
            a -= y_64;
            let expected_64 = x_64.wrapping_sub(y_64);
            assert_eq!(a.low64(), expected_64);
        }

        info!("random_sub_assign_u64_256 tests passed.");
    }
}
