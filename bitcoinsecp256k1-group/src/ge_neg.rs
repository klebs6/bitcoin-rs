// ---------------- [ File: bitcoinsecp256k1-group/src/ge_neg.rs ]
crate::ix!();

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn ge_neg(
    r: *mut Ge,
    a: *const Ge
) {
    unsafe {
        core::ptr::copy(a, r, 1);
        let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_normalize_weak(ry);
        fe_negate(ry, ry as *const Fe, 1);
    }
}
