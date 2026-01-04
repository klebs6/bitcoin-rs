// ---------------- [ File: bitcoinsecp256k1-group/src/gej_clear.rs ]
crate::ix!();

/**
  | Clear a gej to prevent leaking
  | sensitive information.
  |
  */
pub fn gej_clear(r: *mut Gej) {
    unsafe {
        (*r).infinity = 0;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
        fe_clear(core::ptr::addr_of_mut!((*r).z));
    }
}
