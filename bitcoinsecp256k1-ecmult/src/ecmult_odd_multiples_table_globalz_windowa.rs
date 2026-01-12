// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_odd_multiples_table_globalz_windowa.rs ]
crate::ix!();

/// Fill a table 'pre' with precomputed odd multiples of a.
///
/// There are two versions of this function:
///
/// - ecmult_odd_multiples_table_globalz_windowa which brings its resulting point set to a single
/// constant Z denominator, stores the X and Y coordinates as ge_storage points in pre, and stores
/// the global Z in rz.
///
/// It only operates on tables sized for WINDOW_A wnaf multiples.
///
/// - ecmult_odd_multiples_table_storage_var, which converts its resulting point set to actually
/// affine points, and stores those in pre.
///
/// It operates on tables of any size.
///
/// To compute a*P + b*G, we compute a table for P using the first function, and for G using the
/// second (which requires an inverse, but it only needs to happen once).
///
pub fn ecmult_odd_multiples_table_globalz_windowa(
    pre:     *mut Ge,
    globalz: *mut Fe,
    a:       *const Gej,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_odd_multiples_table_globalz_windowa");

    unsafe {
        let mut prej: [Gej; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Gej; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut zr: [Fe; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Fe; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();

        /* Compute the odd multiples in Jacobian form. */
        ecmult_odd_multiples_table(
            ecmult_table_size!(WINDOW_A) as i32,
            prej.as_mut_ptr(),
            zr.as_mut_ptr(),
            a,
        );
        /* Bring them to the same Z denominator. */
        ge_globalz_set_table_gej(
            ecmult_table_size!(WINDOW_A),
            pre,
            globalz,
            prej.as_ptr(),
            zr.as_ptr(),
        );
    }
}

#[cfg(test)]
mod odd_multiples_globalz_windowa_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn odd_multiples_table_globalz_windowa_outputs_non_infinite_points_and_nonzero_globalz() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "odd_multiples_table_globalz_windowa_outputs_non_infinite_points_and_nonzero_globalz"
        );

        unsafe {
            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));

            let mut pre: [Ge; ecmult_table_size!(WINDOW_A)] =
                core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
            let mut globalz: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();

            ecmult_odd_multiples_table_globalz_windowa(
                pre.as_mut_ptr(),
                core::ptr::addr_of_mut!(globalz),
                core::ptr::addr_of!(a),
            );

            let mut i = 0usize;
            while i < ecmult_table_size!(WINDOW_A) {
                assert_eq!(ge_is_infinity(pre.as_ptr().add(i)), 0);
                i += 1;
            }

            fe_normalize_var(core::ptr::addr_of_mut!(globalz));
            let z_is_zero = fe_is_zero(core::ptr::addr_of!(globalz)) != 0;
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                globalz_is_zero = z_is_zero,
                "globalz normalization"
            );
            assert!(!z_is_zero);
        }
    }
}
