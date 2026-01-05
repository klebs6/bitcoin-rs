// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_size.rs ]
crate::ix!();

pub fn context_preallocated_size(flags: u32) -> usize {
    let mut ret: usize = round_to_align!(core::mem::size_of::<Secp256k1Context>());
    /* A return value of 0 is reserved as an indicator for errors when we call this function internally. */
    verify_check!(ret != 0);

    if expect!((flags & FLAGS_TYPE_MASK) != FLAGS_TYPE_CONTEXT, 0) {
        callback_call(&*default_illegal_callback, b"Invalid flags\0".as_ptr());
        return 0;
    }

    if (flags & FLAGS_BIT_CONTEXT_SIGN) != 0 {
        ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
    }
    if (flags & FLAGS_BIT_CONTEXT_VERIFY) != 0 {
        ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
    }
    ret
}

#[cfg(test)]
mod context_preallocated_size_api_contract_suite {
    use super::*;

    #[traced_test]
    fn context_preallocated_size_returns_zero_for_invalid_flags() {
        let flags: u32 = 0;
        tracing::info!(flags, "calling context_preallocated_size with invalid flags");

        let size = context_preallocated_size(flags);
        tracing::debug!(size, "context_preallocated_size returned");
        assert_eq!(size, 0);
    }

    #[traced_test]
    fn context_preallocated_size_adds_expected_components_for_sign_and_verify_bits() {
        let base_flags = FLAGS_TYPE_CONTEXT;
        let sign_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN;
        let verify_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        let sign_verify_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;

        let base = context_preallocated_size(base_flags);
        let sign = context_preallocated_size(sign_flags);
        let verify = context_preallocated_size(verify_flags);
        let sign_verify = context_preallocated_size(sign_verify_flags);

        tracing::debug!(
            base,
            sign,
            verify,
            sign_verify,
            gen_add = ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE,
            verify_add = ECMULT_CONTEXT_PREALLOCATED_SIZE,
            "computed preallocated sizes"
        );

        assert!(base != 0);
        assert_eq!(sign, base + ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE);
        assert_eq!(verify, base + ECMULT_CONTEXT_PREALLOCATED_SIZE);
        assert_eq!(
            sign_verify,
            base + ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE + ECMULT_CONTEXT_PREALLOCATED_SIZE
        );
    }

    #[traced_test]
    fn context_preallocated_size_is_unchanged_by_declassify_flag() {
        let base_flags = FLAGS_TYPE_CONTEXT;
        let decl_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_DECLASSIFY;

        let base = context_preallocated_size(base_flags);
        let decl = context_preallocated_size(decl_flags);

        tracing::debug!(base, decl, "computed base and declassify sizes");
        assert_eq!(base, decl);

        let sign_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN;
        let sign_decl_flags = sign_flags | FLAGS_BIT_CONTEXT_DECLASSIFY;

        let sign = context_preallocated_size(sign_flags);
        let sign_decl = context_preallocated_size(sign_decl_flags);

        tracing::debug!(sign, sign_decl, "computed sign-only and sign+declassify sizes");
        assert_eq!(sign, sign_decl);

        let verify_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_VERIFY;
        let verify_decl_flags = verify_flags | FLAGS_BIT_CONTEXT_DECLASSIFY;

        let verify = context_preallocated_size(verify_flags);
        let verify_decl = context_preallocated_size(verify_decl_flags);

        tracing::debug!(verify, verify_decl, "computed verify-only and verify+declassify sizes");
        assert_eq!(verify, verify_decl);

        let sign_verify_flags = FLAGS_TYPE_CONTEXT | FLAGS_BIT_CONTEXT_SIGN | FLAGS_BIT_CONTEXT_VERIFY;
        let sign_verify_decl_flags = sign_verify_flags | FLAGS_BIT_CONTEXT_DECLASSIFY;

        let sign_verify = context_preallocated_size(sign_verify_flags);
        let sign_verify_decl = context_preallocated_size(sign_verify_decl_flags);

        tracing::debug!(
            sign_verify,
            sign_verify_decl,
            "computed sign+verify and sign+verify+declassify sizes"
        );
        assert_eq!(sign_verify, sign_verify_decl);
    }
}
