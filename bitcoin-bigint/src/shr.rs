// ---------------- [ File: bitcoin-bigint/src/shr.rs ]
crate::ix!();

#[cfg(test)]
mod test_shr_for_baseuint {
    use super::*;
    use crate::simple_lcg::{SimpleLCG, random_u256};
    use tracing::{info, debug, trace};

    #[traced_test]
    fn test_shr_edge_cases_64() {
        info!("Beginning edge-case testing for Shr with BaseUInt64.");
        let all_ones = BaseUInt64::from(u64::MAX);
        let zero = BaseUInt64::default();

        // SHIFT=0 => same
        let mut shift_val = BaseUInt64::from(0u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result, all_ones);

        // SHIFT=1 => (u64::MAX >> 1)
        shift_val = BaseUInt64::from(1u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result.low64(), u64::MAX >> 1);

        // SHIFT=63 => top bit
        shift_val = BaseUInt64::from(63u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result.low64(), u64::MAX >> 63);

        // SHIFT=64 => zero
        shift_val = BaseUInt64::from(64u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result, zero);

        // SHIFT=65 => zero
        shift_val = BaseUInt64::from(65u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result, zero);

        // SHIFT=9999 => zero
        shift_val = BaseUInt64::from(9999u64);
        let result = all_ones.clone() >> &shift_val;
        assert_eq!(result, zero);
    }

    #[traced_test]
    fn test_shr_edge_cases_256() {
        info!("Beginning edge-case testing for Shr with BaseUInt256.");

        let mut all_ones_256 = BaseUInt256::default();
        for limb in all_ones_256.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }
        let zero_256 = BaseUInt256::default();

        // SHIFT=0
        let mut shift_val = BaseUInt256::from(0u64);
        let result = all_ones_256.clone() >> &shift_val;
        assert_eq!(result, all_ones_256);

        // SHIFT=1
        shift_val = BaseUInt256::from(1u64);
        let result = all_ones_256.clone() >> &shift_val;
        let expected_low64 = 0xFFFF_FFFF_FFFF_FFFFu64;
        assert_eq!(result.low64(), expected_low64);

        // SHIFT=255
        shift_val = BaseUInt256::from(255u64);
        let result = all_ones_256.clone() >> &shift_val;
        assert_ne!(result, zero_256);

        // SHIFT=256 => zero
        shift_val = BaseUInt256::from(256u64);
        let result = all_ones_256.clone() >> &shift_val;
        assert_eq!(result, zero_256);

        // SHIFT=9999 => zero
        shift_val = BaseUInt256::from(9999u64);
        let result = all_ones_256.clone() >> &shift_val;
        assert_eq!(result, zero_256);
    }

    #[traced_test]
    fn test_shr_random_64() {
        info!("Beginning random-amount testing of Shr with BaseUInt64.");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF_1234_5678);

        for i in 0..1000 {
            let val = rng.next_u64();
            let a_64 = BaseUInt64::from(val);

            let shift = rng.next_u64() & 0xFFFF_FFFF;
            let shift_val = BaseUInt64::from(shift);
            let result = a_64 >> &shift_val;
            let shift_bits = shift.min(64) as u32;

            let expected = if shift_bits == 64 { 0 } else { val >> shift_bits };
            assert_eq!(
                result.low64(),
                expected,
                "Mismatch in random Shr test, i={}",
                i
            );
        }
    }

    #[traced_test]
    fn test_shr_random_256() {
        info!("Beginning random-amount testing of Shr with BaseUInt256.");
        let mut rng = SimpleLCG::new(0x1234_5678_ABCD_EF01);

        for i in 0..1000 {
            let input_256 = random_u256(&mut rng);
            let raw_shift = rng.next_u64() as u32;
            let shift_val = {
                let mut tmp = BaseUInt256::default();
                tmp.pn[0] = raw_shift;
                tmp
            };
            let result_256 = input_256.clone() >> &shift_val;
            let shift_bits = raw_shift.min(256);

            if shift_bits >= 256 {
                assert_eq!(result_256, BaseUInt256::default());
                continue;
            }
            let input_low = input_256.low64();
            let expected_low = input_low >> shift_bits;
            assert_eq!(
                result_256.low64(),
                expected_low,
                "Mismatch in random Shr test (BITS=256, shift={}).",
                shift_bits
            );
        }
    }
}
