// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_simple_var.rs ]
crate::ix!();

/// Computes ecmult_multi by simply multiplying and adding each point. Does not require a scratch
/// space
///
pub fn ecmult_multi_simple_var(
    ctx:      *const EcMultContext,
    r:        *mut Gej,
    inp_g_sc: *const Scalar,
    cb:       EcMultMultiCallback,
    cbdata:   *mut c_void,
    n_points: usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n_points = n_points,
        "ecmult_multi_simple_var"
    );

    unsafe {
        let mut point_idx: usize;
        let mut szero = Scalar::new();
        let mut tmpj = Gej::new();

        scalar_set_int(core::ptr::addr_of_mut!(szero), 0);
        gej_set_infinity(r);
        gej_set_infinity(core::ptr::addr_of_mut!(tmpj));
        /* r = inp_g_sc*G */
        ecmult(
            ctx,
            r,
            core::ptr::addr_of!(tmpj),
            core::ptr::addr_of!(szero),
            inp_g_sc,
        );
        point_idx = 0;
        while point_idx < n_points {
            let mut point = Ge::new();
            let mut pointj = Gej::new();
            let mut scalar = Scalar::new();

            if cb(core::ptr::addr_of_mut!(scalar), core::ptr::addr_of_mut!(point), point_idx, cbdata)
                == 0
            {
                return 0;
            }
            /* r += scalar*point */
            gej_set_ge(core::ptr::addr_of_mut!(pointj), core::ptr::addr_of!(point));
            ecmult(
                ctx,
                core::ptr::addr_of_mut!(tmpj),
                core::ptr::addr_of!(pointj),
                core::ptr::addr_of!(scalar),
                core::ptr::null(),
            );
            gej_add_var(r, r, core::ptr::addr_of!(tmpj), core::ptr::null_mut());

            point_idx += 1;
        }
        1
    }
}

#[cfg(test)]
mod ecmult_multi_simple_var_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[repr(C)]
    struct SimpleVarCallbackData {
        scalars: [u32; 3],
    }

    fn three_scalar_generator_callback(sc: *mut Scalar, pt: *mut Ge, idx: usize, data: *mut c_void) -> i32 {
        unsafe {
            let data = &*(data as *const SimpleVarCallbackData);
            if idx >= data.scalars.len() {
                return 0;
            }
            scalar_set_int(sc, data.scalars[idx]);
            core::ptr::write(pt, ge_const_g);
        }
        1
    }

    fn fail_at_index_one_callback(_sc: *mut Scalar, _pt: *mut Ge, idx: usize, _data: *mut c_void) -> i32 {
        if idx == 1 {
            return 0;
        }
        1
    }

    #[traced_test]
    fn multi_simple_var_with_no_points_and_null_g_scalar_returns_infinity() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "multi_simple_var_with_no_points_and_null_g_scalar_returns_infinity"
        );

        unsafe {
            let mut r = Gej::new();
            let ok = ecmult_multi_simple_var(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::null(),
                three_scalar_generator_callback,
                core::ptr::null_mut(),
                0,
            );
            assert_eq!(ok, 1);
            assert!(gej_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }

    #[traced_test]
    fn multi_simple_var_returns_zero_when_callback_returns_zero() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "multi_simple_var_returns_zero_when_callback_returns_zero"
        );

        unsafe {
            let mut r = Gej::new();
            let ok = ecmult_multi_simple_var(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::null(),
                fail_at_index_one_callback,
                core::ptr::null_mut(),
                3,
            );

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                "callback failure propagation"
            );

            assert_eq!(ok, 0);
        }
    }

    #[traced_test]
    fn multi_simple_var_matches_naive_sum_for_three_small_scalars_on_generator() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "multi_simple_var_matches_naive_sum_for_three_small_scalars_on_generator"
        );

        unsafe {
            let data = SimpleVarCallbackData {
                scalars: [1u32, 2u32, 3u32],
            };

            let mut r = Gej::new();
            let ok = ecmult_multi_simple_var(
                core::ptr::null(),
                core::ptr::addr_of_mut!(r),
                core::ptr::null(),
                three_scalar_generator_callback,
                core::ptr::addr_of!(data) as *mut c_void,
                3,
            );
            assert_eq!(ok, 1);

            let g = gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let expected = gej_mul_small(core::ptr::addr_of!(g), 6);
            gej_assert_eq_via_add_neg(
                "multi_simple_var sum",
                core::ptr::addr_of!(r),
                core::ptr::addr_of!(expected),
            );
        }
    }
}
