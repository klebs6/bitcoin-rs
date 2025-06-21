// ---------------- [ File: bitcoin-string/src/process_mantissa_digit.rs ]
crate::ix!();

/** 
 | Upper bound for mantissa.
 |
 | 10^18-1 is the largest arbitrary decimal that
 | will fit in a signed 64-bit integer.
 |
 | Larger integers cannot consist of arbitrary
 | combinations of 0-9:
 |
 |   999999999999999999  1^18-1
 |  9223372036854775807  (1<<63)-1  (max int64_t)
 |  9999999999999999999  1^19-1     (would overflow)
 */
pub const UPPER_BOUND: i64 = 1000000000000000000 - 1;

/**
  | Helper function for ParseFixedPoint
  |
  */
#[inline]
pub fn process_mantissa_digit(
    ch: u8,
    mantissa: &mut i64,
    mantissa_tzeros: &mut i32,
) -> bool {
    tracing::trace!(
        target: "parse_fixed_point",
        ch,
        mantissa_before = *mantissa,
        mantissa_tzeros_before = *mantissa_tzeros,
        "processing mantissa digit",
    );

    if ch == b'0' {
        *mantissa_tzeros += 1;
        tracing::trace!(
            mantissa = *mantissa,
            mantissa_tzeros = *mantissa_tzeros,
            "encountered zero digit – incrementing trailing‑zero counter",
        );
        true
    } else {
        for iter in 0..=*mantissa_tzeros {
            if *mantissa > (UPPER_BOUND / 10) {
                tracing::warn!(
                    mantissa = *mantissa,
                    mantissa_tzeros = *mantissa_tzeros,
                    "overflow detected while scaling mantissa (iteration = {iter})",
                );
                return false;
            }
            *mantissa *= 10;
            tracing::trace!(
                iter,
                scaled_mantissa = *mantissa,
                "scaled mantissa by 10 due to trailing zeros/non‑zero digit",
            );
        }

        *mantissa += (ch - b'0') as i64;
        *mantissa_tzeros = 0;
        tracing::trace!(
            mantissa_after = *mantissa,
            "added non‑zero digit and reset trailing‑zero counter",
        );
        true
    }
}

#[cfg(test)]
mod process_mantissa_digit_tests {
    use super::*;

    #[traced_test]
    fn zero_digit_increments_trailing_zero_counter() {
        let mut mantissa = 123_i64;
        let mut tzeros = 0_i32;

        assert!(process_mantissa_digit(b'0', &mut mantissa, &mut tzeros));
        assert_eq!(mantissa, 123);
        assert_eq!(tzeros, 1);
    }

    #[traced_test]
    fn non_zero_digit_scales_and_resets_counter() {
        let mut mantissa = 12_i64;
        let mut tzeros = 2_i32; // simulating two buffered trailing zeros

        assert!(process_mantissa_digit(b'3', &mut mantissa, &mut tzeros));
        // scaling: 12 → 120 → 1 200 → 12 000, then +3 → 12 003
        assert_eq!(mantissa, 12_003);
        assert_eq!(tzeros, 0);
    }

    #[traced_test]
    fn scaling_overflow_is_detected_and_reported() {
        let original = (UPPER_BOUND / 10) + 1; // guarantee overflow on first scale
        let mut mantissa = original;
        let mut tzeros = 0_i32;

        assert!(!process_mantissa_digit(b'1', &mut mantissa, &mut tzeros));
        // state must remain unchanged after overflow short‑circuit
        assert_eq!(mantissa, original);
        assert_eq!(tzeros, 0);
    }

    #[traced_test]
    fn upper_bound_edge_case_is_handled_correctly() {
        let mut mantissa = UPPER_BOUND / 10;
        let mut tzeros = 0_i32;

        assert!(process_mantissa_digit(b'9', &mut mantissa, &mut tzeros));
        // result should equal the precise upper bound without overflow
        assert_eq!(mantissa, UPPER_BOUND);
        assert_eq!(tzeros, 0);
    }
}
