// ---------------- [ File: bitcoin-bigint/src/shl_assign.rs ]
crate::ix!();

impl<const BITS: usize> core::ops::ShlAssign<u32> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Shifts `self` left by `shift` bits in a straightforward, robust way, matching the
    /// typical logic used in Bitcoin Core's arith_uint256:
    ///
    /// 1. If `shift >= BITS`, everything shifts out => result = 0.
    /// 2. Otherwise, split into:
    ///    - `limb_shift = shift / 32`  
    ///    - `bit_shift  = shift % 32`
    /// 3. Make a copy of the original `self` (call it `old`). Zero out `self`.
    /// 4. For each limb index i:
    ///    - Let `new_i = i + limb_shift`.
    ///    - If `new_i` is within bounds, shift `old[i]` left by `bit_shift` bits and OR it into `self[new_i]`.
    ///    - If `bit_shift != 0` and `new_i + 1` is within bounds, shift `old[i]` right by `(32 - bit_shift)` and OR it into `self[new_i + 1]`.
    ///
    /// This reliably moves bits upward without overwriting as we go.
    fn shl_assign(&mut self, shift: u32) {
        trace!("ShlAssign<u32>: self <<= {}, BITS={}", shift, BITS);

        // If shifting by >= total bits, everything becomes zero
        if shift as usize >= BITS {
            for limb in self.pn.iter_mut() {
                *limb = 0;
            }
            return;
        }
        if shift == 0 {
            return;
        }

        let limb_shift = (shift / 32) as usize;
        let bit_shift = shift % 32;
        let old = self.clone();

        // zero out self
        for limb in self.pn.iter_mut() {
            *limb = 0;
        }

        let limb_count = BITS / 32;
        for i in 0..limb_count {
            let val = old.pn[i];
            if val == 0 {
                continue;
            }
            // place the main left-shifted portion into new_i
            let new_i = i + limb_shift;
            if new_i < limb_count {
                self.pn[new_i] |= val << bit_shift;
            }
            // if bit_shift != 0, there's an overflow portion that goes to the next higher limb
            if bit_shift != 0 && (new_i + 1) < limb_count {
                self.pn[new_i + 1] |= val >> (32 - bit_shift);
            }
        }

        debug!("ShlAssign complete => self={:?}", self);
    }
}

#[cfg(test)]
mod shift_left_assign_u32_tests {
    use super::*;

    /// Exercises `ShlAssign<u32>` on 64-bit BaseUInt.
    /// We test random values and shifts from 0 up to beyond 64,
    /// verifying correct zeroing when shift >= 64, and correct
    /// left shifts otherwise. We confirm by comparing the low64
    /// bits with a straightforward `u128` reference shift.
    #[traced_test]
    fn exhaustive_64bit_shl_assign() {
        info!("Beginning exhaustive tests for ShlAssign<u32> on BaseUInt<64>.");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF_F00D_u64);

        // We'll test 100 random values, shifting by 0..80 bits
        for _test_i in 0..100 {
            let original_val = rng.next_u64();
            for shift_amount in 0..=80 {
                trace!(
                    "Testing 64-bit shift: original=0x{:X}, shift_amount={}",
                    original_val,
                    shift_amount
                );

                // Build a 64-bit BaseUInt from `original_val`
                let mut x = BaseUInt::<64>::from(original_val);
                x <<= shift_amount;

                // For reference, if shift >= 64 => result is 0, else original_val << shift
                let expected_val = if shift_amount >= 64 {
                    0
                } else {
                    (original_val as u128) << shift_amount
                };

                // Because this is a 64-bit BaseUInt, low64() is the entire value
                let got_val = x.low64() as u128;
                if got_val != (expected_val & 0xFFFF_FFFF_FFFF_FFFF) {
                    error!(
                        "Mismatch! original=0x{:X}, shift={}, got=0x{:X}, expected=0x{:X}",
                        original_val,
                        shift_amount,
                        got_val,
                        expected_val
                    );
                }
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

    /// Exercises `ShlAssign<u32>` on 256-bit BaseUInt.
    /// We again test random values for shifts from 0..=272 (just beyond 256).
    /// We at least verify the low64 bits match a simple reference rule:
    /// - if shift >= 64, low64 is 0
    /// - else, it's (original_low64 << shift)
    ///
    /// This doesn't confirm upper limbs fully, but it ensures
    /// that partial-limb and full-limb shifts behave correctly
    /// for the lower portion.
    #[traced_test]
    fn partial_exhaustive_256bit_shl_assign() {
        info!("Beginning partial-exhaustive tests for ShlAssign<u32> on BaseUInt<256>.");
        let mut rng = SimpleLCG::new(0xBAD_CAFE_1234_5678_u64);

        // We'll test 100 random 256-bit values, shifting by 0..272 bits
        for _test_i in 0..100 {
            // Generate random limbs. We only fill the first two limbs from a random u64
            // for an easy reference to the original lower 64 bits.
            let r64 = rng.next_u64();
            let mut x = BaseUInt::<256>::default();
            x.pn[0] = (r64 & 0xFFFF_FFFF) as u32;
            x.pn[1] = ((r64 >> 32) & 0xFFFF_FFFF) as u32;

            // Keep track of the "original" lower-64 portion for reference
            let original_low64 = x.low64();

            for shift_amount in 0..=272 {
                trace!(
                    "Testing 256-bit shift: original_low64=0x{:X}, shift_amount={}",
                    original_low64,
                    shift_amount
                );

                // Clone x so we can re-shift from the same original each time
                let mut test_val = x.clone();
                test_val <<= shift_amount;

                // For the reference check, we only verify the low64 bits:
                let got = test_val.low64();
                let expected = if shift_amount >= 64 {
                    0
                } else {
                    (original_low64 as u128) << shift_amount
                };

                if got as u128 != (expected & 0xFFFF_FFFF_FFFF_FFFF) {
                    error!(
                        "Mismatch! original_low64=0x{:X}, shift={}, got=0x{:X}, expected=0x{:X}",
                        original_low64,
                        shift_amount,
                        got,
                        expected
                    );
                }
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
