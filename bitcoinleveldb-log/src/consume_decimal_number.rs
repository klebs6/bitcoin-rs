// ---------------- [ File: bitcoinleveldb-log/src/consume_decimal_number.rs ]
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
            debug!("consume_decimal_number: input Slice has null data pointer");
            *val = 0;
            return false;
        }

        let len = input.size();
        let bytes = std::slice::from_raw_parts(data_ptr, len);

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

            value = value * 10 + digit;
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
