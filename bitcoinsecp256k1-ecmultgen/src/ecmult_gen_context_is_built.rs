// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_context_is_built.rs ]
crate::ix!();

pub fn ecmult_gen_context_is_built(ctx: *const EcMultGenContext) -> i32 {
    unsafe { (!(*ctx).prec().is_null()) as i32 }
}

#[cfg(test)]
mod ecmult_gen_context_is_built_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_gen_context_is_built_reflects_prec_pointer_state() {
        unsafe {
            let mut ctx = EcMultGenContext::new();
            ecmult_gen_context_init(&mut ctx);

            tracing::debug!("checking unbuilt state");
            assert_eq!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext), 0);

            let dummy: *mut EcMultGenContextPrec = 1usize as *mut EcMultGenContextPrec;
            ctx.set_prec(dummy);

            tracing::debug!("checking built state after setting non-null prec pointer");
            assert_eq!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext), 1);

            ctx.set_prec(core::ptr::null_mut());

            tracing::debug!("checking unbuilt state after resetting to null");
            assert_eq!(ecmult_gen_context_is_built((&ctx) as *const EcMultGenContext), 0);
        }
    }
}
