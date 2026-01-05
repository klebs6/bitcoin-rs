// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_xo_var.rs ]
crate::ix!();

/// Set a group element (affine) equal to the point with the given X coordinate, and given oddness
/// for Y. 
///
/// Return value indicates whether the result is valid.
/// 
pub fn ge_set_xo_var(r: *mut Ge, x: *const Fe, odd: i32) -> i32 {
    unsafe {
        let mut x2: Fe = core::mem::zeroed();
        let mut x3: Fe = core::mem::zeroed();

        core::ptr::copy(x, core::ptr::addr_of_mut!((*r).x), 1);
        fe_sqr(core::ptr::addr_of_mut!(x2), x);
        fe_mul(core::ptr::addr_of_mut!(x3), x, core::ptr::addr_of!(x2));
        (*r).infinity = 0;
        fe_add(core::ptr::addr_of_mut!(x3), core::ptr::addr_of!(fe_const_b));

        if fe_sqrt(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(x3)) == 0 {
            return 0;
        }

        fe_normalize_var(core::ptr::addr_of_mut!((*r).y));
        if fe_is_odd(core::ptr::addr_of!((*r).y)) != odd {
            let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
            fe_negate(ry, ry as *const Fe, 1);
        }
        1
    }
}

#[cfg(test)]
mod ge_set_xo_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn x_only_decompression_roundtrips_generator_x_and_controls_y_parity() {
        tracing::info!("Validating ge_set_xo_var with generator x reproduces generator and its negation based on oddness.");

        unsafe {
            let x_ptr: *const Fe = core::ptr::addr_of!(ge_const_g.x);

            let yodd: i32 = fe_is_odd(core::ptr::addr_of!(ge_const_g.y));
            let yodd_flip: i32 = if yodd != 0 { 0 } else { 1 };

            let mut p_same: Ge = core::mem::zeroed();
            assert!(ge_set_xo_var(core::ptr::addr_of_mut!(p_same), x_ptr, yodd) != 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(p_same)) != 0);
            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&p_same, &ge_const_g));

            let mut p_flip: Ge = core::mem::zeroed();
            assert!(ge_set_xo_var(core::ptr::addr_of_mut!(p_flip), x_ptr, yodd_flip) != 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(p_flip)) != 0);

            let mut neg_g: Ge = core::mem::zeroed();
            ge_neg(core::ptr::addr_of_mut!(neg_g), core::ptr::addr_of!(ge_const_g));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&p_flip, &neg_g));
        }
    }

    #[traced_test]
    fn x_only_decompression_reports_failure_for_non_residue_x() {
        tracing::info!("Searching for an x that does not yield a curve point and validating ge_set_xo_var returns 0.");

        unsafe {
            let mut found: i32 = 0;
            let mut tmp: Ge = core::mem::zeroed();

            let mut i: i32 = 0;
            while i < 4096 && found == 0 {
                let x: Fe = secp256k1_group_exhaustive_test_support::fe_int(i);
                if ge_set_xo_var(core::ptr::addr_of_mut!(tmp), core::ptr::addr_of!(x), 0) == 0 {
                    found = 1;
                }
                i += 1;
            }

            if found == 0 {
                tracing::error!("Failed to find a non-residue x within the search bound.");
            }
            assert!(found != 0);
        }
    }
}

#[cfg(test)]
mod ge_set_xo_var_rs_adversarial_failure_state_tests {
    use super::*;

    #[traced_test]
    fn decompression_failure_does_not_yield_a_valid_curve_point_even_if_return_value_is_ignored() {
        tracing::info!(
            "Ensuring ge_set_xo_var failure (return=0) never leaves r as a valid curve point, for either parity."
        );

        unsafe {
            let mut found: i32 = 0;
            let mut bad_x: Fe = core::mem::zeroed();

            let mut i: i32 = 0;
            while i < 4096 && found == 0 {
                let x: Fe = secp256k1_group_exhaustive_test_support::fe_int(i);
                let mut tmp: Ge = core::mem::zeroed();

                if ge_set_xo_var(
                    core::ptr::addr_of_mut!(tmp),
                    core::ptr::addr_of!(x),
                    0,
                ) == 0
                {
                    bad_x = x;
                    found = 1;
                }
                i += 1;
            }

            if found == 0 {
                tracing::error!("Failed to find an x with no square root in search bound.");
            }
            assert!(found != 0);

            let mut r_even: Ge = core::mem::zeroed();
            let ok_even: i32 = ge_set_xo_var(
                core::ptr::addr_of_mut!(r_even),
                core::ptr::addr_of!(bad_x),
                0,
            );
            assert!(ok_even == 0);
            assert!(ge_is_infinity(core::ptr::addr_of!(r_even)) == 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(r_even)) == 0);

            let mut r_odd: Ge = core::mem::zeroed();
            let ok_odd: i32 = ge_set_xo_var(
                core::ptr::addr_of_mut!(r_odd),
                core::ptr::addr_of!(bad_x),
                1,
            );
            assert!(ok_odd == 0);
            assert!(ge_is_infinity(core::ptr::addr_of!(r_odd)) == 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(r_odd)) == 0);
        }
    }
}
