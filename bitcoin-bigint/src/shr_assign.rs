// ---------------- [ File: bitcoin-bigint/src/shr_assign.rs ]
crate::ix!();

impl<const BITS: usize> core::ops::ShrAssign<u32> for BaseUInt<BITS> 
where
    [(); BITS / 32]:,
{
    fn shr_assign(&mut self, shift: u32) {
        tracing::trace!(
            "Entering shr_assign<u32>: BITS={}, shift={}, initial self={:X?}",
            BITS,
            shift,
            self.pn
        );

        let num_limbs = BITS / 32;

        // If shift >= BITS, the entire number becomes zero:
        if shift as usize >= BITS {
            for limb in self.pn.iter_mut() {
                *limb = 0;
            }
            tracing::trace!(
                "Leaving shr_assign<u32>; shift >= {}, so self=0 => {:X?}",
                BITS,
                self.pn
            );
            return;
        }

        // Shift in units of limbs first:
        let limb_shift = (shift / 32) as usize;
        let bit_shift = shift % 32;

        if limb_shift > 0 {
            // Move each limb downward by limb_shift
            for i in 0..(num_limbs - limb_shift) {
                self.pn[i] = self.pn[i + limb_shift];
            }
            // Fill the top limbs with zero:
            for i in (num_limbs - limb_shift)..num_limbs {
                self.pn[i] = 0;
            }

            tracing::debug!(
                "After shifting whole limbs => limb_shift={}, partial self={:X?}",
                limb_shift,
                self.pn
            );
        }

        // Now handle any bit-level shifting (0..31)
        if bit_shift > 0 {
            let mut prev = 0u32;
            // Move from high index down to 0
            for i in (0..num_limbs).rev() {
                let current = self.pn[i];
                // shift current right, then bring in bits from 'prev'
                self.pn[i] = (current >> bit_shift) | (prev << (32 - bit_shift));
                prev = current;
            }

            tracing::debug!(
                "After shifting bits => bit_shift={}, final self={:X?}",
                bit_shift,
                self.pn
            );
        }

        tracing::trace!("Leaving shr_assign<u32>; final self={:X?}", self.pn);
    }
}

#[cfg(test)]
mod shr_assign_u32_64bit_exhaustive {
    use super::*;

    /// We will test `ShrAssign<u32>` on a 64-bit `BaseUInt<64>`.
    /// This lets us directly compare the result against native `u64` shifts.
    /// Because `BaseUInt<64>` can hold up to 64 bits, a normal Rust `u64`
    /// is sufficient as our "reference type" for correctness checks.
    ///
    /// We also do edge-case tests (shifting by 0, shifting by >=64) and random tests.
    /// We rely on `BaseUInt::from(u64)` to create our test values and then
    /// compare the results of `>>= shift` with the standard Rust `u64 >> shift`.
    ///
    /// We produce detailed logs at each step to facilitate debugging.
    #[traced_test]
    fn shr_assign_u32_correctness_against_native_u64() {
        info!("Begin exhaustive correctness checks of `BaseUInt<64> >>= u32` versus native u64.");

        // Check edge cases systematically.
        // We'll test all shift values from 0..=70
        // (anything >=64 in `BaseUInt<64>` should yield 0).
        for shift in 0..=70u32 {
            // Test a variety of special input values:
            let test_inputs: [u64; 5] = [
                0,
                1,
                0xFFFFFFFFFFFFFFFF,
                0x00000000FFFFFFFF,
                0xFFFFFFFF00000000,
            ];

            for &val_u64 in &test_inputs {
                let mut bigval = BaseUInt::<64>::from(val_u64);

                debug!("Before shift => val_u64 = 0x{:016X}, shift={}", val_u64, shift);
                debug!("BaseUInt<64> = {:?}", bigval);

                // Do the shift in our `BaseUInt`.
                bigval >>= shift;

                // Compute the reference result in native u64.
                let expected = if shift >= 64 {
                    0
                } else {
                    val_u64 >> shift
                };

                debug!("After shift => bigval = {:?}, expected = 0x{:016X}", bigval, expected);

                // Now extract `bigval` back to a normal u64 via `low64()`.
                let got_u64 = bigval.low64();

                assert_eq!(
                    got_u64, expected,
                    "ShrAssign failed for val=0x{:016X}, shift={}",
                    val_u64, shift
                );
            }
        }

        info!("Finished systematic edge-case checks for `ShrAssign<u32>` on BaseUInt<64>.");
    }

    /// A light random test that verifies shifting random values by random amounts.
    /// We do 500 iterations with a simple LCG-based RNG. Each iteration:
    ///  1) pick random `val_u64`, create `BaseUInt<64>` from it
    ///  2) pick random shift in [0..128]
    ///  3) do bigval >>= shift
    ///  4) compare to native val_u64 >> shift (clamped to 0 if shift >=64)
    #[traced_test]
    fn shr_assign_u32_random_fuzz_64bit() {
        info!("Begin random-fuzz checks of `BaseUInt<64> >>= u32`.");
        let mut rng = crate::simple_lcg::SimpleLCG::new(0xACE0_1234_5678_90AB);

        for _ in 0..500 {
            let val_u64 = rng.next_u64();
            let mut bigval = BaseUInt::<64>::from(val_u64);

            // Choose a shift up to 128 to test out-of-range paths too.
            let shift = (rng.next_u64() % 129) as u32;

            debug!("Fuzz iteration: val_u64=0x{:016X}, shift={}", val_u64, shift);

            let mut expected = 0u64;
            if shift < 64 {
                expected = val_u64 >> shift;
            }

            bigval >>= shift;
            let got_u64 = bigval.low64();

            debug!("Got => 0x{:016X}, expected => 0x{:016X}", got_u64, expected);

            assert_eq!(
                got_u64, expected,
                "Random fuzz shift mismatch: input=0x{:016X}, shift={}",
                val_u64, shift
            );
        }

        info!("Random-fuzz checks succeeded for `ShrAssign<u32>` on BaseUInt<64>.");
    }

    /// Specifically test the "all zero" result triggered by large shift >= 64.
    /// We pass shifts in [64, 65, 66, 100, 999].
    /// Regardless of the original value, the result must become 0.
    #[traced_test]
    fn shr_assign_u32_large_shift_clears_baseuint64() {
        info!("Testing large shifts that should clear BaseUInt<64> completely.");

        let test_values: [u64; 4] = [
            0,
            1,
            0xFFFFFFFFFFFFFFFF,
            0xABCDEFFF12345678,
        ];

        let large_shifts: [u32; 5] = [64, 65, 66, 100, 999];

        for &val_u64 in &test_values {
            for &shift in &large_shifts {
                let mut bigval = BaseUInt::<64>::from(val_u64);
                debug!("Val=0x{:016X}, shift={}", val_u64, shift);
                bigval >>= shift;
                let got_u64 = bigval.low64();
                debug!("After shift => 0x{:016X}", got_u64);
                assert_eq!(
                    got_u64, 0,
                    "Expected a zero result for shift={} >= 64 on val=0x{:016X}",
                    shift, val_u64
                );
            }
        }

        info!("Confirmed that large shifts zero out BaseUInt<64> as expected.");
    }

    /// Verify that shifting by 0 bits is a no-op. We do a range of values,
    /// confirm they remain unchanged after `>>= 0`.
    #[traced_test]
    fn shr_assign_u32_no_op_for_zero_bits() {
        info!("Testing that `ShrAssign<u32>` by 0 bits does nothing.");

        let test_values: [u64; 5] = [
            0,
            1,
            0xFFFFFFFFFFFFFFFF,
            0x123456789ABCDEF0,
            0x00FF00FF00FF00FF,
        ];

        for &val_u64 in &test_values {
            let mut bigval = BaseUInt::<64>::from(val_u64);
            debug!("Pre => val_u64=0x{:016X}, bigval={:?}", val_u64, bigval);
            bigval >>= 0;
            let got_u64 = bigval.low64();
            debug!("Post => got=0x{:016X}, expected=0x{:016X}", got_u64, val_u64);
            assert_eq!(got_u64, val_u64, "Shift by 0 should not alter the value!");
        }

        info!("Confirmed `ShrAssign<u32>(0)` leaves BaseUInt<64> unchanged.");
    }
}
