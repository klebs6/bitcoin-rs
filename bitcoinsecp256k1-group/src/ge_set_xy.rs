// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_xy.rs ]
crate::ix!();

/**
  | Set a group element equal to the point
  | with given X and Y coordinates
  |
  */
pub fn ge_set_xy(
    r: *mut Ge,
    x: *const Fe,
    y: *const Fe
) {
    unsafe {
        (*r).infinity = 0;
        core::ptr::copy(x, core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(y, core::ptr::addr_of_mut!((*r).y), 1);
    }
}
