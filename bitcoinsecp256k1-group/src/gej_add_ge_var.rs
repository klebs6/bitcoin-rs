// ---------------- [ File: bitcoinsecp256k1-group/src/gej_add_ge_var.rs ]
crate::ix!();

/// Set r equal to the sum of a and b (with b given in affine coordinates). 
///
/// This is more efficient than gej_add_var. It is identical to gej_add_ge but without
/// constant-time guarantee, and b is allowed to be infinity. 
///
/// If rzr is non-NULL this sets *rzr such that r->z == a->z * *rzr (a cannot be infinity in that
/// case).
/// 
pub fn gej_add_ge_var(r: *mut Gej, a: *const Gej, b: *const Ge, rzr: *mut Fe) {
    unsafe {
        /* 8 mul, 3 sqr, 4 normalize, 12 mul_int/add/negate */
        let mut z12: Fe = core::mem::zeroed();
        let mut u1: Fe = core::mem::zeroed();
        let mut u2: Fe = core::mem::zeroed();
        let mut s1: Fe = core::mem::zeroed();
        let mut s2: Fe = core::mem::zeroed();
        let mut h: Fe = core::mem::zeroed();
        let mut i: Fe = core::mem::zeroed();
        let mut i2: Fe = core::mem::zeroed();
        let mut h2: Fe = core::mem::zeroed();
        let mut h3: Fe = core::mem::zeroed();
        let mut t: Fe = core::mem::zeroed();

        if (*a).infinity != 0 {
            verify_check!(rzr.is_null());
            gej_set_ge(r, b);
            return;
        }
        if (*b).infinity != 0 {
            if !rzr.is_null() {
                fe_set_int(rzr, 1);
            }
            core::ptr::copy(a, r, 1);
            return;
        }

        (*r).infinity = 0;

        fe_sqr(core::ptr::addr_of_mut!(z12), core::ptr::addr_of!((*a).z));
        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!(u1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(u1));
        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!((*b).x),
            core::ptr::addr_of!(z12),
        );
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!(s1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(s1));
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!((*b).y),
            core::ptr::addr_of!(z12),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!(s2),
            core::ptr::addr_of!((*a).z),
        );
        fe_negate(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u1), 1);
        fe_add(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u2));
        fe_negate(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s1), 1);
        fe_add(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s2));

        if fe_normalizes_to_zero_var(core::ptr::addr_of!(h)) != 0 {
            if fe_normalizes_to_zero_var(core::ptr::addr_of!(i)) != 0 {
                gej_double_var(r, a, rzr);
            } else {
                if !rzr.is_null() {
                    fe_set_int(rzr, 0);
                }
                gej_set_infinity(r);
            }
            return;
        }

        fe_sqr(core::ptr::addr_of_mut!(i2), core::ptr::addr_of!(i));
        fe_sqr(core::ptr::addr_of_mut!(h2), core::ptr::addr_of!(h));
        fe_mul(
            core::ptr::addr_of_mut!(h3),
            core::ptr::addr_of!(h),
            core::ptr::addr_of!(h2),
        );
        if !rzr.is_null() {
            core::ptr::copy(core::ptr::addr_of!(h), rzr, 1);
        }
        fe_mul(
            core::ptr::addr_of_mut!((*r).z),
            core::ptr::addr_of!((*a).z),
            core::ptr::addr_of!(h),
        );
        fe_mul(
            core::ptr::addr_of_mut!(t),
            core::ptr::addr_of!(u1),
            core::ptr::addr_of!(h2),
        );
        core::ptr::copy(core::ptr::addr_of!(t), core::ptr::addr_of_mut!((*r).x), 1);
        fe_mul_int(core::ptr::addr_of_mut!((*r).x), 2);
        fe_add(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!(h3));
        let rx_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).x);
        fe_negate(rx_ptr, rx_ptr as *const Fe, 3);
        fe_add(rx_ptr, core::ptr::addr_of!(i2));

        fe_negate(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*r).x),
            5,
        );
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(t));
        let ry_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_mul(ry_ptr, ry_ptr as *const Fe, core::ptr::addr_of!(i));

        let h3_ptr: *mut Fe = core::ptr::addr_of_mut!(h3);
        fe_mul(h3_ptr, h3_ptr as *const Fe, core::ptr::addr_of!(s1));
        fe_negate(h3_ptr, h3_ptr as *const Fe, 1);
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(h3));
    }
}

#[cfg(test)]
mod gej_add_ge_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_add_ge_var_identity_cases_and_rzr_relation() {
        tracing::info!("Validating gej_add_ge_var identity behavior and rzr z-relation when requested.");

        unsafe {
            let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);

            let mut a_inf: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a_inf));

            let mut r1: Gej = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r1),
                core::ptr::addr_of!(a_inf),
                g_ptr,
                core::ptr::null_mut(),
            );

            let mut expected1: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(expected1), g_ptr);
            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r1, &expected1));

            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), g_ptr);

            let mut b_inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(b_inf));

            let mut rzr: Fe = core::mem::zeroed();
            let mut r2: Gej = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r2),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(b_inf),
                core::ptr::addr_of_mut!(rzr),
            );

            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r2, &a));

            let fe_int1 = secp256k1_group_exhaustive_test_support::fe_int(1);

            assert!(
                fe_equal_var(
                    core::ptr::addr_of!(rzr),
                    core::ptr::addr_of!(fe_int1)
                ) != 0
            );

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(7);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(s));

            let mut r3: Gej = core::mem::zeroed();
            let mut rzr3: Fe = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r3),
                core::ptr::addr_of!(a),
                g_ptr,
                core::ptr::addr_of_mut!(rzr3),
            );

            let mut expected_z: Fe = core::mem::zeroed();
            fe_mul(
                core::ptr::addr_of_mut!(expected_z),
                core::ptr::addr_of!(a.z),
                core::ptr::addr_of!(rzr3),
            );

            assert!(
                fe_equal_var(core::ptr::addr_of!(r3.z), core::ptr::addr_of!(expected_z)) != 0
            );

            let mut neg_g: Ge = core::mem::zeroed();
            ge_neg(core::ptr::addr_of_mut!(neg_g), g_ptr);

            let mut r4: Gej = core::mem::zeroed();
            let mut rzr4: Fe = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r4),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(neg_g),
                core::ptr::addr_of_mut!(rzr4),
            );

            assert!(gej_is_infinity(core::ptr::addr_of!(r4)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(rzr4)) != 0);
        }
    }

    #[traced_test]
    fn gej_add_ge_var_doubling_path_matches_gej_double_var_and_reports_rzr() {
        tracing::info!("Validating gej_add_ge_var doubling case calls through to gej_double_var semantics.");

        unsafe {
            let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);

            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), g_ptr);

            let mut r_add: Gej = core::mem::zeroed();
            let mut rzr_add: Fe = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r_add),
                core::ptr::addr_of!(a),
                g_ptr,
                core::ptr::addr_of_mut!(rzr_add),
            );

            let mut r_dbl: Gej = core::mem::zeroed();
            let mut rzr_dbl: Fe = core::mem::zeroed();
            gej_double_var(
                core::ptr::addr_of_mut!(r_dbl),
                core::ptr::addr_of!(a),
                core::ptr::addr_of_mut!(rzr_dbl),
            );

            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r_add, &r_dbl));
            assert!(
                fe_equal_var(core::ptr::addr_of!(rzr_add), core::ptr::addr_of!(rzr_dbl)) != 0
            );
        }
    }
}
