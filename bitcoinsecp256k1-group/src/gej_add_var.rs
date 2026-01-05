// ---------------- [ File: bitcoinsecp256k1-group/src/gej_add_var.rs ]
crate::ix!();

/// Set r equal to the sum of a and b. 
///
/// If rzr is non-NULL this sets *rzr such that r->z == a->z * *rzr (a cannot be infinity in that
/// case).
/// 
pub fn gej_add_var(r: *mut Gej, a: *const Gej, b: *const Gej, rzr: *mut Fe) {
    unsafe {
        /* Operations: 12 mul, 4 sqr, 2 normalize, 12 mul_int/add/negate */
        let mut z22: Fe = core::mem::zeroed();
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
            core::ptr::copy(b, r, 1);
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
        fe_sqr(core::ptr::addr_of_mut!(z22), core::ptr::addr_of!((*b).z));
        fe_sqr(core::ptr::addr_of_mut!(z12), core::ptr::addr_of!((*a).z));
        fe_mul(
            core::ptr::addr_of_mut!(u1),
            core::ptr::addr_of!((*a).x),
            core::ptr::addr_of!(z22),
        );
        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!((*b).x),
            core::ptr::addr_of!(z12),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s1),
            core::ptr::addr_of!((*a).y),
            core::ptr::addr_of!(z22),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s1),
            core::ptr::addr_of!(s1),
            core::ptr::addr_of!((*b).z),
        );
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
        let h_ptr: *mut Fe = core::ptr::addr_of_mut!(h);
        fe_mul(h_ptr, h_ptr as *const Fe, core::ptr::addr_of!((*b).z));
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
mod gej_add_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_add_var_identity_commutativity_and_inverse_on_small_set() {
        tracing::info!("Validating gej_add_var identity, commutativity, and inverse behavior on a small set.");

        unsafe {
            const N: usize = 9;

            let points: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let mut i: usize = 0;
            while i < N {
                let mut j: usize = 0;
                while j < N {
                    let mut r1: Gej = core::mem::zeroed();
                    gej_add_var(
                        core::ptr::addr_of_mut!(r1),
                        core::ptr::addr_of!(points[i]),
                        core::ptr::addr_of!(points[j]),
                        core::ptr::null_mut(),
                    );

                    let mut r2: Gej = core::mem::zeroed();
                    gej_add_var(
                        core::ptr::addr_of_mut!(r2),
                        core::ptr::addr_of!(points[j]),
                        core::ptr::addr_of!(points[i]),
                        core::ptr::null_mut(),
                    );

                    assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r1, &r2));

                    j += 1;
                }

                let neg_i: Gej = secp256k1_group_exhaustive_test_support::gej_negate_jacobian(&points[i]);

                let mut r_inv: Gej = core::mem::zeroed();
                gej_add_var(
                    core::ptr::addr_of_mut!(r_inv),
                    core::ptr::addr_of!(points[i]),
                    core::ptr::addr_of!(neg_i),
                    core::ptr::null_mut(),
                );

                assert!(gej_is_infinity(core::ptr::addr_of!(r_inv)) != 0);
                i += 1;
            }
        }
    }

    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    #[traced_test]
    fn gej_add_var_matches_scalar_addition_mod_exhaustive_order() {
        tracing::info!("Validating gej_add_var matches scalar addition table modulo the exhaustive subgroup order.");

        unsafe {
            const N: usize = EXHAUSTIVE_TEST_ORDER_U32 as usize;

            let points: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    core::ptr::addr_of!(ge_const_g),
                );

            let mut i: usize = 0;
            while i < N {
                let mut j: usize = 0;
                while j < N {
                    let mut r: Gej = core::mem::zeroed();
                    gej_add_var(
                        core::ptr::addr_of_mut!(r),
                        core::ptr::addr_of!(points[i]),
                        core::ptr::addr_of!(points[j]),
                        core::ptr::null_mut(),
                    );

                    let expected_idx: usize = (i + j) % N;
                    assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                        &r,
                        &points[expected_idx]
                    ));

                    j += 1;
                }
                i += 1;
            }
        }
    }

    #[traced_test]
    fn gej_add_var_rzr_relation_holds_when_requested() {
        tracing::info!("Validating gej_add_var sets rzr such that r.z == a.z * rzr (when a is non-infinity).");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let mut b: Gej = core::mem::zeroed();
            gej_double(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(a));

            let mut r: Gej = core::mem::zeroed();
            let mut rzr: Fe = core::mem::zeroed();
            gej_add_var(
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(b),
                core::ptr::addr_of_mut!(rzr),
            );

            let mut expected_z: Fe = core::mem::zeroed();
            fe_mul(
                core::ptr::addr_of_mut!(expected_z),
                core::ptr::addr_of!(a.z),
                core::ptr::addr_of!(rzr),
            );

            assert!(
                fe_equal_var(core::ptr::addr_of!(r.z), core::ptr::addr_of!(expected_z)) != 0
            );
        }
    }
}

#[cfg(test)]
mod gej_add_var_rs_adversarial_inplace_aliasing_tests {
    use super::*;

    #[traced_test]
    fn gej_add_var_supports_in_place_output_aliasing_inputs_and_preserves_rzr_contract() {
        tracing::info!(
            "Validating gej_add_var correctness when r aliases a or b, including the rzr contract."
        );

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(
                core::ptr::addr_of_mut!(a),
                core::ptr::addr_of!(ge_const_g),
            );

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(7);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(s));

            let mut b: Gej = core::mem::zeroed();
            gej_double(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(a));

            let expected: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(&a, &b);

            /* r aliases a */
            let mut a_inplace: Gej = a;
            let old_a_z: Fe = a_inplace.z;

            let mut rzr_a: Fe = core::mem::zeroed();
            let a_ptr: *mut Gej = core::ptr::addr_of_mut!(a_inplace);

            gej_add_var(
                a_ptr,
                a_ptr as *const Gej,
                core::ptr::addr_of!(b),
                core::ptr::addr_of_mut!(rzr_a),
            );

            if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&a_inplace, &expected) {
                tracing::error!("In-place r==a result mismatch.");
            }
            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                &a_inplace, &expected
            ));

            let mut expected_z_a: Fe = core::mem::zeroed();
            fe_mul(
                core::ptr::addr_of_mut!(expected_z_a),
                core::ptr::addr_of!(old_a_z),
                core::ptr::addr_of!(rzr_a),
            );
            assert!(
                fe_equal_var(
                    core::ptr::addr_of!(a_inplace.z),
                    core::ptr::addr_of!(expected_z_a)
                ) != 0
            );

            /* r aliases b */
            let mut b_inplace: Gej = b;

            let mut rzr_b: Fe = core::mem::zeroed();
            let b_ptr: *mut Gej = core::ptr::addr_of_mut!(b_inplace);

            gej_add_var(
                b_ptr,
                core::ptr::addr_of!(a),
                b_ptr as *const Gej,
                core::ptr::addr_of_mut!(rzr_b),
            );

            if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&b_inplace, &expected) {
                tracing::error!("In-place r==b result mismatch.");
            }
            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                &b_inplace, &expected
            ));

            let mut expected_z_b: Fe = core::mem::zeroed();
            fe_mul(
                core::ptr::addr_of_mut!(expected_z_b),
                core::ptr::addr_of!(a.z),
                core::ptr::addr_of!(rzr_b),
            );
            assert!(
                fe_equal_var(
                    core::ptr::addr_of!(b_inplace.z),
                    core::ptr::addr_of!(expected_z_b)
                ) != 0
            );
        }
    }
}
