crate::ix!();

/// Similarly for right shifts: read lower 32 bits, clamp, then do the normal shr_assign.
impl<const BITS: usize> core::ops::Shr<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn shr(self, rhs: &BaseUInt<BITS>) -> Self::Output {

        let shift_raw  = rhs.pn[0];
        let shift_bits = shift_raw.min(BITS as u32);

        let mut ret = self.clone();
        ret >>= shift_bits;
        ret
    }
}

#[cfg(test)]
mod test_shr_for_baseuint {
    use super::*;

    /// Verify that shifting a 64-bit BaseUInt right by various edge amounts
    /// behaves correctly. We test 0, 1, 63, 64, 65, and a very large shift.
    #[traced_test]
    fn test_shr_edge_cases_64() {
        info!("Beginning edge-case testing for Shr with BITS=64.");

        let all_ones = BaseUInt::<64>::from(u64::MAX);
        let zero = BaseUInt::<64>::default();

        // SHIFT = 0 => value should remain the same
        let mut shift_val = BaseUInt::<64>::from(0u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 0 bits: expected=all_ones=0x{:X}, got=0x{:X}",
               all_ones.low64(), result.low64());
        assert_eq!(result, all_ones, "Shr by 0 should yield the same value");

        // SHIFT = 1 => high bit is dropped
        shift_val = BaseUInt::<64>::from(1u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 1 bit: expected=0x{:X}, got=0x{:X}",
               (u64::MAX >> 1), result.low64());
        assert_eq!(result.low64(), (u64::MAX >> 1), "Shr by 1 bit mismatch");
        
        // SHIFT = 63 => only the top bit remains after shift
        shift_val = BaseUInt::<64>::from(63u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 63 bits: expected=0x{:X}, got=0x{:X}",
               (u64::MAX >> 63), result.low64());
        assert_eq!(result.low64(), (u64::MAX >> 63), "Shr by 63 bits mismatch");

        // SHIFT = 64 => should yield zero
        shift_val = BaseUInt::<64>::from(64u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 64 bits: expected=0, got=0x{:X}", result.low64());
        assert_eq!(result, zero, "Shr by 64 bits should yield 0");

        // SHIFT = 65 => also zero (since 65 >= 64)
        shift_val = BaseUInt::<64>::from(65u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 65 bits: expected=0, got=0x{:X}", result.low64());
        assert_eq!(result, zero, "Shr by 65 bits should yield 0");

        // SHIFT = a huge number => definitely zero
        shift_val = BaseUInt::<64>::from(9999u64);
        let result = all_ones.clone() >> &shift_val;
        debug!("Shifting 9999 bits: expected=0, got=0x{:X}", result.low64());
        assert_eq!(result, zero, "Shr by 9999 bits should yield 0");
    }

    /// Verify that shifting a 256-bit BaseUInt right by various edge amounts
    /// behaves correctly. For 256 bits, we confirm that shifting by >= 256 yields 0,
    /// and do a few smaller checks too.
    #[traced_test]
    fn test_shr_edge_cases_256() {
        info!("Beginning edge-case testing for Shr with BITS=256.");

        // We'll set all limbs to 0xFFFF_FFFF for an "all ones" pattern in 256 bits.
        let mut all_ones_256 = BaseUInt::<256>::default();
        for limb in all_ones_256.pn.iter_mut() {
            *limb = 0xFFFF_FFFF;
        }

        let zero_256 = BaseUInt::<256>::default();

        // SHIFT = 0 => same value
        let mut shift_val = BaseUInt::<256>::from(0u64);
        let result = all_ones_256.clone() >> &shift_val;
        debug!("Shifting 0 bits => expected=all_ones, got={:?}", result);
        assert_eq!(result, all_ones_256, "Shr by 0 should yield same value");

        // SHIFT = 1 => half of 'all_ones_256'
        // We'll just check that at least the low64 is half and the shift doesn't produce zero.
        shift_val = BaseUInt::<256>::from(1u64);
        let result = all_ones_256.clone() >> &shift_val;
        let expect_low64 = all_ones_256.low64() >> 1;
        debug!("Shifting 1 bit => expected low64=0x{:X}, got=0x{:X}",
               expect_low64, result.low64());
        assert_eq!(result.low64(), expect_low64, "Shr by 1 mismatch at low64");
        assert_ne!(result, zero_256, "Shr by 1 should not be zero");

        // SHIFT = 255 => only 1 bit remains if all_ones_256 was truly full
        shift_val = BaseUInt::<256>::from(255u64);
        let result = all_ones_256.clone() >> &shift_val;
        debug!("Shifting 255 bits => got={:?}", result);
        // The top bit of an all-ones 256 is effectively 0x1 after 255 shifts, but let's just check != 0.
        assert_ne!(result, zero_256, "Shr by 255 should not be 0");

        // SHIFT = 256 => yields zero
        shift_val = BaseUInt::<256>::from(256u64);
        let result = all_ones_256.clone() >> &shift_val;
        debug!("Shifting 256 bits => got={:?}", result);
        assert_eq!(result, zero_256, "Shr by 256 bits should yield 0");

        // SHIFT = 9999 => definitely 0
        shift_val = BaseUInt::<256>::from(9999u64);
        let result = all_ones_256.clone() >> &shift_val;
        debug!("Shifting 9999 bits => got={:?}", result);
        assert_eq!(result, zero_256, "Shr by large number should yield 0");
    }

    /// Test random values for BITS=64, shifting by random amounts.
    /// We compare the result's low64 bits with a plain u64 right shift
    /// (capped at 64). This is valid since 64 bits can't exceed u64.
    #[traced_test]
    fn test_shr_random_64() {
        info!("Beginning random-amount testing of Shr for BITS=64.");

        let mut rng = SimpleLCG::new(0xDEAD_BEEF_1234_5678);

        for i in 0..1000 {
            // Generate random input
            let val = rng.next_u64();
            let a_64 = BaseUInt::<64>::from(val);

            // Generate random shift in the lower 32 bits
            let shift = (rng.next_u64() & 0xFFFF_FFFF);
            let shift_val = BaseUInt::<64>::from(shift);

            let result = a_64 >> &shift_val;
            let shift_bits = (shift & 0xFFFF_FFFF).min(64) as u32;

            let expected = val >> shift_bits;
            debug!("Test #{} => val=0x{:X}, shift_bits={}, result.low64()=0x{:X}, expected=0x{:X}",
                   i, val, shift_bits, result.low64(), expected);

            assert_eq!(
                result.low64(),
                expected,
                "Mismatch in random Shr test (BITS=64)."
            );
        }
    }

    /// Test random values for BITS=256, shifting by random amounts.
    /// For shifts >= 256, result must be 0. For shifts < 256, we do a partial
    /// check on the low64 bits, since that portion can be compared to a naive
    /// shift of the same bits in 64-bit space. This doesn't fully validate upper bits,
    /// but it does confirm correct behavior for smaller shifts and total zero for large shifts.
    #[traced_test]
    fn test_shr_random_256() {
        info!("Beginning random-amount testing of Shr for BITS=256.");

        let mut rng = SimpleLCG::new(0x1234_5678_ABCD_EF01);

        for i in 0..1000 {
            let input_256 = random_u256(&mut rng);

            // We'll pick the shift by pulling a random u64 and taking its lower 32 bits.
            let raw_shift = rng.next_u64() as u32;
            let shift_val = {
                let mut tmp = BaseUInt::<256>::default();
                tmp.pn[0] = raw_shift;
                tmp
            };

            let result_256 = input_256.clone() >> &shift_val;
            let shift_bits = raw_shift.min(256);

            // If shift_bits >= 256 => expect 0
            if shift_bits >= 256 {
                debug!("Test #{} => shift_bits={}, expecting 0 result", i, shift_bits);
                assert_eq!(result_256, BaseUInt::<256>::default(),
                           "Shr by {} bits (>=256) should yield 0", shift_bits);
                continue;
            }

            // For shift_bits < 256, let's do a partial check on the lower 64 bits:
            // We'll interpret input_256's lower 64 bits in a standard u64,
            // right shift that, and ensure the result matches result_256.low64().
            // This doesn't confirm upper bits but is a good partial check.
            let input_low = input_256.low64();
            let expected_low = input_low >> shift_bits;
            debug!("Test #{} => shift_bits={}, input_low=0x{:X}, expected_low=0x{:X}, actual_low=0x{:X}",
                   i, shift_bits, input_low, expected_low, result_256.low64());
            assert_eq!(
                result_256.low64(),
                expected_low,
                "Mismatch in random Shr test (BITS=256, shift={}).",
                shift_bits
            );
        }
    }
}
