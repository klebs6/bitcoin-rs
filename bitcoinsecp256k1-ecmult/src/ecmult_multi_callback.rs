// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_callback.rs ]
crate::ix!();

pub type EcMultMultiCallback = fn(
    sc:   *mut Scalar,
    pt:   *mut Ge,
    idx:  usize,
    data: *mut c_void
) -> i32;

#[cfg(test)]
mod ecmult_multi_callback_type_contract_suite {
    use super::*;

    #[traced_test]
    fn callback_type_accepts_expected_signature_and_can_be_invoked() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "callback_type_accepts_expected_signature_and_can_be_invoked"
        );

        fn sample_callback(sc: *mut Scalar, pt: *mut Ge, idx: usize, _data: *mut c_void) -> i32 {
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                idx = idx,
                "sample_callback invoked"
            );

            unsafe {
                scalar_set_int(sc, idx as u32);
                core::ptr::write(pt, ge_const_g);
            }

            1
        }

        let cb: EcMultMultiCallback = sample_callback;

        unsafe {
            let idx: usize = 7;

            let mut sc = Scalar::new();
            let mut pt = Ge::new();

            let ok = cb(
                core::ptr::addr_of_mut!(sc),
                core::ptr::addr_of_mut!(pt),
                idx,
                core::ptr::null_mut(),
            );

            let pt_is_inf = ge_is_infinity(core::ptr::addr_of!(pt));

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                ok = ok,
                idx = idx,
                pt_is_infinity = pt_is_inf,
                "callback invocation results"
            );

            assert_eq!(ok, 1);
            assert_eq!(pt_is_inf, 0);

            // Strong semantic check (avoids scalar_get_bits_var(…, 32) which can overflow in some backends):
            // Ensure the callback-produced (sc, pt) satisfies sc*pt == idx*G.
            let pt_j = crate::ecmult_test_harness::gej_from_ge(core::ptr::addr_of!(pt));

            let mut got = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(got),
                core::ptr::addr_of!(pt_j),
                core::ptr::addr_of!(sc),
                core::ptr::null(),
            );

            let g_j = crate::ecmult_test_harness::gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let expected = crate::ecmult_test_harness::gej_mul_small(core::ptr::addr_of!(g_j), idx as u32);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                idx = idx,
                "comparing callback-derived multiplication to expected generator multiple"
            );

            crate::ecmult_test_harness::gej_assert_eq_via_add_neg(
                "ecmult_multi_callback contract",
                core::ptr::addr_of!(got),
                core::ptr::addr_of!(expected),
            );
        }
    }
}
