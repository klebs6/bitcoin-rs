// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_signed62.rs ]
crate::ix!();

/// A signed 62-bit limb representation of integers.
/// 
/// Its value is sum(v[i] * 2^(62*i), i=0..4).
/// 
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModInv64Signed62 {
    pub v: [i64; 5],
}

impl ModInv64Signed62 {
    pub(crate) const fn from_limbs(v: [i64; 5]) -> Self {
        Self { v }
    }

    #[inline]
    pub(crate) fn v(&self) -> &[i64; 5] {
        &self.v
    }

    #[inline]
    pub(crate) fn v_mut(&mut self) -> &mut [i64; 5] {
        &mut self.v
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_signed62_contract {
    use super::*;

    #[traced_test]
    fn signed62_from_limbs_and_accessors_round_trip() {
        let mut x = ModInv64Signed62::from_limbs([1, 2, 3, 4, 5]);
        trace!(initial = ?x.v());
        assert!(*x.v() == [1, 2, 3, 4, 5]);

        {
            let v = x.v_mut();
            v[0] = 7;
            v[4] = -9;
        }

        trace!(mutated = ?x.v());
        assert!(*x.v() == [7, 2, 3, 4, -9]);
    }

    #[traced_test]
    fn signed62_copy_clone_and_mutation_semantics_are_stable() {
        let x = ModInv64Signed62::from_limbs([11, 0, 0, 0, 0]);
        let y = x; /* Copy */
        let z = x.clone();

        trace!(x = ?x.v(), y = ?y.v(), z = ?z.v());
        assert!(*x.v() == *y.v());
        assert!(*x.v() == *z.v());

        let mut w = y;
        w.v_mut()[0] = 99;

        trace!(after = ?w.v(), original = ?x.v());
        assert!(w.v()[0] == 99);
        assert!(x.v()[0] == 11);
    }

    #[traced_test]
    fn signed62_u128_round_trip_for_random_values_up_to_120_bits() {
        let mut seed: u64 = 0x1A2B_3C4D_5566_7788;
        let mask_120: u128 = (1u128 << 120) - 1;

        let mut i: usize = 0;
        while i < 512 {
            let x = splitmix128_next(&mut seed) & mask_120;
            let s = signed62_from_u128(x);

            trace!(iter = i, x = x, limbs = ?s.v());
            assert!(signed62_is_fully_normalized_nonnegative(&s));
            assert!(s.v()[0] >= 0 && (s.v()[0] as u64) <= LIMB_MASK_U64);

            let y = signed62_to_u128_assuming_nonnegative_and_fit(&s);
            assert!(x == y);

            i += 1;
        }
    }

    #[traced_test]
    fn signed62_has_expected_size_and_alignment() {
        let sz = mem::size_of::<ModInv64Signed62>();
        let al = mem::align_of::<ModInv64Signed62>();
        debug!(size = sz, align = al);
        assert!(sz == 5 * mem::size_of::<i64>());
        assert!(al == mem::align_of::<i64>());
    }
}
