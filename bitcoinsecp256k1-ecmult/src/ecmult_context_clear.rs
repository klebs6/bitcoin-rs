// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_clear.rs ]
crate::ix!();

pub fn ecmult_context_clear(ctx: *mut EcMultContext) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_clear");

    unsafe {
        ecmult_context_init(ctx);
    }
}

#[cfg(test)]
mod ecmult_context_clear_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_context_clear_is_idempotent_and_nulls_pointers() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_clear_is_idempotent_and_nulls_pointers"
        );

        unsafe {
            let mut ctx = EcMultContext::new();

            ctx.set_pre_g(1usize as *mut GeStorage);
            ctx.set_pre_g_128(2usize as *mut GeStorage);

            ecmult_context_clear(core::ptr::addr_of_mut!(ctx));
            assert!((*ctx.pre_g()).is_null());
            assert!((*ctx.pre_g_128()).is_null());

            ecmult_context_clear(core::ptr::addr_of_mut!(ctx));
            assert!((*ctx.pre_g()).is_null());
            assert!((*ctx.pre_g_128()).is_null());
        }
    }
}
