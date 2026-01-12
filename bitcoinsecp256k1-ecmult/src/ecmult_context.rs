// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_context.rs ]
crate::ix!();

/// For accelerating the computation of a*P + b*G:
///
/// NOTE: these vectors were slices before we wanted a const constructor.
///
/// perhaps this will cause problems... perhaps not
///
#[repr(C)]
#[derive(Getters,Setters,MutGetters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct EcMultContext {
    /// odd multiples of the generator
    pre_g:     *mut GeStorage,

    /// odd multiples of 2^128*generator
    pre_g_128: *mut GeStorage,
}

impl EcMultContext {
    pub const fn new() -> Self {
        Self {
            pre_g: core::ptr::null_mut(),
            pre_g_128: core::ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod ecmult_context_initial_state_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_context_new_starts_with_null_precomputed_tables() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_new_starts_with_null_precomputed_tables"
        );

        let ctx = EcMultContext::new();

        let pre_g_is_null = (*ctx.pre_g()).is_null();
        let pre_g_128_is_null = (*ctx.pre_g_128()).is_null();

        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            pre_g_is_null = pre_g_is_null,
            pre_g_128_is_null = pre_g_128_is_null,
            "EcMultContext::new() pointers"
        );

        assert!(pre_g_is_null);
        assert!(pre_g_128_is_null);
    }
}

#[cfg(test)]
mod ecmult_context_init_contract_suite {
    use super::*;

    #[traced_test]
    fn ecmult_context_init_clears_precomputed_table_pointers() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_context_init_clears_precomputed_table_pointers"
        );

        unsafe {
            let mut ctx = EcMultContext::new();

            ctx.set_pre_g(1usize as *mut GeStorage);
            ctx.set_pre_g_128(2usize as *mut GeStorage);

            assert!(!(*ctx.pre_g()).is_null());
            assert!(!(*ctx.pre_g_128()).is_null());

            ecmult_context_init(core::ptr::addr_of_mut!(ctx));

            assert!((*ctx.pre_g()).is_null());
            assert!((*ctx.pre_g_128()).is_null());
        }
    }
}
