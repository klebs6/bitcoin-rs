// ---------------- [ File: bitcoinsecp256k1-group/src/gej_set_infinity.rs ]
crate::ix!();

/**
  | Set a group element (jacobian) equal
  | to the point at infinity.
  |
  */
pub fn gej_set_infinity(r: *mut Gej) {
    unsafe {
        (*r).infinity = 1;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
        fe_clear(core::ptr::addr_of_mut!((*r).z));
    }
}
