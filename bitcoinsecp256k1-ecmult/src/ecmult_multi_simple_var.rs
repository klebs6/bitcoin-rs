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
    n_points: usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        n_points = n_points,
        "ecmult_multi_simple_var"
    );

    unsafe {
        let mut point_idx: usize;
        let mut szero = Scalar::new();
        let mut tmpj = Gej::new();

        scalar_set_int(core::ptr::addr_of_mut!(szero), 0);
        gej_set_infinity(r);
        gej_set_infinity(core::ptr::addr_of_mut!(tmpj));
        /* r = inp_g_sc*G */
        ecmult(
            ctx,
            r,
            core::ptr::addr_of!(tmpj),
            core::ptr::addr_of!(szero),
            inp_g_sc,
        );
        point_idx = 0;
        while point_idx < n_points {
            let mut point = Ge::new();
            let mut pointj = Gej::new();
            let mut scalar = Scalar::new();

            if cb(core::ptr::addr_of_mut!(scalar), core::ptr::addr_of_mut!(point), point_idx, cbdata)
                == 0
            {
                return 0;
            }
            /* r += scalar*point */
            gej_set_ge(core::ptr::addr_of_mut!(pointj), core::ptr::addr_of!(point));
            ecmult(
                ctx,
                core::ptr::addr_of_mut!(tmpj),
                core::ptr::addr_of!(pointj),
                core::ptr::addr_of!(scalar),
                core::ptr::null(),
            );
            gej_add_var(r, r, core::ptr::addr_of!(tmpj), core::ptr::null_mut());

            point_idx += 1;
        }
        1
    }
}
