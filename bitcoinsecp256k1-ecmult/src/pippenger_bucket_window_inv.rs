// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_bucket_window_inv.rs ]
crate::ix!();

/// Returns the maximum optimal number of points for a bucket_window.
///
pub fn pippenger_bucket_window_inv(bucket_window: i32) -> usize {
    tracing::trace!(
        target: "secp256k1::ecmult",
        bucket_window = bucket_window,
        "pippenger_bucket_window_inv"
    );

    match bucket_window {
        1 => 1,
        2 => 4,
        3 => 20,
        4 => 57,
        5 => 136,
        6 => 235,
        7 => 1260,
        8 => 1260,
        9 => 4420,
        10 => 7880,
        11 => 16050,
        x if x == (PIPPENGER_MAX_BUCKET_WINDOW as i32) => usize::MAX,
        _ => 0,
    }
}

#[cfg(test)]
mod pippenger_bucket_window_inv_contract_suite {
    use super::*;

    #[traced_test]
    fn pippenger_bucket_window_inv_returns_expected_max_points() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_bucket_window_inv_returns_expected_max_points"
        );

        let max_bw: i32 = PIPPENGER_MAX_BUCKET_WINDOW as i32;
        let invalid_above_max: i32 = max_bw + 1;

        let cases: &[(i32, usize)] = &[
            (1, 1),
            (2, 4),
            (3, 20),
            (4, 57),
            (5, 136),
            (6, 235),
            (7, 1260),
            (8, 1260),
            (9, 4420),
            (10, 7880),
            (11, 16050),
            (max_bw, usize::MAX),
            (0, 0),
            (invalid_above_max, 0),
            (-1, 0),
        ];

        for (bw, expected) in cases {
            let got = pippenger_bucket_window_inv(*bw);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                bucket_window = *bw,
                got = got,
                expected = *expected,
                max_bucket_window = max_bw,
                "pippenger_bucket_window_inv(bucket_window)"
            );

            assert_eq!(got, *expected);
        }
    }

    #[traced_test]
    fn bucket_window_and_inverse_are_consistent_at_thresholds() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "bucket_window_and_inverse_are_consistent_at_thresholds"
        );

        let max_bw: i32 = PIPPENGER_MAX_BUCKET_WINDOW as i32;

        // These are the bucket windows actually returned by `pippenger_bucket_window` at the
        // documented thresholds (note that 8 is intentionally skipped by the forward mapping).
        let candidates: [i32; 11] = [1, 2, 3, 4, 5, 6, 7, 9, 10, 11, max_bw];

        for bw in candidates {
            if bw <= 0 || bw > max_bw {
                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    bw = bw,
                    max_bucket_window = max_bw,
                    "skipping non-roundtrippable bucket window candidate"
                );
                continue;
            }

            let max_n = pippenger_bucket_window_inv(bw);
            let got_bw = pippenger_bucket_window(max_n);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                bw = bw,
                max_n = max_n,
                got_bw = got_bw,
                "roundtrip consistency check"
            );

            assert_eq!(got_bw, bw);
        }

        // Bucket window 8 is an intentional alias of 7 in the inverse mapping.
        if 8 <= max_bw {
            let n7 = pippenger_bucket_window_inv(7);
            let n8 = pippenger_bucket_window_inv(8);
            let got_bw_from_n8 = pippenger_bucket_window(n8);

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                n7 = n7,
                n8 = n8,
                got_bw_from_n8 = got_bw_from_n8,
                "bucket_window_inv(8) alias behavior"
            );

            assert_eq!(n8, n7);
            assert_eq!(got_bw_from_n8, 7);
        }
    }
}
