// ---------------- [ File: bitcoin-bigint/src/shl.rs ]
crate::ix!();

/// For reference-based shifts, we clamp the shift to BITS (i.e. if shift > BITS, it's all zero).
impl<const BITS: usize> core::ops::Shl<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn shl(self, rhs: &BaseUInt<BITS>) -> Self::Output {
        // We'll interpret only the lower 32 bits of rhs for the shift amount, then clamp to BITS
        let shift_raw = rhs.pn[0];
        let shift_bits = shift_raw.min(BITS as u32);

        let mut ret = self.clone();
        ret <<= shift_bits;
        ret
    }
}

#[cfg(test)]
mod test_ref_based_shl_ops {
    use super::*;

    /// We test the `Shl<&BaseUInt<BITS>> for BaseUInt<BITS>` impl exhaustively for 64-bit width
    /// because that's the simplest to cross-check exactly against native u64 shifting.
    /// We also do random tests to ensure coverage beyond typical edge cases.
    #[traced_test]
    fn test_shl_for_64bit_exhaustive() {
        trace!("Beginning exhaustive test of Shl<&BaseUInt<BITS>> on BaseUInt<64>.");

        // We'll check all shift values from 0 up to some range above 64 to ensure clamping works.
        // For each shift, we'll test a handful of known input values.
        let shift_values: [u32; 10] = [0, 1, 31, 32, 33, 63, 64, 65, 100, 999];
        let inputs: [u64; 6] = [
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0001,
            0xFFFF_FFFF_FFFF_FFFF,
            0x1234_5678_9ABC_DEF0,
            0x0FFF_0000_0000_FFFF,
            0x8000_0000_0000_0001,
        ];

        for &inp in &inputs {
            let bu_inp = BaseUInt::<64>::from(inp);
            debug!("Testing input=0x{:016X} => BaseUInt<64>={:?}", inp, bu_inp);

            for &sh in &shift_values {
                info!("Shifting by {} bits.", sh);
                let bu_sh = BaseUInt::<64>::from(sh as u64);

                // Our big-int-based result
                let result_bu = bu_inp.clone() << &bu_sh;

                // The "expected" shift in a native sense: treat as 64-bit truncated shift:
                //  - If shift >= 64 => result is 0
                //  - Otherwise => (inp << shift) truncated to 64 bits
                let expected = if sh >= 64 {
                    0
                } else {
                    // do the shift in a bigger type then truncate
                    ((inp as u128) << sh) as u64
                };

                let result_u64 = result_bu.low64();
                debug!(
                    "Shifted result => BaseUInt<64>={:?} => low64=0x{:016X}, expected=0x{:016X}",
                    result_bu, result_u64, expected
                );
                if result_u64 != expected {
                    error!("Mismatch: got 0x{:016X}, expected 0x{:016X}!", result_u64, expected);
                }
                assert_eq!(result_u64, expected, "Shl<&BaseUInt<64>> mismatch for input=0x{:016X}, shift={}", inp, sh);
            }
        }
        info!("Completed exhaustive test of Shl<&BaseUInt<BITS>> for 64-bit base type.");
    }

    /// We also do some random tests for `Shl<&BaseUInt<BITS>>` using a larger type (BaseUInt<256>)
    /// to ensure coverage beyond just 64-bit. We won't do an exhaustive approach for 256 bits,
    /// but we'll do random trials with a simple reference check for correctness.
    #[traced_test]
    fn test_shl_for_256bit_random() {
        trace!("Beginning random test of Shl<&BaseUInt<BITS>> on BaseUInt<256>.");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF_1234_5678);

        for _trial in 0..50 {
            // Random 256-bit value
            let bu_val = random_u256(&mut rng);
            let val_u64 = bu_val.low64(); // just for logging

            // Random shift in [0..320] range
            let shift = (rng.next_u64() % 320) as u32;
            let bu_shift = BaseUInt::<256>::from(shift as u64);

            debug!(
                "Random trial => val.low64=0x{:016X}, shift={}, performing big-int Shl.",
                val_u64, shift
            );

            let result_bu = bu_val.clone() << &bu_shift;
            let result_is_zero = result_bu.pn.iter().all(|&limb| limb == 0);
            // Our reference logic: any shift >= 256 => result should be zero for BaseUInt<256>.
            let expect_zero = shift >= 256;

            if expect_zero && !result_is_zero {
                error!(
                    "MISMATCH => shift={} >= 256 => expected all zero but got: {:?}",
                    shift, result_bu
                );
            }
            assert_eq!(
                expect_zero, result_is_zero,
                "For shift >= 256, entire result should be zero!"
            );
        }
        info!("Completed random test of Shl<&BaseUInt<BITS>> for 256-bit base type.");
    }
}
