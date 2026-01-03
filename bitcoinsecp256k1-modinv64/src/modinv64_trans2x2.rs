// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_trans2x2.rs ]
crate::ix!();

/// Data type for transition matrices (see section
/// 3 of explanation).
/// 
/// t = [ u  v ]
///     [ q  r ]
///
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModInv64Trans2x2 {
    u: i64,
    v: i64,
    q: i64,
    r: i64,
}

impl ModInv64Trans2x2 {
    #[inline]
    pub(crate) fn u(&self) -> i64 {
        self.u
    }

    #[inline]
    pub(crate) fn v(&self) -> i64 {
        self.v
    }

    #[inline]
    pub(crate) fn q(&self) -> i64 {
        self.q
    }

    #[inline]
    pub(crate) fn r(&self) -> i64 {
        self.r
    }

    #[inline]
    pub(crate) fn set(&mut self, u: i64, v: i64, q: i64, r: i64) {
        self.u = u;
        self.v = v;
        self.q = q;
        self.r = r;
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_trans2x2_contract {
    use super::*;

    #[traced_test]
    fn trans2x2_set_and_get_round_trip() {
        let mut t = ModInv64Trans2x2 { u: 0, v: 0, q: 0, r: 0 };
        t.set(-7, 11, 13, -17);

        trace!(u = t.u(), v = t.v(), q = t.q(), r = t.r());
        assert!(t.u() == -7);
        assert!(t.v() == 11);
        assert!(t.q() == 13);
        assert!(t.r() == -17);
    }

    #[traced_test]
    fn trans2x2_copy_clone_semantics_are_stable() {
        let mut t = ModInv64Trans2x2 { u: 1, v: 2, q: 3, r: 4 };
        let a = t; /* Copy */
        let b = t.clone();

        trace!(
            a_u = a.u(),
            a_v = a.v(),
            a_q = a.q(),
            a_r = a.r(),
            b_u = b.u(),
            b_v = b.v(),
            b_q = b.q(),
            b_r = b.r()
        );

        assert!(a.u() == 1 && a.v() == 2 && a.q() == 3 && a.r() == 4);
        assert!(b.u() == 1 && b.v() == 2 && b.q() == 3 && b.r() == 4);

        t.set(9, 8, 7, 6);

        trace!(
            t_u = t.u(),
            t_v = t.v(),
            t_q = t.q(),
            t_r = t.r(),
            a_u = a.u(),
            a_v = a.v(),
            a_q = a.q(),
            a_r = a.r()
        );

        assert!(t.u() == 9 && t.v() == 8 && t.q() == 7 && t.r() == 6);
        assert!(a.u() == 1 && a.v() == 2 && a.q() == 3 && a.r() == 4);
    }

    #[traced_test]
    fn trans2x2_has_expected_size_and_alignment() {
        let sz = mem::size_of::<ModInv64Trans2x2>();
        let al = mem::align_of::<ModInv64Trans2x2>();
        debug!(size = sz, align = al);
        assert!(sz == 4 * mem::size_of::<i64>());
        assert!(al == mem::align_of::<i64>());
    }
}
