// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult.rs ]
crate::ix!();

pub fn ecmult(
    ctx: *const EcMultContext,
    r:   *mut Gej,
    a:   *const Gej,
    na:  *const Scalar,
    ng:  *const Scalar,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult");

    unsafe {
        let mut prej: [Gej; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Gej; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut zr: [Fe; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Fe; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut pre_a: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut ps: [StraussPointState; 1] =
            core::mem::MaybeUninit::<[StraussPointState; 1]>::uninit().assume_init();
        let mut pre_a_lam: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();

        let mut state = StraussState::new();
        state.set_prej(prej.as_mut_ptr());
        state.set_zr(zr.as_mut_ptr());
        state.set_pre_a(pre_a.as_mut_ptr());
        state.set_pre_a_lam(pre_a_lam.as_mut_ptr());
        state.set_ps(ps.as_mut_ptr());

        ecmult_strauss_wnaf(ctx, core::ptr::addr_of!(state), r, 1usize, a, na, ng);
    }
}

/// Double multiply: R = na*A + ng*G
///
pub fn ecmult(
    ctx: *const EcMultContext,
    r:   *mut Gej,
    a:   *const Gej,
    na:  *const Scalar,
    ng:  *const Scalar,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult");

    unsafe {
        let mut prej: [Gej; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Gej; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut zr: [Fe; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Fe; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut pre_a: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut ps: [StraussPointState; 1] =
            core::mem::MaybeUninit::<[StraussPointState; 1]>::uninit().assume_init();
        let mut pre_a_lam: [Ge; ecmult_table_size!(WINDOW_A)] =
            core::mem::MaybeUninit::<[Ge; ecmult_table_size!(WINDOW_A)]>::uninit().assume_init();
        let mut state = StraussState {
            prej: prej.as_mut_ptr(),
            zr: zr.as_mut_ptr(),
            pre_a: pre_a.as_mut_ptr(),
            pre_a_lam: pre_a_lam.as_mut_ptr(),
            ps: ps.as_mut_ptr(),
        };

        ecmult_strauss_wnaf(ctx, core::ptr::addr_of!(state), r, 1usize, a, na, ng);
    }
        /*
            gej prej[ECMULT_TABLE_SIZE(WINDOW_A)];
        fe zr[ECMULT_TABLE_SIZE(WINDOW_A)];
        ge pre_a[ECMULT_TABLE_SIZE(WINDOW_A)];
        struct strauss_point_state ps[1];
        ge pre_a_lam[ECMULT_TABLE_SIZE(WINDOW_A)];
        struct strauss_state state;

        state.prej = prej;
        state.zr = zr;
        state.pre_a = pre_a;
        state.pre_a_lam = pre_a_lam;
        state.ps = ps;
        ecmult_strauss_wnaf(ctx, &state, r, 1, a, na, ng);
        */
}
