// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_contex.rs ]
crate::ix!();

pub type EcMultGenContextPrec = [[GeStorage; ECMULT_GEN_PREC_G]; ECMULT_GEN_PREC_N];

/// For accelerating the computation of a*G:
///
/// To harden against timing attacks, use the following mechanism:
///
/// - Break up the multiplicand into groups of PREC_B bits, called n_0, n_1, n_2, ...,
/// n_(PREC_N-1).
///
/// - Compute sum(n_i * (PREC_G)^i * G + U_i, i=0 ... PREC_N-1), where:
///
///   - U_i = U * 2^i, for i=0 ... PREC_N-2
///
///   - U_i = U * (1-2^(PREC_N-1)), for i=PREC_N-1 where U is a point with no known corresponding
///   scalar. Note that sum(U_i, i=0 ... PREC_N-1) = 0.
///
/// For each i, and each of the PREC_G possible values of n_i, (n_i * (PREC_G)^i * G + U_i) is
/// precomputed (call it prec(i, n_i)). The formula now becomes sum(prec(i, n_i), i=0
/// ... PREC_N-1).
///
/// None of the resulting prec group elements have a known scalar, and neither do any of the
/// intermediate sums while computing a*G.
///
#[derive(Getters, CopyGetters, MutGetters, Setters)]
pub struct EcMultGenContext {

    /// prec[j][i] = (PREC_G)^j * i * G + U_i
    #[getset(get_copy="pub", set="pub")]
    prec:    *mut EcMultGenContextPrec,

    #[getset(get="pub", get_mut="pub")]
    blind:   Scalar,

    #[getset(get="pub", get_mut="pub")]
    initial: Gej,

}

impl EcMultGenContext {

    pub const fn new() -> Self {
        Self {
            prec:    core::ptr::null_mut(),
            blind:   Scalar::new(),
            initial: Gej::new(),
        }
    }
}

#[cfg(test)]
mod ecmult_gen_context_type_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_gen_context_new_starts_unbuilt_with_null_prec() {
        let ctx = EcMultGenContext::new();

        tracing::info!(
            prec_is_null = ctx.prec().is_null(),
            "validating default EcMultGenContext construction"
        );

        assert!(ctx.prec().is_null());
        assert_eq!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext), 0);
    }

    #[traced_test]
    fn ecmult_gen_context_prec_setter_round_trips_pointer() {
        let mut ctx = EcMultGenContext::new();

        let dummy: *mut EcMultGenContextPrec = 1usize as *mut EcMultGenContextPrec;
        ctx.set_prec(dummy);

        tracing::debug!(prec = (ctx.prec() as usize), "set prec to dummy pointer");
        assert_eq!(ctx.prec(), dummy);

        ctx.set_prec(core::ptr::null_mut());
        tracing::debug!("reset prec to null");
        assert!(ctx.prec().is_null());
    }

    #[traced_test]
    fn ecmult_gen_context_mut_getters_allow_in_place_updates() {
        let mut ctx = EcMultGenContext::new();

        unsafe {
            scalar_set_int(ctx.blind_mut(), 5);
            assert!(scalar_is_zero(ctx.blind() as *const Scalar) == 0);

            gej_set_ge(ctx.initial_mut(), &ge_const_g);
            assert!(gej_is_infinity(ctx.initial() as *const Gej) == 0);
        }

        tracing::info!("validated blind/initial mut accessors are operational");
    }

    #[traced_test]
    fn ecmult_gen_context_prec_type_has_expected_dimensions_and_layout() {
        let expected_entries = ECMULT_GEN_PREC_N
            .checked_mul(ECMULT_GEN_PREC_G)
            .unwrap();

        let expected_size = expected_entries
            .checked_mul(core::mem::size_of::<GeStorage>())
            .unwrap();

        let actual_size = core::mem::size_of::<EcMultGenContextPrec>();

        tracing::info!(
            expected_entries,
            expected_size,
            actual_size,
            "validating EcMultGenContextPrec layout"
        );

        assert_eq!(actual_size, expected_size);
        assert!(core::mem::align_of::<EcMultGenContextPrec>() >= core::mem::align_of::<GeStorage>());
    }
}
