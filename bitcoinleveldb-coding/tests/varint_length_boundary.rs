// ---------------- [ File: bitcoinleveldb-coding/tests/varint_length_boundary.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn varint_length_respects_power_of_two_boundaries() {
    info!("varint_length_respects_power_of_two_boundaries: start");

    let cases: &[(u64, i32)] = &[
        (0, 1),
        (127, 1),
        (128, 2),
        (16_383, 2),      // 2^14 - 1
        (16_384, 3),      // 2^14
        (2_097_151, 3),   // 2^21 - 1
        (2_097_152, 4),   // 2^21
        (268_435_455, 4), // 2^28 - 1
        (268_435_456, 5), // 2^28
    ];

    for &(value, expected_len) in cases {
        let actual = varint_length(value);
        assert_eq!(
            expected_len, actual,
            "varint_length_respects_power_of_two_boundaries: mismatch for {}",
            value
        );
    }

    info!("varint_length_respects_power_of_two_boundaries: success");
}
