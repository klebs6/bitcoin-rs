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
        n_points: usize) -> i32 {
    
    todo!();
        /*
        size_t point_idx;
        scalar szero;
        gej tmpj;

        scalar_set_int(&szero, 0);
        gej_set_infinity(r);
        gej_set_infinity(&tmpj);
        /* r = inp_g_sc*G */
        ecmult(ctx, r, &tmpj, &szero, inp_g_sc);
        for (point_idx = 0; point_idx < n_points; point_idx++) {
            ge point;
            gej pointj;
            scalar scalar;
            if (!cb(&scalar, &point, point_idx, cbdata)) {
                return 0;
            }
            /* r += scalar*point */
            gej_set_ge(&pointj, &point);
            ecmult(ctx, &tmpj, &pointj, &scalar, NULL);
            gej_add_var(r, r, &tmpj, NULL);
        }
        return 1;
        */
}
