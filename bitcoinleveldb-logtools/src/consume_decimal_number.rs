// ---------------- [ File: bitcoinleveldb-logtools/src/consume_decimal_number.rs ]
crate::ix!();

/**
  | Parse a human-readable number from "*in" into
  | *value.  On success, advances "*in" past the
  | consumed number and sets "*val" to the numeric
  | value.  Otherwise, returns false and leaves *in
  | in an unspecified state.
  */
pub fn consume_decimal_number(in_: *mut Slice, val: *mut u64) -> bool {
    trace!("consume_decimal_number: enter");

    if in_.is_null() || val.is_null() {
        error!(
            "consume_decimal_number: null pointer(s): in_={:?}, val={:?}",
            in_, val
        );
        return false;
    }

    unsafe {
        let input: &mut Slice = &mut *in_;
        let max_u64: u64 = u64::MAX;
        let last_digit_of_max: u8 = b'0' + (max_u64 % 10) as u8;

        let data_ptr = input.data();
        if data_ptr.is_null() {
            debug!("consume_decimal_number: input Slice has null outer data pointer");
            *val = 0;
            return false;
        }

        let inner_ptr = *data_ptr;
        if inner_ptr.is_null() {
            debug!("consume_decimal_number: input Slice has null inner data pointer");
            *val = 0;
            return false;
        }

        let len = *input.size();
        let bytes = core::slice::from_raw_parts(inner_ptr, len);

        let mut value: u64 = 0;
        let mut digits_consumed: usize = 0;

        for &ch in bytes {
            if ch < b'0' || ch > b'9' {
                break;
            }

            let digit = (ch - b'0') as u64;

            if value > max_u64 / 10
                || (value == max_u64 / 10 && ch > last_digit_of_max)
            {
                warn!(
                    "consume_decimal_number: overflow while parsing; current_value={} digit={}",
                    value, digit
                );
                return false;
            }

            value = value
                .saturating_mul(10)
                .saturating_add(digit);
            digits_consumed += 1;
        }

        *val = value;

        if digits_consumed > 0 {
            input.remove_prefix(digits_consumed);
            trace!(
                "consume_decimal_number: parsed value={} digits_consumed={}",
                value,
                digits_consumed
            );
            true
        } else {
            trace!("consume_decimal_number: no leading digits found");
            false
        }
    }
}

#[cfg(test)]
mod consume_decimal_number_spec {
    use super::*;

    fn slice_and_backing_from_str(input: &str) -> (Slice, Vec<u8>) {
        let backing = input.as_bytes().to_vec();
        let slice = Slice::from_ptr_len(backing.as_ptr(), backing.len());
        (slice, backing)
    }

    fn slice_to_utf8_string(slice: &Slice) -> String {
        unsafe {
            let data_ptr = slice.data();
            if data_ptr.is_null() {
                debug!("slice_to_utf8_string: outer data pointer is null");
                return String::new();
            }

            let inner_ptr = *data_ptr;
            let len = *slice.size();

            if inner_ptr.is_null() || len == 0 {
                debug!(
                    "slice_to_utf8_string: inner pointer null or length zero (len={})",
                    len
                );
                return String::new();
            }

            let bytes = core::slice::from_raw_parts(inner_ptr, len);
            String::from_utf8_lossy(bytes).to_string()
        }
    }

    #[traced_test]
    fn consume_parses_simple_digit_string_and_consumes_all() {
        let (mut slice, _backing) = slice_and_backing_from_str("12345");
        let mut value: u64 = 0;

        info!("consume_parses_simple_digit_string_and_consumes_all: starting");
        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);

        info!(
            "consume_parses_simple_digit_string_and_consumes_all: ok={} value={}",
            ok, value
        );
        let remaining = slice_to_utf8_string(&slice);
        info!(
            "consume_parses_simple_digit_string_and_consumes_all: remaining_slice='{}'",
            remaining
        );

        assert!(ok);
        assert_eq!(value, 12345);
        assert!(remaining.is_empty());
    }

    #[traced_test]
    fn consume_parses_prefix_digits_and_leaves_suffix() {
        let (mut slice, _backing) = slice_and_backing_from_str("123abc");
        let mut value: u64 = 0;

        info!("consume_parses_prefix_digits_and_leaves_suffix: starting");
        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
        let remaining = slice_to_utf8_string(&slice);

        info!(
            "consume_parses_prefix_digits_and_leaves_suffix: ok={} value={} remaining='{}'",
            ok, value, remaining
        );

        assert!(ok);
        assert_eq!(value, 123);
        assert_eq!(remaining, "abc");
    }

    #[traced_test]
    fn consume_fails_when_first_character_not_digit_and_slice_unchanged() {
        let (mut slice, _backing) = slice_and_backing_from_str("abc123");
        let mut value: u64 = 999;

        info!("consume_fails_when_first_character_not_digit_and_slice_unchanged: starting");
        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
        let remaining = slice_to_utf8_string(&slice);

        info!(
            "consume_fails_when_first_character_not_digit_and_slice_unchanged: ok={} value={} remaining='{}'",
            ok, value, remaining
        );

        assert!(!ok);
        // Current implementation sets value to 0 when no digits are consumed.
        assert_eq!(value, 0);
        assert_eq!(remaining, "abc123");
    }

    #[traced_test]
    fn consume_fails_for_empty_slice() {
        let (mut slice, _backing) = slice_and_backing_from_str("");
        let mut value: u64 = 123;

        info!("consume_fails_for_empty_slice: starting");
        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
        let remaining = slice_to_utf8_string(&slice);

        info!(
            "consume_fails_for_empty_slice: ok={} value={} remaining='{}'",
            ok, value, remaining
        );

        assert!(!ok);
        assert_eq!(value, 0);
        assert!(remaining.is_empty());
    }

    #[traced_test]
    fn consume_handles_leading_zeros_correctly() {
        let (mut slice, _backing) = slice_and_backing_from_str("00042");
        let mut value: u64 = 0;

        info!("consume_handles_leading_zeros_correctly: starting");
        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
        let remaining = slice_to_utf8_string(&slice);

        info!(
            "consume_handles_leading_zeros_correctly: ok={} value={} remaining='{}'",
            ok, value, remaining
        );

        assert!(ok);
        assert_eq!(value, 42);
        assert!(remaining.is_empty());
    }

    #[traced_test]
    fn consume_parses_exact_u64_max_without_overflow() {
        let max_value = u64::MAX;
        let input_string = max_value.to_string();
        let (mut slice, _backing) = slice_and_backing_from_str(&input_string);
        let mut value: u64 = 0;

        info!(
            "consume_parses_exact_u64_max_without_overflow: input='{}'",
            input_string
        );

        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
        let remaining = slice_to_utf8_string(&slice);

        info!(
            "consume_parses_exact_u64_max_without_overflow: ok={} value={} remaining='{}'",
            ok, value, remaining
        );

        assert!(ok);
        assert_eq!(value, max_value);
        assert!(remaining.is_empty());
    }

    #[traced_test]
    fn consume_detects_overflow_and_reports_failure() {
        let overflow_value = (u64::MAX as u128) + 1;
        let input_string = overflow_value.to_string();
        let (mut slice, _backing) = slice_and_backing_from_str(&input_string);
        let mut value: u64 = 0;

        info!(
            "consume_detects_overflow_and_reports_failure: input='{}'",
            input_string
        );

        let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);

        info!(
            "consume_detects_overflow_and_reports_failure: ok={} value={}",
            ok, value
        );

        assert!(!ok);
    }

    #[traced_test]
    fn consume_handles_null_input_pointer() {
        let mut value: u64 = 0;

        info!("consume_handles_null_input_pointer: starting");
        let ok =
            consume_decimal_number(core::ptr::null_mut::<Slice>(), &mut value as *mut u64);

        info!(
            "consume_handles_null_input_pointer: ok={} value={}",
            ok, value
        );

        assert!(!ok);
    }

    #[traced_test]
    fn consume_handles_null_value_pointer() {
        let (mut slice, _backing) = slice_and_backing_from_str("123");

        info!("consume_handles_null_value_pointer: starting");
        let ok = consume_decimal_number(
            &mut slice as *mut Slice,
            core::ptr::null_mut::<u64>(),
        );

        info!("consume_handles_null_value_pointer: ok={}", ok);
        assert!(!ok);
    }
}
