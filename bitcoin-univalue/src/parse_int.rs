// ---------------- [ File: bitcoin-univalue/src/parse_int.rs ]
crate::ix!();

#[instrument(level = "trace", skip_all)]
pub fn parse_int32(str_: &String, out: *mut i32) -> bool {
    if !parse_prechecks(str_) { return false; }

    match str_.parse::<i64>() {
        Ok(n) if (i32::MIN as i64..=i32::MAX as i64).contains(&n) => {
            unsafe { if !out.is_null() { *out = n as i32; } }
            trace!(value = n, "parse_int32 success");
            true
        }
        _ => {
            trace!("parse_int32 failure: range or syntax error");
            false
        }
    }
}

#[instrument(level = "trace", skip_all)]
pub fn parse_int64(str_: &String, out: *mut i64) -> bool {
    if !parse_prechecks(str_) { return false; }

    match str_.parse::<i64>() {
        Ok(n) => {
            unsafe { if !out.is_null() { *out = n; } }
            trace!(value = n, "parse_int64 success");
            true
        }
        Err(e) => {
            trace!(%e, "parse_int64 failure");
            false
        }
    }
}

#[cfg(test)]
mod parse_int_spec {
    use super::*;

    /* ---------------------------------------------------------------------- */
    /* helpers                                                                */
    /* ---------------------------------------------------------------------- */
    fn ok32(s: &str, expect: i32) {
        let mut out = 0i32;
        assert!(
            parse_int32(&s.to_string(), &mut out as *mut i32),
            "{s:?} should parse"
        );
        assert_eq!(out, expect);
    }

    fn err32(s: &str) {
        let mut out = 123i32; // dummy value
        assert!(
            !parse_int32(&s.to_string(), &mut out as *mut i32),
            "{s:?} should fail"
        );
    }

    fn ok64(s: &str, expect: i64) {
        let mut out = 0i64;
        assert!(
            parse_int64(&s.to_string(), &mut out as *mut i64),
            "{s:?} should parse"
        );
        assert_eq!(out, expect);
    }

    fn err64(s: &str) {
        let mut out = 123i64;
        assert!(
            !parse_int64(&s.to_string(), &mut out as *mut i64),
            "{s:?} should fail"
        );
    }

    /* ---------------------------------------------------------------------- */
    /* happy‑path                                                             */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn parses_min_max_and_zero() {
        ok32("0", 0);
        ok32(&i32::MIN.to_string(), i32::MIN);
        ok32(&i32::MAX.to_string(), i32::MAX);

        ok64("0", 0);
        ok64(&i64::MIN.to_string(), i64::MIN);
        ok64(&i64::MAX.to_string(), i64::MAX);
    }

    #[traced_test]
    fn accepts_leading_plus_and_minus() {
        ok32("+7", 7);
        ok32("-7", -7);
        ok64("+7000000000", 7_000_000_000);
        ok64("-7000000000", -7_000_000_000);
    }

    /* ---------------------------------------------------------------------- */
    /* error conditions                                                       */
    /* ---------------------------------------------------------------------- */
    #[traced_test]
    fn rejects_whitespace_padding() {
        err32(" 1");
        err32("1 ");
        err64("\t99");
        err64("99\n");
    }

    #[traced_test]
    fn rejects_non_digit_garbage() {
        err32("abc");
        err32("123abc");
        err64("--1");
        err64("+-1");
    }

    #[traced_test]
    fn detects_overflow_and_underflow() {
        // 32‑bit overflow
        err32("2147483648");
        err32("-2147483649");

        // 64‑bit overflow
        err64("9223372036854775808");
        err64("-9223372036854775809");
    }
}
