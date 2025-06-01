// ---------------- [ File: bitcoin-bigint/src/shl.rs ]
crate::ix!();

// ---------------------------------------------------------------------------
// 7) Macro for Shl / ShlAssign and Shr / ShrAssign
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! define_base_uint_shl_shr {
    ($name:ident, $bits:expr, $limbs:expr) => {

        // Shl<&$name>
        impl core::ops::Shl<&$name> for $name {
            type Output = $name;
            fn shl(self, rhs: &Self) -> Self::Output {
                let shift_raw = rhs.pn[0];
                let shift_bits = shift_raw.min($bits as u32);
                let mut ret = self.clone();
                ret <<= shift_bits;
                ret
            }
        }

        // ShlAssign<u32>
        impl core::ops::ShlAssign<u32> for $name {
            fn shl_assign(&mut self, shift: u32) {
                trace!("ShlAssign<u32>: self <<= {}, BITS={}", shift, $bits);
                if shift as usize >= $bits {
                    for limb in self.pn.iter_mut() {
                        *limb = 0;
                    }
                    return;
                }
                if shift == 0 {
                    return;
                }

                let limb_shift = (shift / 32) as usize;
                let bit_shift  = shift % 32;
                let old = self.clone();

                // zero out self
                for limb in self.pn.iter_mut() {
                    *limb = 0;
                }

                for i in 0..$limbs {
                    let val = old.pn[i];
                    if val == 0 {
                        continue;
                    }
                    let new_i = i + limb_shift;
                    if new_i < $limbs {
                        self.pn[new_i] |= val << bit_shift;
                    }
                    if bit_shift != 0 && (new_i + 1) < $limbs {
                        self.pn[new_i + 1] |= val >> (32 - bit_shift);
                    }
                }

                debug!("ShlAssign complete => self={:?}", self);
            }
        }

        // Shr<&$name>
        impl core::ops::Shr<&$name> for $name {
            type Output = $name;
            fn shr(self, rhs: &Self) -> Self::Output {
                let shift_raw = rhs.pn[0];
                let shift_bits = shift_raw.min($bits as u32);
                let mut ret = self.clone();
                ret >>= shift_bits;
                ret
            }
        }

        // ShrAssign<u32>
        impl core::ops::ShrAssign<u32> for $name {
            fn shr_assign(&mut self, shift: u32) {
                tracing::trace!(
                    "Entering shr_assign<u32>: BITS={}, shift={}, initial self={:X?}",
                    $bits,
                    shift,
                    self.pn
                );
                let num_limbs = $limbs;

                if shift as usize >= $bits {
                    for limb in self.pn.iter_mut() {
                        *limb = 0;
                    }
                    tracing::trace!(
                        "Leaving shr_assign<u32>; shift >= {}, self=0 => {:X?}",
                        $bits,
                        self.pn
                    );
                    return;
                }

                let limb_shift = (shift / 32) as usize;
                let bit_shift = shift % 32;

                if limb_shift > 0 {
                    for i in 0..(num_limbs - limb_shift) {
                        self.pn[i] = self.pn[i + limb_shift];
                    }
                    for i in (num_limbs - limb_shift)..num_limbs {
                        self.pn[i] = 0;
                    }
                    tracing::debug!(
                        "After shifting whole limbs => limb_shift={}, partial self={:X?}",
                        limb_shift,
                        self.pn
                    );
                }

                if bit_shift > 0 {
                    let mut prev = 0u32;
                    for i in (0..num_limbs).rev() {
                        let current = self.pn[i];
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
    }
}

#[cfg(test)]
mod test_ref_based_shl_ops {
    use super::*;
    use crate::simple_lcg::{SimpleLCG, random_u256};
    use tracing::{info, debug};

    #[traced_test]
    fn test_shl_for_64bit_exhaustive() {
        trace!("Beginning exhaustive test of Shl<&BaseUInt<BITS>> on BaseUInt64.");

        let shift_values: [u32; 10] = [0, 1, 31, 32, 33, 63, 64, 65, 100, 999];
        let inputs: [u64; 6] = [
            0, 1, 0xFFFF_FFFF_FFFF_FFFF,
            0x1234_5678_9ABC_DEF0,
            0x0FFF_0000_0000_FFFF,
            0x8000_0000_0000_0001,
        ];

        for &inp in &inputs {
            let bu_inp = BaseUInt64::from(inp);
            for &sh in &shift_values {
                let bu_sh = BaseUInt64::from(sh as u64);
                let result_bu = bu_inp.clone() << &bu_sh;
                let expected = if sh >= 64 {
                    0
                } else {
                    ((inp as u128) << sh) as u64
                };
                let result_u64 = result_bu.low64();
                assert_eq!(result_u64, expected, "Shl<&BaseUInt64> mismatch for input=0x{:X}, shift={}", inp, sh);
            }
        }
        info!("Completed exhaustive test of Shl<&BaseUInt64>.");
    }

    #[traced_test]
    fn test_shl_for_256bit_random() {
        trace!("Beginning random test of Shl<&BaseUInt<BITS>> on BaseUInt256.");
        let mut rng = SimpleLCG::new(0xDEAD_BEEF_1234_5678);

        for _trial in 0..50 {
            let bu_val = random_u256(&mut rng);
            let val_u64 = bu_val.low64();
            let shift = (rng.next_u64() % 320) as u32;
            let bu_shift = BaseUInt256::from(shift as u64);
            let result_bu = bu_val.clone() << &bu_shift;
            let result_is_zero = result_bu.pn.iter().all(|&limb| limb == 0);
            let expect_zero = shift >= 256;
            assert_eq!(expect_zero, result_is_zero, "For shift >= 256, entire result should be zero!");
        }
        info!("Completed random test of Shl<&BaseUInt256>.");
    }
}
