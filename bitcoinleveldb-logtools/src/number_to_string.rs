// ---------------- [ File: bitcoinleveldb-logtools/src/number_to_string.rs ]
crate::ix!();

/**
  | Return a human-readable printout of
  | "num"
  |
  */
pub fn number_to_string(num: u64) -> String {
    trace!("number_to_string: num={}", num);
    let mut r = String::new();
    append_number_to(&mut r as *mut String, num);
    r
}

#[cfg(test)]
mod number_to_string_spec {
    use super::*;

    #[traced_test]
    fn number_to_string_matches_standard_to_string_for_representative_values() {
        let values = [
            0u64,
            1u64,
            7u64,
            42u64,
            999u64,
            10_000u64,
            u64::MAX,
        ];

        for &value in &values {
            let expected = value.to_string();
            let actual = number_to_string(value);

            info!(
                "number_to_string_matches_standard_to_string_for_representative_values: value={} expected='{}' actual='{}'",
                value,
                expected,
                actual
            );

            assert_eq!(actual, expected);
        }
    }

    #[traced_test]
    fn number_to_string_produces_non_empty_for_non_zero_values() {
        let value = 123456789u64;
        let result = number_to_string(value);

        info!(
            "number_to_string_produces_non_empty_for_non_zero_values: value={} result='{}'",
            value,
            result
        );

        assert!(!result.is_empty());
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[traced_test]
    fn number_to_string_zero_is_single_zero_character() {
        let result = number_to_string(0);

        info!(
            "number_to_string_zero_is_single_zero_character: result='{}'",
            result
        );

        assert_eq!(result, "0");
    }
}
