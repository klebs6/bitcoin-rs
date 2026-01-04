// ---------------- [ File: bitcoinsecp256k1-group/src/gej_neg.rs ]
crate::ix!();

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn gej_neg(r: *mut Gej, a: *const Gej) {
    unsafe {
        (*r).infinity = (*a).infinity;
        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!((*r).y), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).z), core::ptr::addr_of_mut!((*r).z), 1);

        let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_normalize_weak(ry);
        fe_negate(ry, ry as *const Fe, 1);
    }
}
