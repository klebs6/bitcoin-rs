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
