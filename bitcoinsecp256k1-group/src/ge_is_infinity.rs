// ---------------- [ File: bitcoinsecp256k1-group/src/ge_is_infinity.rs ]
crate::ix!();

/**
  | Check whether a group element is the
  | point at infinity.
  |
  */
pub fn ge_is_infinity(a: *const Ge) -> i32 {
    unsafe { (*a).infinity }
}
