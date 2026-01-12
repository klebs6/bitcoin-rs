// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult.rs ]
crate::ix!();

/// Double multiply: R = na*A + ng*G
///
pub fn ecmult(
    ctx: *const EcMultContext,
    r:   *mut Gej,
    a:   *const Gej,
    na:  *const Scalar,
    ng:  *const Scalar,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult");

    unsafe {
        let mut prej: [Gej; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Gej; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut zr: [Fe; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Fe; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut pre_a: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut ps: [StraussPointState; 1] =
            core::mem::MaybeUninit::<[StraussPointState; 1]>::uninit().assume_init();
        let mut pre_a_lam: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();

        let mut state = StraussState::new();
        state.set_prej(prej.as_mut_ptr());
        state.set_zr(zr.as_mut_ptr());
        state.set_pre_a(pre_a.as_mut_ptr());
        state.set_pre_a_lam(pre_a_lam.as_mut_ptr());
        state.set_ps(ps.as_mut_ptr());

        ecmult_strauss_wnaf(ctx, core::ptr::addr_of!(state), r, 1usize, a, na, ng);
    }
}

#[cfg(test)]
mod ecmult_double_multiply_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_with_zero_na_and_null_ng_returns_infinity() {
        tracing::info!(target: "secp256k1::ecmult::tests", "ecmult_with_zero_na_and_null_ng_returns_infinity");

        unsafe {
            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let na = scalar_from_u32(0);

            let mut r = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(na),
                core::ptr::null(),
            );

            let inf = gej_is_infinity(core::ptr::addr_of!(r)) != 0;
            tracing::debug!(target: "secp256k1::ecmult::tests", result_is_infinity = inf, "ecmult result");
            assert!(inf);
        }
    }

    #[traced_test]
    fn ecmult_with_na_one_returns_input_point_when_ng_is_null() {
        tracing::info!(target: "secp256k1::ecmult::tests", "ecmult_with_na_one_returns_input_point_when_ng_is_null");

        unsafe {
            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let na = scalar_from_u32(1);

            let mut r = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(na),
                core::ptr::null(),
            );

            gej_assert_eq_via_add_neg("na=1, ng=NULL", core::ptr::addr_of!(r), core::ptr::addr_of!(a));
        }
    }

    #[traced_test]
    fn ecmult_with_small_na_matches_naive_double_and_add_for_generator() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_with_small_na_matches_naive_double_and_add_for_generator"
        );

        unsafe {
            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));

            for k in 0u32..=20u32 {
                let na = scalar_from_u32(k);

                let mut r = Gej::new();
                ecmult(
                    core::ptr::null(),
                    core::ptr::addr_of_mut!(r),
                    core::ptr::addr_of!(a),
                    core::ptr::addr_of!(na),
                    core::ptr::null(),
                );

                let expected = gej_mul_small(core::ptr::addr_of!(a), k);
                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    k = k,
                    "comparing ecmult output to naive multiplication"
                );
                gej_assert_eq_via_add_neg("small-na", core::ptr::addr_of!(r), core::ptr::addr_of!(expected));
            }
        }
    }

    #[traced_test]
    fn ecmult_ignores_infinity_input_point_when_ng_is_null() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_ignores_infinity_input_point_when_ng_is_null"
        );

        unsafe {
            let mut a = Gej::new();
            gej_set_infinity(core::ptr::addr_of_mut!(a));
            let na = scalar_from_u32(7);

            let mut r = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(na),
                core::ptr::null(),
            );

            assert!(gej_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }
}
