// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej.rs ]
crate::ix!();

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  | Constant time.
  |
  */
pub fn ge_set_gej(r: *mut Ge, a: *mut Gej) {
    unsafe {
        let mut z2: Fe = core::mem::zeroed();
        let mut z3: Fe = core::mem::zeroed();

        (*r).infinity = (*a).infinity;

        let az: *mut Fe = core::ptr::addr_of_mut!((*a).z);
        fe_inv(az, az as *const Fe);

        fe_sqr(core::ptr::addr_of_mut!(z2), az as *const Fe);
        fe_mul(
            core::ptr::addr_of_mut!(z3),
            az as *const Fe,
            core::ptr::addr_of!(z2),
        );

        let ax: *mut Fe = core::ptr::addr_of_mut!((*a).x);
        let ay: *mut Fe = core::ptr::addr_of_mut!((*a).y);

        fe_mul(ax, ax as *const Fe, core::ptr::addr_of!(z2));
        fe_mul(ay, ay as *const Fe, core::ptr::addr_of!(z3));
        fe_set_int(az, 1);

        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!((*r).y), 1);
    }
}

#[cfg(test)]
mod ge_set_gej_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_set_gej_matches_ge_set_gej_var_for_rescaled_point() {
        tracing::info!("Validating ge_set_gej matches ge_set_gej_var for a rescaled jacobian point.");

        unsafe {
            let mut a1: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a1), core::ptr::addr_of!(ge_const_g));

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(9);
            gej_rescale(core::ptr::addr_of_mut!(a1), core::ptr::addr_of!(s));

            let mut a2: Gej = core::ptr::read(core::ptr::addr_of!(a1));

            let mut r_ct: Ge = core::mem::zeroed();
            ge_set_gej(core::ptr::addr_of_mut!(r_ct), core::ptr::addr_of_mut!(a1));

            let mut r_vt: Ge = core::mem::zeroed();
            ge_set_gej_var(core::ptr::addr_of_mut!(r_vt), core::ptr::addr_of_mut!(a2));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r_ct, &r_vt));
            assert!(ge_is_valid_var(core::ptr::addr_of!(r_ct)) != 0);
        }
    }

    #[traced_test]
    fn ge_set_gej_propagates_infinity_flag() {
        tracing::info!("Validating ge_set_gej preserves infinity flag.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a));

            let mut r: Ge = core::mem::zeroed();
            ge_set_gej(core::ptr::addr_of_mut!(r), core::ptr::addr_of_mut!(a));

            assert!(ge_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }
}
