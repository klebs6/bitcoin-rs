// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_bucket_window.rs ]
crate::ix!();

/// Returns optimal bucket_window (number of bits of a scalar represented by a set of buckets) for
/// a given number of points.
///
pub fn pippenger_bucket_window(n: usize) -> i32 {
    tracing::trace!(target: "secp256k1::ecmult", n = n, "pippenger_bucket_window");

    if n <= 1 {
        1
    } else if n <= 4 {
        2
    } else if n <= 20 {
        3
    } else if n <= 57 {
        4
    } else if n <= 136 {
        5
    } else if n <= 235 {
        6
    } else if n <= 1260 {
        7
    } else if n <= 4420 {
        9
    } else if n <= 7880 {
        10
    } else if n <= 16050 {
        11
    } else {
        PIPPENGER_MAX_BUCKET_WINDOW as i32
    }
}

#[cfg(test)]
mod pippenger_bucket_window_contract_suite {
    use super::*;

    #[traced_test]
    fn pippenger_bucket_window_returns_expected_values_at_boundaries() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "pippenger_bucket_window_returns_expected_values_at_boundaries"
        );

        let cases: &[(usize, i32)] = &[
            (0, 1),
            (1, 1),
            (2, 2),
            (4, 2),
            (5, 3),
            (20, 3),
            (21, 4),
            (57, 4),
            (58, 5),
            (136, 5),
            (137, 6),
            (235, 6),
            (236, 7),
            (1260, 7),
            (1261, 9),
            (4420, 9),
          (4421, 10),
            (7880, 10),
            (7881, 11),
            (16050, 11),
            (16051, PIPPENGER_MAX_BUCKET_WINDOW as i32),
            (1000000, PIPPENGER_MAX_BUCKET_WINDOW as i32),
        ];

        for (n, expected) in cases {
            let got = pippenger_bucket_window(*n);
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                n = *n,
                got = got,
                expected = *expected,
                "pippenger_bucket_window(n)"
            );
            assert_eq!(got, *expected);
        }
    }
}
