// ---------------- [ File: bitcoin-bigint/src/shr_assign.rs ]
crate::ix!();

#[cfg(test)]
mod shr_assign_u32_64bit_exhaustive {
    use super::*;
    use tracing::{info, debug};

    #[traced_test]
    fn shr_assign_u32_correctness_against_native_u64() {
        info!("Begin correctness checks of `BaseUInt64 >>= u32` vs native u64.");

        for shift in 0..=70u32 {
            let test_inputs: [u64; 5] = [
                0,
                1,
                0xFFFFFFFFFFFFFFFF,
                0x00000000FFFFFFFF,
                0xFFFFFFFF00000000,
            ];

            for &val_u64 in &test_inputs {
                let mut bigval = BaseUInt64::from(val_u64);
                bigval >>= shift;
                let expected = if shift >= 64 { 0 } else { val_u64 >> shift };
                let got_u64 = bigval.low64();
                assert_eq!(
                    got_u64, expected,
                    "ShrAssign failed for val=0x{:016X}, shift={}",
                    val_u64, shift
                );
            }
        }

        info!("Finished systematic edge-case checks for `ShrAssign<u32>` on BaseUInt64.");
    }

    #[traced_test]
    fn shr_assign_u32_random_fuzz_64bit() {
        info!("Begin random-fuzz checks of `BaseUInt64 >>= u32`.");
        let mut rng = super::super::simple_lcg::SimpleLCG::new(0xACE0_1234_5678_90AB);

        for _ in 0..500 {
            let val_u64 = rng.next_u64();
            let mut bigval = BaseUInt64::from(val_u64);
            let shift = (rng.next_u64() % 129) as u32;

            let expected = if shift < 64 {
                val_u64 >> shift
            } else {
                0
            };

            bigval >>= shift;
            let got_u64 = bigval.low64();
            assert_eq!(got_u64, expected, "Random fuzz shift mismatch for shift={}", shift);
        }

        info!("Random-fuzz checks succeeded for `ShrAssign<u32>` on BaseUInt64.");
    }

    #[traced_test]
    fn shr_assign_u32_large_shift_clears_baseuint64() {
        info!("Testing large shifts that should clear BaseUInt64 completely.");

        let test_values: [u64; 4] = [
            0,
            1,
            0xFFFFFFFFFFFFFFFF,
            0xABCDEFFF12345678,
        ];
        let large_shifts: [u32; 5] = [64, 65, 66, 100, 999];

        for &val_u64 in &test_values {
            for &shift in &large_shifts {
                let mut bigval = BaseUInt64::from(val_u64);
                bigval >>= shift;
                assert_eq!(
                    bigval.low64(), 
                    0,
                    "Expected zero result for shift={} >=64, val=0x{:016X}",
                    shift, val_u64
                );
            }
        }

        info!("Confirmed large shifts zero out BaseUInt64 as expected.");
    }

    #[traced_test]
    fn shr_assign_u32_no_op_for_zero_bits() {
        info!("Testing that `ShrAssign<u32>(0)` does nothing.");

        let test_values: [u64; 5] = [
            0,
            1,
            0xFFFFFFFFFFFFFFFF,
            0x123456789ABCDEF0,
            0x00FF00FF00FF00FF,
        ];

        for &val_u64 in &test_values {
            let mut bigval = BaseUInt64::from(val_u64);
            bigval >>= 0;
            assert_eq!(bigval.low64(), val_u64);
        }

        info!("Confirmed `ShrAssign<u32>(0)` leaves BaseUInt64 unchanged.");
    }
}
