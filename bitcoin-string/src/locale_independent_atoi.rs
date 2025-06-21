// ---------------- [ File: bitcoin-string/src/locale_independent_atoi.rs ]
crate::ix!();

/**
  | LocaleIndependentAtoi is provided for backwards
  | compatibility reasons.
  |
  | New code should use ToIntegral or the ParseInt*
  | functions which provide parse error feedback.
  |
  | The goal of LocaleIndependentAtoi is to
  | replicate the exact defined behaviour of atoi
  | and atoi64 as they behave under the "C" locale.
  */
pub fn locale_independent_atoi<T>(s: &str) -> T
where
    T: std::str::FromStr<Err = std::num::ParseIntError> + Default,
{
    trace!("locale_independent_atoi: input = \"{}\"", s);
    let trimmed = s.trim();

    /* emulate C `atoi` handling of leading “+” and “+-” */
    let cleaned = if trimmed.starts_with('+') {
        if trimmed.get(1..2) == Some("-") {
            trace!("locale_independent_atoi: detected \"+-\" sequence → 0");
            return T::default();
        }
        &trimmed[1..]
    } else {
        trimmed
    };

    match cleaned.parse::<T>() {
        Ok(v) => v,
        Err(_) => {
            trace!("locale_independent_atoi: parse error → 0");
            T::default()
        }
    }
}

#[cfg(test)]
mod tests_locale_independent_atoi {
    use super::*;

    #[traced_test]
    fn parses_signed_and_unsigned() {
        let v: i32 = locale_independent_atoi(" -42 ");
        assert_eq!(v, -42);

        let u: u32 = locale_independent_atoi("42");
        assert_eq!(u, 42);
    }

    #[traced_test]
    fn plus_minus_sequence_returns_zero() {
        let v: i64 = locale_independent_atoi("+-7");
        assert_eq!(v, 0);
    }

    #[traced_test]
    fn overflow_returns_zero() {
        let big = "92233720368547758070"; // > i64::MAX
        let v: i64 = locale_independent_atoi(big);
        assert_eq!(v, 0);
    }
}
