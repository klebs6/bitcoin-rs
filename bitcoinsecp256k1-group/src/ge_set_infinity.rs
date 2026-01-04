// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_infinity.rs ]
crate::ix!();

/**
  | Set a group element (affine) equal to
  | the point at infinity.
  |
  */
pub fn ge_set_infinity(r: *mut Ge) {
    unsafe {
        (*r).infinity = 1;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
    }
}
