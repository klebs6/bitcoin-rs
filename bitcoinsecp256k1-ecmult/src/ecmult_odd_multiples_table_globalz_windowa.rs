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
