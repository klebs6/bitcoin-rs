// ---------------- [ File: bitcoinsecp256k1-group/src/ge_is_in_correct_subgroup.rs ]
crate::ix!();

/// Determine if a point (which is assumed to be on the curve) is in the correct (sub)group of the
/// curve.
/// 
/// In normal mode, the used group is secp256k1, which has cofactor=1 meaning that every point on
/// the curve is in the group, and this function returns always true.
/// 
/// When compiling in exhaustive test mode, a slightly different curve equation is used, leading to
/// a group with a (very) small subgroup, and that subgroup is what is used for all cryptographic
/// operations. In that mode, this function checks whether a point that is on the curve is in fact
/// also in that subgroup.
/// 
pub fn ge_is_in_correct_subgroup(ge: *const Ge) -> i32 {
    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    unsafe {
        let mut out: Gej = core::mem::zeroed();
        let out_ptr: *mut Gej = core::ptr::addr_of_mut!(out);

        let mut i: i32 = 0;

        /* A very simple EC multiplication ladder that avoids a dependency on ecmult. */
        gej_set_infinity(out_ptr);
        while i < 32 {
            gej_double_var(out_ptr, out_ptr as *const Gej, core::ptr::null_mut());
            if (((EXHAUSTIVE_TEST_ORDER_U32) >> (31 - (i as u32))) & 1) != 0 {
                gej_add_ge_var(out_ptr, out_ptr as *const Gej, ge, core::ptr::null_mut());
            }
            i += 1;
        }
        gej_is_infinity(out_ptr as *const Gej)
    }

    #[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
    {
        let _ = ge;
        /* The real secp256k1 group has cofactor 1, so the subgroup is the entire curve. */
        1
    }
}

#[cfg(test)]
mod ge_is_in_correct_subgroup_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn subgroup_check_accepts_generator_and_infinity() {
        tracing::info!("Validating ge_is_in_correct_subgroup accepts generator and infinity.");

        unsafe {
            let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);
            assert!(ge_is_in_correct_subgroup(g_ptr) != 0);

            let mut inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(inf));
            assert!(ge_is_in_correct_subgroup(core::ptr::addr_of!(inf)) != 0);
        }
    }

    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    #[traced_test]
    fn subgroup_check_rejects_generic_curve_point_outside_small_subgroup() {
        tracing::info!("Searching for a curve point not in the small exhaustive subgroup, and validating rejection.");

        unsafe {
            let mut found: i32 = 0;
            let mut candidate: Ge = core::mem::zeroed();

            let mut i: i32 = 2;
            while i < 4096 && found == 0 {
                let x: Fe = secp256k1_group_exhaustive_test_support::fe_int(i);

                if ge_set_xo_var(core::ptr::addr_of_mut!(candidate), core::ptr::addr_of!(x), 0) != 0
                {
                    assert!(ge_is_valid_var(core::ptr::addr_of!(candidate)) != 0);

                    if ge_is_in_correct_subgroup(core::ptr::addr_of!(candidate)) == 0 {
                        found = 1;
                    }
                }

                i += 1;
            }

            if found == 0 {
                tracing::error!("Failed to find a curve point outside the subgroup within the search bound.");
            }
            assert!(found != 0);
        }
    }
}
