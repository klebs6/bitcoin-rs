// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context_is_built.rs ]
crate::ix!();

pub fn ecmult_context_is_built(ctx: *const EcMultContext) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_context_is_built");

    unsafe { (!(*ctx).pre_g().is_null()) as i32 }
}

#[cfg(test)]
mod ecmult_context_is_built_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_context_is_built_reports_unbuilt_then_built() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_is_built_reports_unbuilt_then_built"
        );

        unsafe {
            let mut ctx = EcMultContext::new();

            let before = ecmult_context_is_built(core::ptr::addr_of!(ctx));
            tracing::debug!(target: "secp256k1::ecmult::tests", before = before, "before build");
            assert_eq!(before, 0);

            let (_buf, layout, ctx_ptr, _cursor_end, _ctx_offset) = alloc_and_build_ecmult_context_preallocated();
            let after = ecmult_context_is_built(ctx_ptr);
            tracing::debug!(target: "secp256k1::ecmult::tests", after = after, "after build");
            assert_eq!(after, 1);

            dealloc_aligned(_buf, layout);
        }
    }
}
