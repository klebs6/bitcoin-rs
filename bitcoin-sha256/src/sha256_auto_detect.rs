// ---------------- [ File: bitcoin-sha256/src/sha256_auto_detect.rs ]
crate::ix!();

/// Autodetect and configure the best available SHA‑256 backend.
///
/// Returns a human‑readable description mirroring Bitcoin Core’s string.
pub fn sha256auto_detect() -> String {
    unsafe {
        /* ------------------------------------------------------------------
         * Default: portable scalar reference implementation.
         * ---------------------------------------------------------------- */
        TRANSFORM            = sha256_transform_wrapper;
        TRANSFORM_D64        = transform_d64_scalar;
        TRANSFORM_D64_2WAY   = None;
        TRANSFORM_D64_4WAY   = None;
        TRANSFORM_D64_8WAY   = None;
        let mut desc = String::from("standard");

        /* ------------------------------------------------------------------
         * x86‑64 feature probing (mirrors Core ≈1‑for‑1).
         * ---------------------------------------------------------------- */
        #[cfg(target_arch = "x86_64")]
        {
            use std::arch::is_x86_feature_detected;

            // ---- SHA‑NI pathway (highest priority) ------------------------
            #[cfg(feature = "enable-shani")]
            if is_x86_feature_detected!("sha") {
                TRANSFORM          = sha256_shani::Transform;
                TRANSFORM_D64      = sha256d64_shani::Transform_2way;
                TRANSFORM_D64_2WAY = Some(sha256d64_shani::Transform_2way);
                desc = "shani(1way,2way)".into();
                trace!(target: "sha256", "backend: {}", desc);
                assert!(self_test(), "SHA‑NI backend failed self‑test");
                return desc;
            }

            // ---- SSE4 / SSE4.1 -------------------------------------------
            //
            // Only compile this block when the specialised SSE4 crate has
            // been ported and the symbols exist.  Until then the scalar
            // fallback is kept.
            #[cfg(all(feature = "enable-sse4", feature = "sse4-port-complete"))]
            {
                if is_x86_feature_detected!("sse4.1") {
                    TRANSFORM     = bitcoin_sha256_sse4::Transform;
                    TRANSFORM_D64 = bitcoin_sha256_sse4::transform_d64_sse4;

                    desc = "sse4(1way)".into();
                    #[cfg(feature = "enable-sse41")]
                    {
                        TRANSFORM_D64_4WAY = Some(sha256d64_sse41::Transform_4way);
                        desc.push_str(",sse41(4way)");
                    }
                    trace!(target: "sha256", "backend: {}", desc);
                    assert!(self_test(), "SSE4 backend failed self‑test");
                    return desc;
                }
            }

            // ---- AVX2 eight‑way ------------------------------------------
            #[cfg(feature = "enable-avx2")]
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("avx") {
                TRANSFORM_D64_8WAY = Some(sha256d64_avx2::Transform_8way);
                desc = "avx2(8way)".into();
                trace!(target: "sha256", "backend: {}", desc);
                assert!(self_test(), "AVX2 backend failed self‑test");
                return desc;
            }
        }

        /* ------------------------------------------------------------------
         * Fallback path was already initialised above.
         * ---------------------------------------------------------------- */
        trace!(target: "sha256", "backend: {}", desc);
        assert!(self_test(), "scalar backend failed self‑test");
        desc
    }
}
