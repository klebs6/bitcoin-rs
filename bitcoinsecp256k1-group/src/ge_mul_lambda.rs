// ---------------- [ File: bitcoinsecp256k1-group/src/ge_mul_lambda.rs ]
crate::ix!();

/// Set r to be equal to lambda times a, where lambda is chosen in a way such that this is very
/// fast.
/// 
pub fn ge_mul_lambda(r: *mut Ge, a: *const Ge) {
    unsafe {
        static beta: Fe = fe_const!(
            0x7ae96a2b_u32,
            0x657c0710_u32,
            0x6e64479e_u32,
            0xac3434e9_u32,
            0x9cf04975_u32,
            0x12f58995_u32,
            0xc1396c28_u32,
            0x719501ee_u32
        );
        core::ptr::copy(a, r, 1);
        let rx: *mut Fe = core::ptr::addr_of_mut!((*r).x);
        fe_mul(rx, rx as *const Fe, core::ptr::addr_of!(beta));
    }
}

#[cfg(test)]
mod ge_mul_lambda_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_mul_lambda_matches_manual_beta_x_and_preserves_y() {
        tracing::info!("Validating ge_mul_lambda: x' = beta*x, y unchanged, and result remains on-curve.");

        unsafe {
            let mut out: Ge = core::mem::zeroed();
            ge_mul_lambda(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(ge_const_g));

            let beta: Fe = fe_const!(
                0x7ae96a2b_u32,
                0x657c0710_u32,
                0x6e64479e_u32,
                0xac3434e9_u32,
                0x9cf04975_u32,
                0x12f58995_u32,
                0xc1396c28_u32,
                0x719501ee_u32
            );

            let mut expected_x: Fe = core::mem::zeroed();
            core::ptr::copy(
                core::ptr::addr_of!(ge_const_g.x),
                core::ptr::addr_of_mut!(expected_x),
                1,
            );
            fe_mul(
                core::ptr::addr_of_mut!(expected_x),
                core::ptr::addr_of!(expected_x),
                core::ptr::addr_of!(beta),
            );

            assert!(
                fe_equal_var(core::ptr::addr_of!(out.x), core::ptr::addr_of!(expected_x)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(out.y), core::ptr::addr_of!(ge_const_g.y)) != 0
            );

            assert!(ge_is_infinity(core::ptr::addr_of!(out)) == 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(out)) != 0);
        }
    }

    #[traced_test]
    fn ge_mul_lambda_preserves_infinity_flag() {
        tracing::info!("Validating ge_mul_lambda preserves infinity and keeps coordinates zeroed for infinity.");

        unsafe {
            let mut inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(inf));

            let mut out: Ge = core::mem::zeroed();
            ge_mul_lambda(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(inf));

            assert!(ge_is_infinity(core::ptr::addr_of!(out)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(out.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(out.y)) != 0);
        }
    }
}

#[cfg(test)]
mod ge_mul_lambda_rs_adversarial_endomorphism_tests {
    use super::*;

    #[traced_test]
    fn lambda_is_order_three_and_preserves_curve_membership_on_sampled_points() {
        tracing::info!(
            "Validating lambda endomorphism has order 3 (lambda^3 == id) and preserves curve validity on sampled points."
        );

        unsafe {
            const N: usize = 32;

            let points: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let mut i: usize = 0;
            while i < N {
                let p: Ge = secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(
                    &points[i],
                );

                let mut l1: Ge = core::mem::zeroed();
                ge_mul_lambda(
                    core::ptr::addr_of_mut!(l1),
                    core::ptr::addr_of!(p),
                );

                let mut l2: Ge = core::mem::zeroed();
                ge_mul_lambda(
                    core::ptr::addr_of_mut!(l2),
                    core::ptr::addr_of!(l1),
                );

                let mut l3: Ge = core::mem::zeroed();
                ge_mul_lambda(
                    core::ptr::addr_of_mut!(l3),
                    core::ptr::addr_of!(l2),
                );

                if !secp256k1_group_exhaustive_test_support::ge_eq(&l3, &p) {
                    tracing::error!(index = i, "lambda^3(P) != P for sampled point.");
                }
                assert!(secp256k1_group_exhaustive_test_support::ge_eq(&l3, &p));

                if ge_is_infinity(core::ptr::addr_of!(l1)) == 0 {
                    assert!(ge_is_valid_var(core::ptr::addr_of!(l1)) != 0);
                }

                i += 1;
            }
        }
    }

    #[traced_test]
    fn lambda_commutes_with_scalar_multiples_in_the_generator_subgroup() {
        tracing::info!(
            "Validating lambda(k*G) == k*lambda(G) for a sampled table of multiples."
        );

        unsafe {
            const N: usize = 32;

            let mut lambda_g: Ge = core::mem::zeroed();
            ge_mul_lambda(
                core::ptr::addr_of_mut!(lambda_g),
                core::ptr::addr_of!(ge_const_g),
            );

            let points_g: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );
            let points_lg: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &lambda_g,
                );

            let mut k: usize = 0;
            while k < N {
                let kg_aff: Ge =
                    secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(&points_g[k]);
                let klg_aff: Ge =
                    secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(&points_lg[k]);

                let mut lambda_kg: Ge = core::mem::zeroed();
                ge_mul_lambda(
                    core::ptr::addr_of_mut!(lambda_kg),
                    core::ptr::addr_of!(kg_aff),
                );

                if !secp256k1_group_exhaustive_test_support::ge_eq(&lambda_kg, &klg_aff) {
                    tracing::error!(
                        k = k,
                        "lambda(k*G) != k*lambda(G) for sampled multiple."
                    );
                }
                assert!(secp256k1_group_exhaustive_test_support::ge_eq(&lambda_kg, &klg_aff));

                if ge_is_infinity(core::ptr::addr_of!(lambda_kg)) == 0 {
                    assert!(ge_is_valid_var(core::ptr::addr_of!(lambda_kg)) != 0);
                }

                k += 1;
            }
        }
    }
}
