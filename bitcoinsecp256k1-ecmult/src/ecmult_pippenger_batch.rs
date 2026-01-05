// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_pippenger_batch.rs ]
crate::ix!();

pub fn ecmult_pippenger_batch(
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
            const size_t scratch_checkpoint = scratch_checkpoint(error_callback, scratch);
        /* Use 2(n+1) with the endomorphism, when calculating batch
         * sizes. The reason for +1 is that we add the G scalar to the list of
         * other scalars. */
        size_t entries = 2*n_points + 2;
        ge *points;
        scalar *scalars;
        gej *buckets;
        struct pippenger_state *state_space;
        size_t idx = 0;
        size_t point_idx = 0;
        int i, j;
        int bucket_window;

        (c_void)ctx;
        gej_set_infinity(r);
        if (inp_g_sc == NULL && n_points == 0) {
            return 1;
        }

        bucket_window = pippenger_bucket_window(n_points);
        points = (ge *) scratch_alloc(error_callback, scratch, entries * sizeof(*points));
        scalars = (scalar *) scratch_alloc(error_callback, scratch, entries * sizeof(*scalars));
        state_space = (struct pippenger_state *) scratch_alloc(error_callback, scratch, sizeof(*state_space));
        if (points == NULL || scalars == NULL || state_space == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        state_space->ps = (struct pippenger_point_state *) scratch_alloc(error_callback, scratch, entries * sizeof(*state_space->ps));
        state_space->wnaf_na = (int *) scratch_alloc(error_callback, scratch, entries*(WNAF_SIZE(bucket_window+1)) * sizeof(int));
        buckets = (gej *) scratch_alloc(error_callback, scratch, (1<<bucket_window) * sizeof(*buckets));
        if (state_space->ps == NULL || state_space->wnaf_na == NULL || buckets == NULL) {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        if (inp_g_sc != NULL) {
            scalars[0] = *inp_g_sc;
            points[0] = ge_const_g;
            idx++;
            ecmult_endo_split(&scalars[0], &scalars[1], &points[0], &points[1]);
            idx++;
        }

        while (point_idx < n_points) {
            if (!cb(&scalars[idx], &points[idx], point_idx + cb_offset, cbdata)) {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            idx++;
            ecmult_endo_split(&scalars[idx - 1], &scalars[idx], &points[idx - 1], &points[idx]);
            idx++;
            point_idx++;
        }

        ecmult_pippenger_wnaf(buckets, bucket_window, state_space, r, scalars, points, idx);

        /* Clear data */
        for(i = 0; (size_t)i < idx; i++) {
            scalar_clear(&scalars[i]);
            state_space->ps[i].skew_na = 0;
            for(j = 0; j < WNAF_SIZE(bucket_window+1); j++) {
                state_space->wnaf_na[i * WNAF_SIZE(bucket_window+1) + j] = 0;
            }
        }
        for(i = 0; i < 1<<bucket_window; i++) {
            gej_clear(&buckets[i]);
        }
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        return 1;
        */
}
