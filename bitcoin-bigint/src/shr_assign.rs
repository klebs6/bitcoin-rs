crate::ix!();

impl<const BITS: usize> core::ops::ShrAssign<u32> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Shifts `self` right by `shift` bits in a similarly robust manner:
    ///
    /// 1. If `shift >= BITS`, result = 0.
    /// 2. Else, `limb_shift = shift/32`, `bit_shift = shift%32`.
    /// 3. Copy `self` -> `old`, zero out `self`.
    /// 4. For each limb i in old, figure out new_i = i - limb_shift (if >= 0).
    ///    - The main portion is `old[i] >> bit_shift` => goes into `self[new_i]`.
    ///    - If bit_shift != 0, the leftover bits shift into `self[new_i-1]`, specifically `old[i] << (32 - bit_shift)`.
    ///
    /// The net effect is a correct “downward” bit shift, with partial bits crossing limb boundaries.
    fn shr_assign(&mut self, shift: u32) {

        trace!("ShrAssign<u32>: self >>= {}, BITS={}", shift, BITS);

        // If shift >= total bits, everything is lost => 0
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
            // target index is i - limb_shift
            if i >= limb_shift {
                let new_i = i - limb_shift;
                // main portion => old[i] >> bit_shift
                self.pn[new_i] |= val >> bit_shift;
                // leftover bits => go into new_i-1
                if bit_shift != 0 && new_i > 0 {
                    self.pn[new_i - 1] |= val << (32 - bit_shift);
                }
            }
        }

        debug!("ShrAssign complete => self={:?}", self);
    }
}
