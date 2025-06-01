// ---------------- [ File: bitcoin-bigint/src/shl_assign.rs ]
crate::ix!();

#[cfg(test)]
mod shift_left_assign_u32_tests {
    use super::*;
    use tracing::{info, trace, error, debug};

    #[traced_test]
    fn exhaustive_64bit_shl_assign() {
        info!("Beginning exhaustive tests for ShlAssign<u32> on BaseUInt64.");
        let mut rng = super::super::simple_lcg::SimpleLCG::new(0xDEAD_BEEF_F00D_u64);

        for _test_i in 0..100 {
            let original_val = rng.next_u64();
            for shift_amount in 0..=80 {
                let mut x = BaseUInt64::from(original_val);
                x <<= shift_amount;
                let expected_val = if shift_amount >= 64 {
                    0
                } else {
                    (original_val as u128) << shift_amount
                };
                let got_val = x.low64() as u128;
                assert_eq!(
                    got_val,
                    expected_val & 0xFFFF_FFFF_FFFF_FFFF,
                    "64-bit ShlAssign failed for original=0x{:X}, shift={}",
                    original_val,
                    shift_amount
                );
            }
        }
        info!("All exhaustive 64-bit ShlAssign<u32> tests passed.");
    }

    #[traced_test]
    fn partial_exhaustive_256bit_shl_assign() {
        info!("Beginning partial-exhaustive tests for ShlAssign<u32> on BaseUInt256.");
        let mut rng = super::super::simple_lcg::SimpleLCG::new(0xBAD_CAFE_1234_5678_u64);

        for _test_i in 0..100 {
            let r64 = rng.next_u64();
            let mut x = BaseUInt256::default();
            x.pn[0] = (r64 & 0xFFFF_FFFF) as u32;
            x.pn[1] = ((r64 >> 32) & 0xFFFF_FFFF) as u32;
            let original_low64 = x.low64();

            for shift_amount in 0..=272 {
                let mut test_val = x.clone();
                test_val <<= shift_amount;
                let got = test_val.low64();
                let expected = if shift_amount >= 64 {
                    0
                } else {
                    (original_low64 as u128) << shift_amount
                };
                assert_eq!(
                    got as u128,
                    expected & 0xFFFF_FFFF_FFFF_FFFF,
                    "256-bit ShlAssign failed for shift={}",
                    shift_amount
                );
            }
        }
        info!("All partial-exhaustive 256-bit ShlAssign<u32> tests passed.");
    }
}
