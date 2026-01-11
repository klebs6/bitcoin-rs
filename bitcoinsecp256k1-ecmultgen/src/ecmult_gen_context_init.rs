// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_init.rs ]
crate::ix!();

pub fn ecmult_gen_context_init(ctx: *mut EcMultGenContext)  {
    unsafe {
        (*ctx).set_prec(core::ptr::null_mut());
    }
}

#[cfg(test)]
mod ecmult_gen_context_init_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_gen_context_init_sets_prec_to_null_even_if_previously_non_null() {
        unsafe {
            let mut ctx = EcMultGenContext::new();

            let dummy: *mut EcMultGenContextPrec = 1usize as *mut EcMultGenContextPrec;
            ctx.set_prec(dummy);
            assert_eq!(ctx.prec(), dummy);

            ecmult_gen_context_init(&mut ctx);

            tracing::info!(prec_is_null = ctx.prec().is_null(), "init applied");
            assert!(ctx.prec().is_null());

            ecmult_gen_context_init(&mut ctx);
            assert!(ctx.prec().is_null());
        }
    }
}
