crate::ix!();

/// Parse number as fixed point according to JSON number syntax.
/// 
/// See https://json.org/number.gif
/// 
/// ----------- @note
/// 
/// The result must be in the range (-10^18,10^18), otherwise an overflow error will trigger.
/// 
/// ----------- @return
/// 
/// true on success, false on error.
///
/// ----------- @ note
/// Parse a JSON‐syntax number into a signed 64‑bit fixed‑point value.
///
/// * `val`      – Text representation of the number (JSON number grammar).
/// * `decimals` – Number of implied decimal places in the *target* fixed‑point type.
/// * `amount_out` – Destination for the parsed mantissa; may be a null pointer.
///
/// Returns **`true`** on success or **`false`** if the input is out‑of‑range,
/// malformed, or cannot be represented with the requested precision.
///
/// The resulting integer is guaranteed to lie strictly inside the range
/// `(-10¹⁸, 10¹⁸)`.
pub fn parse_fixed_point(val: &str, decimals: i32, amount_out: *mut i64) -> bool {

    const UPPER_BOUND: i64 = 1_000_000_000_000_000_000 - 1; // 10¹⁸ − 1

    debug!(input = %val, decimals, "parse_fixed_point: start");

    let mut mantissa: i64 = 0;
    let mut exponent: i64 = 0;
    let mut mantissa_tzeros: i32 = 0;
    let mut mantissa_sign = false;
    let mut exponent_sign = false;
    let mut ptr: usize = 0;
    let end = val.len();
    let mut point_ofs: i32 = 0;
    let bytes = val.as_bytes();

    /* sign */
    if ptr < end && bytes[ptr] == b'-' {
        mantissa_sign = true;
        ptr += 1;
    }

    /* integer part */
    if ptr < end {
        match bytes[ptr] {
            b'0' => ptr += 1, /* single zero */
            b'1'..=b'9' => {
                while ptr < end && bytes[ptr].is_ascii_digit() {
                    if !process_mantissa_digit(bytes[ptr], &mut mantissa, &mut mantissa_tzeros) {
                        error!("mantissa overflow while scanning integer part");
                        return false;
                    }
                    ptr += 1;
                }
            }
            _ => return false, /* expected digit */
        }
    } else {
        return false; /* empty string or bare '-' */
    }

    /* fractional part */
    if ptr < end && bytes[ptr] == b'.' {
        ptr += 1;
        if ptr < end && bytes[ptr].is_ascii_digit() {
            while ptr < end && bytes[ptr].is_ascii_digit() {
                if !process_mantissa_digit(bytes[ptr], &mut mantissa, &mut mantissa_tzeros) {
                    error!("mantissa overflow while scanning fractional part");
                    return false;
                }
                ptr += 1;
                point_ofs += 1;
            }
        } else {
            return false; /* expected digit after '.' */
        }
    }

    /* exponent */
    if ptr < end && (bytes[ptr] == b'e' || bytes[ptr] == b'E') {
        ptr += 1;
        if ptr < end && bytes[ptr] == b'+' {
            ptr += 1;
        } else if ptr < end && bytes[ptr] == b'-' {
            exponent_sign = true;
            ptr += 1;
        }

        if ptr < end && bytes[ptr].is_ascii_digit() {
            while ptr < end && bytes[ptr].is_ascii_digit() {
                if exponent > UPPER_BOUND / 10 {
                    error!("exponent overflow");
                    return false;
                }
                exponent = exponent * 10 + (bytes[ptr] - b'0') as i64;
                ptr += 1;
            }
        } else {
            return false; /* expected digit after 'e' or 'E' */
        }
    }

    if ptr != end {
        return false; /* trailing garbage */
    }

    /* apply exponent sign and adjust for decimal point + trimmed zeros */
    if exponent_sign {
        exponent = -exponent;
    }
    exponent = exponent - point_ofs as i64 + mantissa_tzeros as i64;

    /* apply mantissa sign */
    if mantissa_sign {
        mantissa = -mantissa;
    }

    /* convert to unified fixed‑point value */
    exponent += decimals as i64;
    if exponent < 0 || exponent >= 18 {
        return false; /* under‑ or overflow */
    }

    for _ in 0..(exponent as usize) {
        if mantissa > UPPER_BOUND / 10 || mantissa < -UPPER_BOUND / 10 {
            error!("mantissa overflow during scaling");
            return false;
        }
        mantissa *= 10;
    }

    if mantissa > UPPER_BOUND || mantissa < -UPPER_BOUND {
        return false; /* final overflow check */
    }

    if !amount_out.is_null() {
        // SAFETY: caller opted‑in by providing a non‑null pointer.
        unsafe { *amount_out = mantissa };
    }

    debug!(result = mantissa, "parse_fixed_point: success");
    true
}



#[cfg(test)]
mod parse_fixed_point_tests {
    use super::*;
    use std::ptr;

    /// Helper that invokes `parse_fixed_point` and validates the outcome.
    fn check(
        input: &str,
        decimals: i32,
        expect_ok: bool,
        expect_val: i64,
        use_null_out: bool,
    ) {
        let mut out: i64 = 0;
        let out_ptr = if use_null_out { ptr::null_mut() } else { &mut out };
        let ok = parse_fixed_point(input, decimals, out_ptr);
        assert_eq!(
            ok, expect_ok,
            "unexpected success/failure for “{}” with {} decimals",
            input, decimals
        );
        if expect_ok && !use_null_out {
            assert_eq!(
                out, expect_val,
                "mismatched parsed value for “{}” with {} decimals",
                input, decimals
            );
        }
    }

    #[traced_test]
    fn integer_zero() {
        check("0", 0, true, 0, false);
    }

    #[traced_test]
    fn integer_positive() {
        check("123456", 0, true, 123_456, false);
    }

    #[traced_test]
    fn integer_negative() {
        check("-98765", 0, true, -98_765, false);
    }

    #[traced_test]
    fn fractional_exact_decimals() {
        check("1.23", 2, true, 123, false);
    }

    #[traced_test]
    fn fractional_leading_zero() {
        check("0.01", 2, true, 1, false);
    }

    #[traced_test]
    fn fractional_trailing_zeros() {
        check("12.3400", 2, true, 1_234, false);
    }

    #[traced_test]
    fn exponent_positive() {
        check("1e3", 0, true, 1_000, false);
    }

    #[traced_test]
    fn exponent_negative() {
        check("1e-1", 1, true, 1, false);
    }

    #[traced_test]
    fn decimal_and_exponent() {
        check("1.23e2", 0, true, 123, false);
    }

    #[traced_test]
    fn overflow_large_positive() {
        check("1000000000000000000", 0, false, 0, false); // 10¹⁸
    }

    #[traced_test]
    fn overflow_large_negative() {
        check("-1000000000000000000", 0, false, 0, false); // -10¹⁸
    }

    #[traced_test]
    fn underflow_small_fraction() {
        check("0.0001", 2, false, 0, false); // needs 4 decimals but only 2 allowed
    }

    #[traced_test]
    fn invalid_empty_string() {
        check("", 0, false, 0, false);
    }

    #[traced_test]
    fn invalid_trailing_garbage() {
        check("1.0xyz", 0, false, 0, false);
    }

    #[traced_test]
    fn null_output_pointer_still_ok() {
        check("42", 0, true, 0, true);
    }
}
