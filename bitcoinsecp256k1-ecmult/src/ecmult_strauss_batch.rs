// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_strauss_batch.rs ]
crate::ix!();

pub fn ecmult_strauss_batch(
        error_callback: *const Callback,
        ctx:            *const EcMultContext,
        scratch:        *mut Scratch,
        r:              *mut Gej,
        inp_g_sc:       *const Scalar,
        cb:             EcMultMultiCallback,
        cbdata:         *mut c_void,
        n_points:       usize,
        cb_offset:      usize) -> i32 {
    
    todo!();
        /*
        gej* points;
        scalar* scalars;
        struct strauss_state state;
        size_t i;
        const size_t scratch_checkpoint = scratch_checkpoint(error_callback, scratch);

        gej_set_infinity(r);
        if (inp_g_sc == NULL && n_points == 0) {
            return 1;
        }

        points = (gej*)scratch_alloc(error_callback, scratch, n_points * sizeof(gej));
        scalars = (scalar*)scratch_alloc(error_callback, scratch, n_points * sizeof(scalar));
        state.prej = (gej*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(gej));
        state.zr = (fe*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(fe));
        state.pre_a = (ge*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(ge));
        state.pre_a_lam = (ge*)scratch_alloc(error_callback, scratch, n_points * ECMULT_TABLE_SIZE(WINDOW_A) * sizeof(ge));
        state.ps = (struct strauss_point_state*)scratch_alloc(error_callback, scratch, n_points * sizeof(struct strauss_point_state));

        if (points == NULL || scalars == NULL || state.prej == NULL || state.zr == NULL || state.pre_a == NULL || state.pre_a_lam == NULL || state.ps == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        for (i = 0; i < n_points; i++) {
            ge point;
            if (!cb(&scalars[i], &point, i+cb_offset, cbdata)) {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            gej_set_ge(&points[i], &point);
        }
        ecmult_strauss_wnaf(ctx, &state, r, n_points, points, scalars, inp_g_sc);
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        return 1;
        */
}
