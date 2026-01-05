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

#[cfg(test)]
mod gej_set_infinity_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_set_infinity_sets_flag_and_clears_xyz() {
        tracing::info!("Validating gej_set_infinity sets infinity=1 and clears x/y/z.");

        unsafe {
            let mut p: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(p), core::ptr::addr_of!(ge_const_g));
            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);

            gej_set_infinity(core::ptr::addr_of_mut!(p));

            assert!(gej_is_infinity(core::ptr::addr_of!(p)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.z)) != 0);
        }
    }
}
