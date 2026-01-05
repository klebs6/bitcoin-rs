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

#[cfg(test)]
mod gej_clear_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_clear_zeroizes_xyz_and_resets_infinity_flag() {
        tracing::info!("Validating gej_clear clears x/y/z and sets infinity=0.");

        unsafe {
            let mut p: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(p), core::ptr::addr_of!(ge_const_g));

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(17);
            gej_rescale(core::ptr::addr_of_mut!(p), core::ptr::addr_of!(s));

            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);

            gej_clear(core::ptr::addr_of_mut!(p));

            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.z)) != 0);
        }
    }

    #[traced_test]
    fn gej_clear_overwrites_infinity_to_non_infinity_zero_point() {
        tracing::info!("Validating gej_clear on infinity resets infinity flag to 0 and clears xyz.");

        unsafe {
            let mut p: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(p));
            assert!(gej_is_infinity(core::ptr::addr_of!(p)) != 0);

            gej_clear(core::ptr::addr_of_mut!(p));

            assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.z)) != 0);
        }
    }
}
