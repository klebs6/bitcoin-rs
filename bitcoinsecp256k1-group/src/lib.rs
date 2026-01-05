// ---------------- [ File: bitcoinsecp256k1-group/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group_impl.h]

x!{constants}
x!{ge_const}
x!{ge}
x!{ge_clear}
x!{ge_from_storage}
x!{ge_globalz_set_table_gej}
x!{ge_is_in_correct_subgroup}
x!{ge_is_infinity}
x!{ge_is_valid_var}
x!{ge_mul_lambda}
x!{ge_neg}
x!{ge_set_all_gej_var}
x!{ge_set_gej}
x!{ge_set_gej_var}
x!{ge_set_gej_zinv}
x!{ge_set_infinity}
x!{ge_set_xo_var}
x!{ge_set_xy}
x!{ge_storage}
x!{ge_storage_cmov}
x!{ge_to_storage}
x!{gej}
x!{gej_add_ge}
x!{gej_add_ge_var}
x!{gej_add_var}
x!{gej_add_zinv_var}
x!{gej_clear}
x!{gej_const}
x!{gej_double}
x!{gej_double_var}
x!{gej_eq_x_var}
x!{gej_is_infinity}
x!{gej_neg}
x!{gej_rescale}
x!{gej_set_ge}
x!{gej_set_infinity}

#[cfg(test)]
x!{secp256k1_group_exhaustive_test_support}

#[cfg(test)]
mod lib_rs_exhaustive_wiring_smoke_tests {
    use super::*;

    #[traced_test]
    fn crate_exports_and_core_group_primitives_are_linkable() {
        tracing::info!("Validating that core symbols are linkable from the crate root.");

        let _ = core::mem::size_of::<Fe>();
        let _ = core::mem::size_of::<Ge>();
        let _ = core::mem::size_of::<Gej>();
        let _ = core::mem::size_of::<GeStorage>();

        let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);
        tracing::debug!("Checking generator constant basic invariants.");
        assert!(ge_is_infinity(g_ptr) == 0);
        assert!(ge_is_valid_var(g_ptr) != 0);
    }
}

#[cfg(test)]
mod imports_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn imports_module_reexports_expected_core_symbols() {
        tracing::info!("Smoke-checking imports module reexports and tracing availability.");

        let _ = core::mem::size_of::<Fe>();
        let _ = core::mem::size_of::<Ge>();
        let _ = core::mem::size_of::<Gej>();
        let _ = core::mem::size_of::<GeStorage>();

        tracing::debug!("Imports smoke test complete.");
    }
}

#[cfg(test)]
mod lib_rs_adversarial_group_law_and_subgroup_membership_tests {
    use super::*;

    #[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
    #[traced_test]
    fn addition_variants_agree_on_small_prefix_and_associativity_holds() {
        tracing::info!(
            "Cross-checking group law invariants and agreement between add variants in normal (secp256k1) mode."
        );

        unsafe {
            const N: usize = 32;
            const ASSOC: usize = 8;

            let jac: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let mut aff: [Ge; N] = core::array::from_fn(|_| core::mem::zeroed());
            let mut i: usize = 0;
            while i < N {
                aff[i] = secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(&jac[i]);

                if i == 0 {
                    assert!(ge_is_infinity(core::ptr::addr_of!(aff[i])) != 0);
                    assert!(ge_is_valid_var(core::ptr::addr_of!(aff[i])) == 0);
                } else {
                    assert!(ge_is_infinity(core::ptr::addr_of!(aff[i])) == 0);
                    assert!(ge_is_valid_var(core::ptr::addr_of!(aff[i])) != 0);
                }

                i += 1;
            }

            let mut a_idx: usize = 0;
            while a_idx < N {
                let mut b_idx: usize = 0;
                while b_idx < N {
                    let sum_var: Gej =
                        secp256k1_group_exhaustive_test_support::gej_add_var_result(&jac[a_idx], &jac[b_idx]);

                    let mut sum_ge_var: Gej = core::mem::zeroed();
                    gej_add_ge_var(
                        core::ptr::addr_of_mut!(sum_ge_var),
                        core::ptr::addr_of!(jac[a_idx]),
                        core::ptr::addr_of!(aff[b_idx]),
                        core::ptr::null_mut(),
                    );

                    if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_var, &sum_ge_var) {
                        tracing::error!(
                            a = a_idx,
                            b = b_idx,
                            "Mismatch: gej_add_var vs gej_add_ge_var."
                        );
                    }
                    assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                        &sum_var, &sum_ge_var
                    ));

                    if b_idx != 0 {
                        let mut sum_ct: Gej = core::mem::zeroed();
                        gej_add_ge(
                            core::ptr::addr_of_mut!(sum_ct),
                            core::ptr::addr_of!(jac[a_idx]),
                            core::ptr::addr_of!(aff[b_idx]),
                        );

                        if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_ct, &sum_ge_var) {
                            tracing::error!(
                                a = a_idx,
                                b = b_idx,
                                "Mismatch: gej_add_ge (CT) vs gej_add_ge_var."
                            );
                        }
                        assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                            &sum_ct, &sum_ge_var
                        ));
                    }

                    b_idx += 1;
                }
                a_idx += 1;
            }

            let mut x: usize = 0;
            while x < ASSOC {
                let mut y: usize = 0;
                while y < ASSOC {
                    let mut z: usize = 0;
                    while z < ASSOC {
                        let xy: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[x], &jac[y],
                        );
                        let left: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &xy, &jac[z],
                        );

                        let yz: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[y], &jac[z],
                        );
                        let right: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[x], &yz,
                        );

                        if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&left, &right) {
                            tracing::error!(x = x, y = y, z = z, "Associativity failure on subset.");
                        }
                        assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                            &left, &right
                        ));

                        z += 1;
                    }
                    y += 1;
                }
                x += 1;
            }
        }
    }

    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    #[traced_test]
    fn exhaustive_addition_variants_agree_and_match_scalar_table_on_broad_coverage() {
        tracing::info!(
            "Exhaustive-mode: validating broad agreement among add variants and the scalar table modulo subgroup order."
        );

        unsafe {
            const N: usize = EXHAUSTIVE_TEST_ORDER_U32 as usize;
            const EXTRA_LIMIT: usize = 32;

            let jac: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let mut aff: [Ge; N] = core::array::from_fn(|_| core::mem::zeroed());
            let mut b_xy: [Ge; N] = core::array::from_fn(|_| core::mem::zeroed());
            let mut b_zinv: [Fe; N] = core::array::from_fn(|_| core::mem::zeroed());

            let mut i: usize = 0;
            while i < N {
                aff[i] = secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(&jac[i]);

                ge_set_xy(
                    core::ptr::addr_of_mut!(b_xy[i]),
                    core::ptr::addr_of!(jac[i].x),
                    core::ptr::addr_of!(jac[i].y),
                );
                b_xy[i].infinity = jac[i].infinity;

                if jac[i].infinity != 0 {
                    b_zinv[i] = secp256k1_group_exhaustive_test_support::fe_int(0);
                } else {
                    if fe_is_zero(core::ptr::addr_of!(jac[i].z)) != 0 {
                        tracing::warn!(index = i, "Unexpected jacobian z=0 for non-infinity point; zinv set to 0.");
                        b_zinv[i] = secp256k1_group_exhaustive_test_support::fe_int(0);
                    } else {
                        fe_inv_var(
                            core::ptr::addr_of_mut!(b_zinv[i]),
                            core::ptr::addr_of!(jac[i].z),
                        );
                    }
                }

                assert!(ge_is_in_correct_subgroup(core::ptr::addr_of!(aff[i])) != 0);

                if i == 0 {
                    assert!(ge_is_infinity(core::ptr::addr_of!(aff[i])) != 0);
                    assert!(ge_is_valid_var(core::ptr::addr_of!(aff[i])) == 0);
                } else {
                    assert!(ge_is_infinity(core::ptr::addr_of!(aff[i])) == 0);
                    assert!(ge_is_valid_var(core::ptr::addr_of!(aff[i])) != 0);
                }

                i += 1;
            }

            let extra: usize = if N < EXTRA_LIMIT { N } else { EXTRA_LIMIT };

            let mut a_idx: usize = 0;
            while a_idx < N {
                let mut b_idx: usize = 0;
                while b_idx < N {
                    let sum_var: Gej =
                        secp256k1_group_exhaustive_test_support::gej_add_var_result(&jac[a_idx], &jac[b_idx]);

                    let expected_idx: usize = (a_idx + b_idx) % N;
                    if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_var, &jac[expected_idx]) {
                        tracing::error!(
                            a = a_idx,
                            b = b_idx,
                            expected = expected_idx,
                            "gej_add_var result does not match scalar table modulo subgroup order."
                        );
                    }
                    assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                        &sum_var,
                        &jac[expected_idx]
                    ));

                    let mut sum_ge_var: Gej = core::mem::zeroed();
                    gej_add_ge_var(
                        core::ptr::addr_of_mut!(sum_ge_var),
                        core::ptr::addr_of!(jac[a_idx]),
                        core::ptr::addr_of!(aff[b_idx]),
                        core::ptr::null_mut(),
                    );

                    if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_var, &sum_ge_var) {
                        tracing::error!(a = a_idx, b = b_idx, "Mismatch: gej_add_var vs gej_add_ge_var.");
                    }
                    assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                        &sum_var, &sum_ge_var
                    ));

                    if a_idx < extra && b_idx < extra {
                        let mut sum_zinv: Gej = core::mem::zeroed();
                        gej_add_zinv_var(
                            core::ptr::addr_of_mut!(sum_zinv),
                            core::ptr::addr_of!(jac[a_idx]),
                            core::ptr::addr_of!(b_xy[b_idx]),
                            core::ptr::addr_of!(b_zinv[b_idx]),
                        );

                        if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_zinv, &sum_var) {
                            tracing::error!(a = a_idx, b = b_idx, "Mismatch: gej_add_zinv_var vs gej_add_var.");
                        }
                        assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                            &sum_zinv, &sum_var
                        ));

                        if b_idx != 0 {
                            let mut sum_ct: Gej = core::mem::zeroed();
                            gej_add_ge(
                                core::ptr::addr_of_mut!(sum_ct),
                                core::ptr::addr_of!(jac[a_idx]),
                                core::ptr::addr_of!(aff[b_idx]),
                            );

                            if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&sum_ct, &sum_var) {
                                tracing::error!(a = a_idx, b = b_idx, "Mismatch: gej_add_ge (CT) vs gej_add_var.");
                            }
                            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                                &sum_ct, &sum_var
                            ));
                        }
                    }

                    b_idx += 1;
                }

                a_idx += 1;
            }
        }
    }

    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    #[traced_test]
    fn exhaustive_associativity_holds_on_reasonable_subset() {
        tracing::info!("Exhaustive-mode: validating associativity on a bounded subset of subgroup elements.");

        unsafe {
            const N: usize = EXHAUSTIVE_TEST_ORDER_U32 as usize;
            const MAX_SUBSET: usize = 16;

            let jac: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let m: usize = if N < MAX_SUBSET { N } else { MAX_SUBSET };

            let mut x: usize = 0;
            while x < m {
                let mut y: usize = 0;
                while y < m {
                    let mut z: usize = 0;
                    while z < m {
                        let xy: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[x], &jac[y],
                        );
                        let left: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &xy, &jac[z],
                        );

                        let yz: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[y], &jac[z],
                        );
                        let right: Gej = secp256k1_group_exhaustive_test_support::gej_add_var_result(
                            &jac[x], &yz,
                        );

                        if !secp256k1_group_exhaustive_test_support::gej_affine_eq(&left, &right) {
                            tracing::error!(x = x, y = y, z = z, "Associativity failure on subset.");
                        }
                        assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                            &left, &right
                        ));

                        z += 1;
                    }
                    y += 1;
                }
                x += 1;
            }
        }
    }

    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    #[traced_test]
    fn subgroup_membership_predicate_has_no_false_positives_in_small_search_window() {
        tracing::info!(
            "Exhaustive-mode: validating ge_is_in_correct_subgroup has no false positives in a bounded x-search window."
        );

        unsafe {
            const N: usize = EXHAUSTIVE_TEST_ORDER_U32 as usize;
            const SEARCH: i32 = 1024;

            let jac: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let mut subgroup: [Ge; N] = core::array::from_fn(|_| core::mem::zeroed());
            let mut i: usize = 0;
            while i < N {
                subgroup[i] =
                    secp256k1_group_exhaustive_test_support::ge_from_gej_via_set_gej_var(&jac[i]);
                assert!(ge_is_in_correct_subgroup(core::ptr::addr_of!(subgroup[i])) != 0);
                i += 1;
            }

            let mut x: i32 = 0;
            while x < SEARCH {
                let fx: Fe = secp256k1_group_exhaustive_test_support::fe_int(x);

                let mut cand: Ge = core::mem::zeroed();
                if ge_set_xo_var(
                    core::ptr::addr_of_mut!(cand),
                    core::ptr::addr_of!(fx),
                    0,
                ) != 0
                {
                    assert!(ge_is_valid_var(core::ptr::addr_of!(cand)) != 0);

                    if ge_is_in_correct_subgroup(core::ptr::addr_of!(cand)) != 0 {
                        let mut found: i32 = 0;

                        let mut j: usize = 0;
                        while j < N && found == 0 {
                            if secp256k1_group_exhaustive_test_support::ge_eq(&cand, &subgroup[j]) {
                                found = 1;
                            }
                            j += 1;
                        }

                        if found == 0 {
                            tracing::error!(
                                x = x,
                                "Subgroup predicate accepted a point that is not a generated subgroup element."
                            );
                        }
                        assert!(found != 0);
                    }
                }

                x += 1;
            }
        }
    }
}
