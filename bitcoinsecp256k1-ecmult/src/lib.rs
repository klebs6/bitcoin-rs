// ---------------- [ File: bitcoinsecp256k1-ecmult/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{config}
x!{ecmult}
x!{ecmult_context}
x!{ecmult_context_build}
x!{ecmult_context_clear}
x!{ecmult_context_finalize_memcpy}
x!{ecmult_context_init}
x!{ecmult_context_is_built}
x!{ecmult_endo_split}
x!{ecmult_multi_batch_size_helper}
x!{ecmult_multi_callback}
x!{ecmult_multi_func}
x!{ecmult_multi_simple_var}
x!{ecmult_multi_var}
x!{ecmult_odd_multiples_table}
x!{ecmult_odd_multiples_table_globalz_windowa}
x!{ecmult_odd_multiples_table_storage_var}
x!{ecmult_pippenger_batch}
x!{ecmult_pippenger_batch_single}
x!{ecmult_pippenger_wnaf}
x!{ecmult_strauss_batch}
x!{ecmult_strauss_batch_single}
x!{ecmult_strauss_wnaf}
x!{ecmult_table_get_ge}
x!{ecmult_wnaf}
x!{pippenger_bucket_window}
x!{pippenger_bucket_window_inv}
x!{pippenger_max_points}
x!{pippenger_point_state}
x!{pippenger_scratch_size}
x!{strauss_max_points}
x!{strauss_scratch_size}
x!{strauss_state}
x!{strauss_point_state}
x!{windows}
x!{wnaf}
x!{wnaf_fixed}

#[cfg(test)]
x!{ecmult_test_harness}

#[cfg(test)]
mod crate_root_public_surface_contract {
    use super::*;

    #[traced_test]
    fn crate_root_exports_key_symbols_and_macros() {
        tracing::info!(target: "secp256k1::ecmult::tests", "crate_root_exports_key_symbols_and_macros");

        let w = WINDOW_A;
        let ts = ecmult_table_size!(w);
        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            window_a = w,
            table_size = ts,
            "validated ecmult_table_size!(WINDOW_A)"
        );

        assert!(WINDOW_A >= 2);
        assert!(WINDOW_G >= 2);
        assert!(ts >= 1);

        let size = *ECMULT_CONTEXT_PREALLOCATED_SIZE;
        tracing::debug!(
            target: "secp256k1::ecmult::tests",
            prealloc_size = size,
            "validated ECMULT_CONTEXT_PREALLOCATED_SIZE is accessible"
        );
        assert!(size > 0);
    }
}
