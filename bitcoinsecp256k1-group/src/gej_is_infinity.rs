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

#[cfg(test)]
mod gej_is_infinity_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_is_infinity_tracks_state_transitions() {
        tracing::info!("Validating gej_is_infinity flag transitions.");

        unsafe {
            let mut p: Gej = Gej::new();
            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);

            gej_set_infinity(core::ptr::addr_of_mut!(p));
            assert!(gej_is_infinity(core::ptr::addr_of!(p)) != 0);

            gej_clear(core::ptr::addr_of_mut!(p));
            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);
        }
    }
}
