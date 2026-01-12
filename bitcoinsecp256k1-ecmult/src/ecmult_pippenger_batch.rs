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
    cb_offset:      usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n_points = n_points,
        cb_offset = cb_offset,
        inp_g_sc_is_null = inp_g_sc.is_null(),
        "ecmult_pippenger_batch"
    );

    unsafe {
        let scratch_checkpoint = scratch_checkpoint(error_callback, scratch);
        /* Use 2(n+1) with the endomorphism, when calculating batch
         * sizes. The reason for +1 is that we add the G scalar to the list of
         * other scalars. */
        let entries: usize = 2usize * n_points + 2usize;
        let mut points: *mut Ge;
        let mut scalars: *mut Scalar;
        let mut buckets: *mut Gej;
        let mut state_space: *mut PippengerState;
        let mut ps: *mut PippengerPointState;
        let mut wnaf_na: *mut i32;
        let mut idx: usize = 0;
        let mut point_idx: usize = 0;
        let mut i: i32;
        let mut j: i32;
        let bucket_window: i32;

        let _ = ctx;
        gej_set_infinity(r);
        if inp_g_sc.is_null() && n_points == 0 {
            return 1;
        }

        bucket_window = pippenger_bucket_window(n_points);
        points =
            scratch_alloc(error_callback, scratch, entries * core::mem::size_of::<Ge>()) as *mut Ge;
        scalars = scratch_alloc(error_callback, scratch, entries * core::mem::size_of::<Scalar>())
            as *mut Scalar;
        state_space = scratch_alloc(
            error_callback,
            scratch,
            core::mem::size_of::<PippengerState>(),
        ) as *mut PippengerState;
        if points.is_null() || scalars.is_null() || state_space.is_null() {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        ps = scratch_alloc(
            error_callback,
            scratch,
            entries * core::mem::size_of::<PippengerPointState>(),
        ) as *mut PippengerPointState;
        wnaf_na = scratch_alloc(
            error_callback,
            scratch,
            entries * wnaf_size!(bucket_window + 1) * core::mem::size_of::<i32>(),
        ) as *mut i32;
        buckets = scratch_alloc(
            error_callback,
            scratch,
            (1usize << (bucket_window as usize)) * core::mem::size_of::<Gej>(),
        ) as *mut Gej;

        if ps.is_null() || wnaf_na.is_null() || buckets.is_null() {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        PippengerState::write_ps(state_space, ps);
        PippengerState::write_wnaf_na(state_space, wnaf_na);

        if !inp_g_sc.is_null() {
            core::ptr::write(scalars.add(0), core::ptr::read(inp_g_sc));
            core::ptr::write(points.add(0), ge_const_g);
            idx += 1;
            ecmult_endo_split(scalars.add(0), scalars.add(1), points.add(0), points.add(1));
            idx += 1;
        }

        while point_idx < n_points {
            if cb(
                scalars.add(idx),
                points.add(idx),
                point_idx + cb_offset,
                cbdata,
            ) == 0
            {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            idx += 1;
            ecmult_endo_split(
                scalars.add(idx - 1),
                scalars.add(idx),
                points.add(idx - 1),
                points.add(idx),
            );
            idx += 1;
            point_idx += 1;
        }

        ecmult_pippenger_wnaf(buckets, bucket_window, state_space, r, scalars, points, idx);

        /* Clear data */
        i = 0;
        while (i as usize) < idx {
            scalar_clear(scalars.add(i as usize));
            PippengerPointState::write_skew_na(ps.add(i as usize), 0);

            j = 0;
            while (j as usize) < wnaf_size!(bucket_window + 1) {
                *wnaf_na.add((i as usize) * wnaf_size!(bucket_window + 1) + (j as usize)) = 0;
                j += 1;
            }
            i += 1;
        }

        i = 0;
        while i < (1i32 << bucket_window) {
            gej_clear(buckets.add(i as usize));
            i += 1;
        }

        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        1
    }
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

#[cfg(test)]
mod pippenger_batch_signature_contract_suite {
    use super::*;

    #[traced_test]
    fn pippenger_batch_matches_expected_batch_fn_signature() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_batch_matches_expected_batch_fn_signature"
        );

        type BatchFn = fn(
            error_callback: *const Callback,
            ctx: *const EcMultContext,
            scratch: *mut Scratch,
            r: *mut Gej,
            inp_g_sc: *const Scalar,
            cb: EcMultMultiCallback,
            cbdata: *mut c_void,
            n_points: usize,
            cb_offset: usize,
        ) -> i32;

        let f: BatchFn = ecmult_pippenger_batch;
        let _addr = f as usize;

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            f_addr = _addr,
            "captured ecmult_pippenger_batch fn pointer"
        );

        assert!(_addr != 0);
    }
}
