// ---------------- [ File: bitcoinsecp256k1-group/src/ge_clear.rs ]
crate::ix!();

/**
  | Clear a ge to prevent leaking
  | sensitive information.
  |
  */
pub fn ge_clear(r: *mut Ge) {
    unsafe {
        (*r).infinity = 0;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
    }
}
