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

#[cfg(test)]
mod ge_is_infinity_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_is_infinity_tracks_state_transitions() {
        tracing::info!("Validating ge_is_infinity reports correct flag transitions.");

        unsafe {
            let mut p: Ge = core::mem::zeroed();
            p = Ge::new();
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);

            ge_set_infinity(core::ptr::addr_of_mut!(p));
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) != 0);

            ge_clear(core::ptr::addr_of_mut!(p));
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
        }
    }
}
