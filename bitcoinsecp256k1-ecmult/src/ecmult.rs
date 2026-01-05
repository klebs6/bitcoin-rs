// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult.rs ]
crate::ix!();

/// Double multiply: R = na*A + ng*G
///
pub fn ecmult(
        ctx: *const EcMultContext,
        r:   *mut Gej,
        a:   *const Gej,
        na:  *const Scalar,
        ng:  *const Scalar)  {
    
    todo!();
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
