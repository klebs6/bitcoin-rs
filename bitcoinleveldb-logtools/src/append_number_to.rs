// ---------------- [ File: bitcoinleveldb-logtools/src/append_number_to.rs ]
crate::ix!();

/**
  | Append a human-readable printout of
  | "num" to *str
  |
  */
pub fn append_number_to(str_: *mut String, num: u64) {
    trace!("append_number_to: num={}", num);

    if str_.is_null() {
        error!("append_number_to: received null String pointer");
        return;
    }

    unsafe {
        let s: &mut String = &mut *str_;
        use std::fmt::Write as _;
        let _ = write!(s, "{}", num);
    }
}

#[cfg(test)]
mod append_number_to_spec {
    use super::*;

    fn append_number_from(start: &str, num: u64) -> String {
        let mut s = start.to_owned();
        append_number_to(&mut s as *mut String, num);
        s
    }

    #[traced_test]
    fn append_number_to_empty_string_yields_decimal_representation() {
        info!(
            "append_number_to_empty_string_yields_decimal_representation: testing num=0 and num=42"
        );

        let result_zero = append_number_from("", 0);
        info!(
            "append_number_to_empty_string_yields_decimal_representation: result_zero={}",
            result_zero
        );
        assert_eq!(result_zero, "0");

        let result_forty_two = append_number_from("", 42);
        info!(
            "append_number_to_empty_string_yields_decimal_representation: result_forty_two={}",
            result_forty_two
        );
        assert_eq!(result_forty_two, "42");
    }

    #[traced_test]
    fn append_number_multiple_calls_concatenate() {
        let mut output = String::from("v=");

        info!(
            "append_number_multiple_calls_concatenate: initial_output={}",
            output
        );

        append_number_to(&mut output as *mut String, 1);
        append_number_to(&mut output as *mut String, 23);
        append_number_to(&mut output as *mut String, 456);

        info!(
            "append_number_multiple_calls_concatenate: final_output={}",
            output
        );
        assert_eq!(output, "v=123456");
    }

    #[traced_test]
    fn append_number_handles_large_u64_values() {
        let value = u64::MAX;
        let expected = value.to_string();

        info!(
            "append_number_handles_large_u64_values: value={} expected={}",
            value, expected
        );

        let result = append_number_from("", value);
        info!(
            "append_number_handles_large_u64_values: result={}",
            result
        );
        assert_eq!(result, expected);
    }

    #[traced_test]
    fn append_number_handles_null_destination_pointer_safely() {
        info!("append_number_handles_null_destination_pointer_safely: invoking with null pointer");
        append_number_to(core::ptr::null_mut::<String>(), 123);
    }
}
