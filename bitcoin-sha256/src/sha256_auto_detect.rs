crate::ix!();

/**
  | Select the fastest SHA‑256 backend supported by the current
  | CPU **at runtime** and initialise global function pointers so
  | that subsequent calls transparently dispatch to that backend.
  |
  | *Returns* the backend description string (e.g.  
  | `"standard"`, `"sse4(1way)"`, `"shani(1way,2way)"`, …).
  |
  | The routine is *idempotent*: the detection logic runs exactly
  | once per process.  Re‑invocations cheaply return the cached
  | backend label.
  */
#[inline]
pub fn sha256auto_detect() -> String {

    static BACKEND: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    BACKEND
        .get_or_init(|| {
            // ----------------------------------------------------------------
            // 1.  Detect ISA extensions
            // ----------------------------------------------------------------
            #[cfg(target_arch = "x86_64")]
            let (have_sse41, have_avx, have_avx2, have_shani) = {
                (
                    std::arch::is_x86_feature_detected!("sse4.1"),
                    std::arch::is_x86_feature_detected!("avx"),
                    std::arch::is_x86_feature_detected!("avx2"),
                    std::arch::is_x86_feature_detected!("sha"),
                )
            };

            #[cfg(not(target_arch = "x86_64"))]
            let (have_sse41, have_avx, have_avx2, have_shani) =
                (false, false, false, false);

            trace!(
                target: "sha256",
                have_sse41,
                have_avx,
                have_avx2,
                have_shani,
                "CPU feature probe completed"
            );

            // ----------------------------------------------------------------
            // 2.  Bind global function pointers
            // ----------------------------------------------------------------
            let mut label = "standard".to_string();

            unsafe {
                if have_shani {
                    // Intel SHA‑NI back‑end (1‑way + 2‑way double‑SHA)
                    TRANSFORM           = bitcoin_sha256_shani::sha256_shani_transform;
                    TRANSFORM_D64       = transform_d64_stub; // no dedicated 1‑way impl yet
                    TRANSFORM_D64_2WAY  =
                        Some(bitcoin_sha256_shani::sha256d64_shani_transform_2way);
                    label = "shani(1way,2way)".into();
                } else if have_sse41 {
                    // SSE4.1 scalar 1‑way back‑end
                    TRANSFORM     = bitcoin_sha256_sse4::sha256_sse4_transform;
                    TRANSFORM_D64 = transform_d64_stub; // placeholder
                    label         = "sse4(1way)".into();

                    // Optional 4‑way SSE4.1 double‑SHA
                    TRANSFORM_D64_4WAY =
                        Some(bitcoin_sha256_sse41::sha256d64_sse41_transform_4way);
                    label.push_str(",sse41(4way)");

                    // Optional 8‑way AVX2 double‑SHA
                    if have_avx && have_avx2 {
                        TRANSFORM_D64_8WAY =
                            Some(bitcoin_sha256_avx2::sha256d64_avx2_transform_8way);
                        label.push_str(",avx2(8way)");
                    }
                }
            }

            // ----------------------------------------------------------------
            // 3.  Sanity self‑test
            // ----------------------------------------------------------------
            assert!(
                crate::self_test(),
                "sha256auto_detect: internal SELF‑TEST failed after \
                 selecting backend `{}`",
                label
            );

            info!(
                target: "sha256",
                backend = %label,
                "SHA‑256 backend selected"
            );

            label
        })
        .clone()
}

#[cfg(test)]
mod autodetect_tests {
    use super::*;

    #[traced_test]
    fn autodetect_produces_valid_backend_label_and_passes_selftest() {
        let backend = sha256auto_detect();
        assert!(
            !backend.is_empty(),
            "backend label must not be empty"
        );
        assert!(
            self_test(),
            "self‑test must succeed after backend selection"
        );
    }
}
