// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_multi_var.rs ]
crate::ix!();

/// Multi-multiply: R = inp_g_sc * G + sum_i ni * Ai.
///
/// Chooses the right algorithm for a given number of points and scratch space size.
///
/// Resets and overwrites the given scratch space. If the points do not fit in the scratch space
/// the algorithm is repeatedly run with batches of points.
///
/// If no scratch space is given then a simple algorithm is used that simply multiplies the points
/// with the corresponding scalars and adds them up.
///
/// Returns:
///
/// 1 on success (including when inp_g_sc is NULL and n is 0)
///
/// 0 if there is not enough scratch space for a single point or callback returns 0
///
pub fn ecmult_multi_var(
    error_callback: *const Callback,
    ctx:            *const EcMultContext,
    scratch:        *mut Scratch,
    r:              *mut Gej,
    inp_g_sc:       *const Scalar,
    cb:             EcMultMultiCallback,
    cbdata:         *mut c_void,
    n:              usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n = n,
        scratch_is_null = scratch.is_null(),
        inp_g_sc_is_null = inp_g_sc.is_null(),
        "ecmult_multi_var"
    );

    unsafe {
        let mut i: usize;

        type BatchFn = fn(
            error_callback: *const Callback,
            ctx:            *const EcMultContext,
            scratch:        *mut Scratch,
            r:              *mut Gej,
            inp_g_sc:       *const Scalar,
            cb:             EcMultMultiCallback,
            cbdata:         *mut c_void,
            n_points:       usize,
            cb_offset:      usize,
        ) -> i32;

        let mut f: BatchFn;
        let mut n_batches: usize;
        let mut n_batch_points: usize;
        let mut n: usize = n;

        gej_set_infinity(r);
        if inp_g_sc.is_null() && n == 0 {
            return 1;
        } else if n == 0 {
            let mut szero = Scalar::new();
            scalar_set_int(core::ptr::addr_of_mut!(szero), 0);
            ecmult(ctx, r, r, core::ptr::addr_of!(szero), inp_g_sc);
            return 1;
        }
        if scratch.is_null() {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }

        /* Compute the batch sizes for Pippenger's algorithm given a scratch space. If it's greater than
         * a threshold use Pippenger's algorithm. Otherwise use Strauss' algorithm.
         * As a first step check if there's enough space for Pippenger's algo (which requires less space
         * than Strauss' algo) and if not, use the simple algorithm. */
        if ecmult_multi_batch_size_helper(
            core::ptr::addr_of_mut!(n_batches),
            core::ptr::addr_of_mut!(n_batch_points),
            pippenger_max_points(error_callback, scratch),
            n,
        ) == 0
        {
            tracing::warn!(
                target: "secp256k1::ecmult",
                "ecmult_multi_var: insufficient scratch for pippenger; falling back to simple"
            );
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }
        if n_batch_points >= ECMULT_PIPPENGER_THRESHOLD {
            f = ecmult_pippenger_batch;
        } else {
            if ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                strauss_max_points(error_callback, scratch),
                n,
            ) == 0
            {
                tracing::warn!(
                    target: "secp256k1::ecmult",
                    "ecmult_multi_var: insufficient scratch for strauss; falling back to simple"
                );
                return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
            }
            f = ecmult_strauss_batch;
        }

        tracing::debug!(
            target: "secp256k1::ecmult",
            n_batches = n_batches,
            n_batch_points = n_batch_points,
            pippenger_threshold = ECMULT_PIPPENGER_THRESHOLD,
            "ecmult_multi_var: batching decision"
        );

        i = 0;
        while i < n_batches {
            let nbp: usize = if n < n_batch_points { n } else { n_batch_points };
            let offset: usize = n_batch_points * i;
            let mut tmp = Gej::new();

            if f(
                error_callback,
                ctx,
                scratch,
                core::ptr::addr_of_mut!(tmp),
                if i == 0 { inp_g_sc } else { core::ptr::null() },
                cb,
                cbdata,
                nbp,
                offset,
            ) == 0
            {
                return 0;
            }
            gej_add_var(r, r, core::ptr::addr_of!(tmp), core::ptr::null_mut());
            n -= nbp;

            i += 1;
        }
        1
    }

        /*
        size_t i;

        int (*f)(const callback* error_callback, const ecmult_context*, scratch*, gej*, const scalar*, ecmult_multi_callback cb, c_void*, size_t, size_t);
        size_t n_batches;
        size_t n_batch_points;

        gej_set_infinity(r);
        if (inp_g_sc == NULL && n == 0) {
            return 1;
        } else if (n == 0) {
            scalar szero;
            scalar_set_int(&szero, 0);
            ecmult(ctx, r, r, &szero, inp_g_sc);
            return 1;
        }
        if (scratch == NULL) {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }

        /* Compute the batch sizes for Pippenger's algorithm given a scratch space. If it's greater than
         * a threshold use Pippenger's algorithm. Otherwise use Strauss' algorithm.
         * As a first step check if there's enough space for Pippenger's algo (which requires less space
         * than Strauss' algo) and if not, use the simple algorithm. */
        if (!ecmult_multi_batch_size_helper(&n_batches, &n_batch_points, pippenger_max_points(error_callback, scratch), n)) {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }
        if (n_batch_points >= ECMULT_PIPPENGER_THRESHOLD) {
            f = ecmult_pippenger_batch;
        } else {
            if (!ecmult_multi_batch_size_helper(&n_batches, &n_batch_points, strauss_max_points(error_callback, scratch), n)) {
                return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
            }
            f = ecmult_strauss_batch;
        }
        for(i = 0; i < n_batches; i++) {
            size_t nbp = n < n_batch_points ? n : n_batch_points;
            size_t offset = n_batch_points*i;
            gej tmp;
            if (!f(error_callback, ctx, scratch, &tmp, i == 0 ? inp_g_sc : NULL, cb, cbdata, nbp, offset)) {
                return 0;
            }
            gej_add_var(r, r, &tmp, NULL);
            n -= nbp;
        }
        return 1;
        */

}

pub fn ecmult_multi_var(
    error_callback: *const Callback,
    ctx:            *const EcMultContext,
    scratch:        *mut Scratch,
    r:              *mut Gej,
    inp_g_sc:       *const Scalar,
    cb:             EcMultMultiCallback,
    cbdata:         *mut c_void,
    n:              usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n = n,
        scratch_is_null = scratch.is_null(),
        inp_g_sc_is_null = inp_g_sc.is_null(),
        "ecmult_multi_var"
    );

    unsafe {
        let mut i: usize;

        type BatchFn = fn(
            error_callback: *const Callback,
            ctx:            *const EcMultContext,
            scratch:        *mut Scratch,
            r:              *mut Gej,
            inp_g_sc:       *const Scalar,
            cb:             EcMultMultiCallback,
            cbdata:         *mut c_void,
            n_points:       usize,
            cb_offset:      usize,
        ) -> i32;

        let mut f: BatchFn;
        let mut n_batches: usize = 0;
        let mut n_batch_points: usize = 0;
        let mut n: usize = n;

        gej_set_infinity(r);
        if inp_g_sc.is_null() && n == 0 {
            return 1;
        } else if n == 0 {
            let mut szero = Scalar::new();
            scalar_set_int(core::ptr::addr_of_mut!(szero), 0);
            ecmult(ctx, r, r, core::ptr::addr_of!(szero), inp_g_sc);
            return 1;
        }
        if scratch.is_null() {
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }

        /* Compute the batch sizes for Pippenger's algorithm given a scratch space. If it's greater than
         * a threshold use Pippenger's algorithm. Otherwise use Strauss' algorithm.
         * As a first step check if there's enough space for Pippenger's algo (which requires less space
         * than Strauss' algo) and if not, use the simple algorithm. */
        if ecmult_multi_batch_size_helper(
            core::ptr::addr_of_mut!(n_batches),
            core::ptr::addr_of_mut!(n_batch_points),
            pippenger_max_points(error_callback, scratch),
            n,
        ) == 0
        {
            tracing::warn!(
                target: "secp256k1::ecmult",
                "ecmult_multi_var: insufficient scratch for pippenger; falling back to simple"
            );
            return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
        }
        if n_batch_points >= ECMULT_PIPPENGER_THRESHOLD {
            f = ecmult_pippenger_batch;
        } else {
            if ecmult_multi_batch_size_helper(
                core::ptr::addr_of_mut!(n_batches),
                core::ptr::addr_of_mut!(n_batch_points),
                strauss_max_points(error_callback, scratch),
                n,
            ) == 0
            {
                tracing::warn!(
                    target: "secp256k1::ecmult",
                    "ecmult_multi_var: insufficient scratch for strauss; falling back to simple"
                );
                return ecmult_multi_simple_var(ctx, r, inp_g_sc, cb, cbdata, n);
            }
            f = ecmult_strauss_batch;
        }

        tracing::debug!(
            target: "secp256k1::ecmult",
            n_batches = n_batches,
            n_batch_points = n_batch_points,
            pippenger_threshold = ECMULT_PIPPENGER_THRESHOLD,
            "ecmult_multi_var: batching decision"
        );

        i = 0;
        while i < n_batches {
            let nbp: usize = if n < n_batch_points { n } else { n_batch_points };
            let offset: usize = n_batch_points * i;
            let mut tmp = Gej::new();

            if f(
                error_callback,
                ctx,
                scratch,
                core::ptr::addr_of_mut!(tmp),
                if i == 0 { inp_g_sc } else { core::ptr::null() },
                cb,
                cbdata,
                nbp,
                offset,
            ) == 0
            {
                return 0;
            }
            gej_add_var(r, r, core::ptr::addr_of!(tmp), core::ptr::null_mut());
            n -= nbp;

            i += 1;
        }
        1
    }
}
