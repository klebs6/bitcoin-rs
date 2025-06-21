// ---------------- [ File: bitcoin-string/src/parse_money.rs ]
crate::ix!();

/**
  | Parse an amount denoted in full coins.
  | E.g. "0.0034" supplied on the command
  | line. *
  |
  */
pub fn parse_money(money_string: &String) -> Option<Amount> {
    trace!(target: "parse_money", ?money_string, "starting parse");

    /* 1. C‑string validation */
    if !valid_as_cstring(money_string) {
        trace!("embedded NUL detected – invalid C‑string");
        return None;
    }

    /* 2. trim leading / trailing space */
    let s = money_string.trim();
    if s.is_empty() {
        trace!("string empty after trim");
        return None;
    }

    /* 3. scan integral + fractional parts */
    let mut str_whole = String::with_capacity(10);
    let mut n_units: i64 = 0;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '.' {
            let mut n_mult: i64 = COIN / 10;
            while let (Some(&d), true) = (chars.peek(), n_mult > 0) {
                if d.is_ascii_digit() {
                    n_units += n_mult * (d as i64 - '0' as i64);
                    n_mult /= 10;
                    chars.next();
                } else {
                    break;
                }
            }
            break;
        }

        if ch.is_ascii_whitespace() || !ch.is_ascii_digit() {
            trace!(%ch, "invalid character in integral part");
            return None;
        }
        str_whole.push(ch);
    }

    /* trailing garbage */
    if chars.next().is_some() {
        trace!("extraneous trailing characters detected");
        return None;
    }

    /* guards */
    if str_whole.len() > 10 || !(0..=COIN).contains(&n_units) {
        trace!("integral or fractional part out of range");
        return None;
    }

    /* integral parse */
    let n_whole: i64 = if str_whole.is_empty() {
        0
    } else {
        match str_whole.parse() {
            Ok(v) => v,
            Err(e) => {
                error!(error = %e, "failed to parse integral part");
                return None;
            }
        }
    };

    /* compose final amount */
    let value = match n_whole.checked_mul(COIN).and_then(|v| v.checked_add(n_units)) {
        Some(v) => v,
        None => {
            trace!("overflow composing final amount");
            return None;
        }
    };

    if !money_range(&value) {
        trace!(value, "value outside valid money range");
        return None;
    }

    trace!(value, "parse successful");
    Some(value)
}


#[cfg(test)]
mod parse_money_tests {
    use super::*;

    /* -------- valid inputs -------- */
    #[traced_test]
    fn parses_zero_and_simple_values() {
        assert_eq!(parse_money(&"0".to_string()), Some(0));
        assert_eq!(parse_money(&"1".to_string()), Some(COIN));
        assert_eq!(parse_money(&"  123  ".to_string()), Some(123 * COIN));
    }

    #[traced_test]
    fn parses_fractional_values_up_to_8_decimals() {
        assert_eq!(parse_money(&"0.1".to_string()), Some(COIN / 10));
        assert_eq!(parse_money(&"0.01".to_string()), Some(COIN / 100));
        assert_eq!(parse_money(&"0.00000001".to_string()), Some(1));
        assert_eq!(
            parse_money(&"1.00000001".to_string()),
            Some(COIN + 1)
        );
        assert_eq!(
            parse_money(&"21000000.00000000".to_string()),
            Some(MAX_MONEY)
        );
    }

    /* -------- invalid inputs -------- */
    #[traced_test]
    fn rejects_empty_and_whitespace_only() {
        assert_eq!(parse_money(&"".to_string()), None);
        assert_eq!(parse_money(&"   ".to_string()), None);
    }

    #[traced_test]
    fn rejects_internal_spaces_and_letters() {
        assert_eq!(parse_money(&"1 0".to_string()), None);
        assert_eq!(parse_money(&"abc".to_string()), None);
        assert_eq!(parse_money(&"12abc".to_string()), None);
    }

    #[traced_test]
    fn rejects_multiple_decimals_or_excess_fractional_digits() {
        assert_eq!(parse_money(&"1.0.0".to_string()), None);
        assert_eq!(parse_money(&"0.000000001".to_string()), None); /* 9 decimals */
    }

    #[traced_test]
    fn rejects_integral_overflow_or_value_out_of_range() {
        assert_eq!(parse_money(&"12345678901".to_string()), None); /* 11 digits */
        assert_eq!(parse_money(&"21000001".to_string()), None);    /* > MAX_MONEY */
    }

    #[traced_test]
    fn rejects_embedded_nul() {
        assert_eq!(parse_money(&"abc\0def".to_string()), None);
    }
}
