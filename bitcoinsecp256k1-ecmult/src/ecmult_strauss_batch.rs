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
    cb_offset:      usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n_points = n_points,
        cb_offset = cb_offset,
        inp_g_sc_is_null = inp_g_sc.is_null(),
        "ecmult_strauss_batch"
    );

    unsafe {
        let mut points: *mut Gej;
        let mut scalars: *mut Scalar;
        let mut state = StraussState {
            prej: core::ptr::null_mut(),
            zr: core::ptr::null_mut(),
            pre_a: core::ptr::null_mut(),
            pre_a_lam: core::ptr::null_mut(),
            ps: core::ptr::null_mut(),
        };
        let mut i: usize;
        let scratch_checkpoint = scratch_checkpoint(error_callback, scratch);

        gej_set_infinity(r);
        if inp_g_sc.is_null() && n_points == 0 {
            return 1;
        }

        points = scratch_alloc(error_callback, scratch, n_points * core::mem::size_of::<Gej>()) as *mut Gej;
        scalars = scratch_alloc(error_callback, scratch, n_points * core::mem::size_of::<Scalar>()) as *mut Scalar;
        state.prej = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Gej>(),
        ) as *mut Gej;
        state.zr = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Fe>(),
        ) as *mut Fe;
        state.pre_a = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Ge>(),
        ) as *mut Ge;
        state.pre_a_lam = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Ge>(),
        ) as *mut Ge;
        state.ps = scratch_alloc(
            error_callback,
            scratch,
            n_points * core::mem::size_of::<StraussPointState>(),
        ) as *mut StraussPointState;

        if points.is_null()
            || scalars.is_null()
            || state.prej.is_null()
            || state.zr.is_null()
            || state.pre_a.is_null()
            || state.pre_a_lam.is_null()
            || state.ps.is_null()
        {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        i = 0;
        while i < n_points {
            let mut point = Ge::new();
            if cb(scalars.add(i), core::ptr::addr_of_mut!(point), i + cb_offset, cbdata) == 0 {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            gej_set_ge(points.add(i), core::ptr::addr_of!(point));
            i += 1;
        }
        ecmult_strauss_wnaf(ctx, core::ptr::addr_of!(state), r, n_points, points, scalars, inp_g_sc);
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        1
    }

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

pub fn ecmult_strauss_batch(
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
        "ecmult_strauss_batch"
    );

    unsafe {
        let mut points: *mut Gej;
        let mut scalars: *mut Scalar;

        let mut state = StraussState::new();

        let mut prej_ptr: *mut Gej;
        let mut zr_ptr: *mut Fe;
        let mut pre_a_ptr: *mut Ge;
        let mut pre_a_lam_ptr: *mut Ge;
        let mut ps_ptr: *mut StraussPointState;

        let mut i: usize;
        let scratch_checkpoint = scratch_checkpoint(error_callback, scratch);

        gej_set_infinity(r);
        if inp_g_sc.is_null() && n_points == 0 {
            return 1;
        }

        points = scratch_alloc(error_callback, scratch, n_points * core::mem::size_of::<Gej>())
            as *mut Gej;
        scalars = scratch_alloc(error_callback, scratch, n_points * core::mem::size_of::<Scalar>())
            as *mut Scalar;

        prej_ptr = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Gej>(),
        ) as *mut Gej;
        zr_ptr = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Fe>(),
        ) as *mut Fe;
        pre_a_ptr = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Ge>(),
        ) as *mut Ge;
        pre_a_lam_ptr = scratch_alloc(
            error_callback,
            scratch,
            n_points * ecmult_table_size!(WINDOW_A) * core::mem::size_of::<Ge>(),
        ) as *mut Ge;
        ps_ptr = scratch_alloc(
            error_callback,
            scratch,
            n_points * core::mem::size_of::<StraussPointState>(),
        ) as *mut StraussPointState;

        state.set_prej(prej_ptr);
        state.set_zr(zr_ptr);
        state.set_pre_a(pre_a_ptr);
        state.set_pre_a_lam(pre_a_lam_ptr);
        state.set_ps(ps_ptr);

        if points.is_null()
            || scalars.is_null()
            || prej_ptr.is_null()
            || zr_ptr.is_null()
            || pre_a_ptr.is_null()
            || pre_a_lam_ptr.is_null()
            || ps_ptr.is_null()
        {
            scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
            return 0;
        }

        i = 0;
        while i < n_points {
            let mut point = Ge::new();
            if cb(scalars.add(i), core::ptr::addr_of_mut!(point), i + cb_offset, cbdata) == 0 {
                scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
                return 0;
            }
            gej_set_ge(points.add(i), core::ptr::addr_of!(point));
            i += 1;
        }

        ecmult_strauss_wnaf(
            ctx,
            core::ptr::addr_of!(state),
            r,
            n_points,
            points,
            scalars,
            inp_g_sc,
        );
        scratch_apply_checkpoint(error_callback, scratch, scratch_checkpoint);
        1
    }
}
