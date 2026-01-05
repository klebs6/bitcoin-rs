// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej_var.rs ]
crate::ix!();

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  |
  */
pub fn ge_set_gej_var(r: *mut Ge, a: *mut Gej) {
    unsafe {
        let mut z2: Fe = core::mem::zeroed();
        let mut z3: Fe = core::mem::zeroed();

        if (*a).infinity != 0 {
            ge_set_infinity(r);
            return;
        }

        let az: *mut Fe = core::ptr::addr_of_mut!((*a).z);
        fe_inv_var(az, az as *const Fe);
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

        ge_set_xy(r, ax as *const Fe, ay as *const Fe);
    }
}

#[cfg(test)]
mod ge_set_gej_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_set_gej_var_matches_ge_set_gej_zinv_with_manual_inverse() {
        tracing::info!("Validating ge_set_gej_var matches ge_set_gej_zinv when given z^{{-1}}.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(7);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(s));

            let mut zinv: Fe = core::mem::zeroed();
            fe_inv_var(core::ptr::addr_of_mut!(zinv), core::ptr::addr_of!(a.z));

            let mut expected: Ge = core::mem::zeroed();
            ge_set_gej_zinv(
                core::ptr::addr_of_mut!(expected),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(zinv),
            );

            let mut a_copy: Gej = core::ptr::read(core::ptr::addr_of!(a));
            let mut got: Ge = core::mem::zeroed();
            ge_set_gej_var(core::ptr::addr_of_mut!(got), core::ptr::addr_of_mut!(a_copy));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&got, &expected));
            assert!(ge_is_valid_var(core::ptr::addr_of!(got)) != 0);
        }
    }

    #[traced_test]
    fn ge_set_gej_var_short_circuits_infinity_to_ge_infinity() {
        tracing::info!("Validating ge_set_gej_var maps jacobian infinity to affine infinity.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a));

            let mut r: Ge = core::mem::zeroed();
            ge_set_gej_var(core::ptr::addr_of_mut!(r), core::ptr::addr_of_mut!(a));

            assert!(ge_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }
}
