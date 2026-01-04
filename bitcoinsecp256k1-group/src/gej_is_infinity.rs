// ---------------- [ File: bitcoinsecp256k1-group/src/gej_is_infinity.rs ]
crate::ix!();

/**
  | Check whether a group element is the
  | point at infinity.
  |
  */
pub fn gej_is_infinity(a: *const Gej) -> i32 {
    unsafe { (*a).infinity }
}
