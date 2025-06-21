// ---------------- [ File: bitcoin-string/src/parse_integral.rs ]
crate::ix!();

/// Generic helper that reproduces the exact control‑flow of the original C++
/// `ParseIntegral`.  
/// * Rejects the legacy “`+-`” prefix.  
/// * Accepts one leading `'+'` by stripping it before parsing.  
/// * Requires the *entire* string to be a valid, in‑range integer.  
///
/// On success, the parsed value is written to `out` **iff** the pointer is
/// non‑null and the function returns `true`.
pub fn parse_integral<T>(str_: &str, out: *mut T) -> bool
where
    T: std::str::FromStr + Copy + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    trace!(input = %str_, "parse_integral entered");

    // Reject the historical “+-” sequence verbatim.
    if str_.len() >= 2 && &str_[..1] == "+" && &str_[1..2] == "-" {
        debug!("leading '+-' sequence detected – refusing to parse");
        return false;
    }

    // Strip a single leading '+' (matches strtol/strtoul behaviour).
    let slice = if str_.starts_with('+') { &str_[1..] } else { str_ };

    match slice.parse::<T>() {
        Ok(val) => {
            if !out.is_null() {
                // SAFETY: caller provided a mutable pointer they own, mirroring the C++ API.
                unsafe { *out = val };
                info!(?val, "parsed value stored through out‑pointer");
            } else {
                debug!("out pointer is null – result intentionally discarded");
            }
            true
        }
        Err(e) => {
            error!(%slice, error = ?e, "failed to parse integral");
            false
        }
    }
}

/// Strict 32‑bit signed parse.
pub fn parse_int32(str_: &String, out: *mut i32) -> bool {
    parse_integral::<i32>(str_.as_str(), out)
}

/// Strict 64‑bit signed parse.
pub fn parse_int64(str_: &String, out: *mut i64) -> bool {
    parse_integral::<i64>(str_.as_str(), out)
}

/// Strict 8‑bit unsigned parse.
pub fn parse_uint8(str_: &str, out: *mut u8) -> bool {
    parse_integral::<u8>(str_, out)
}

/// Strict 16‑bit unsigned parse.
pub fn parse_uint16(str_: &str, out: *mut u16) -> bool {
    parse_integral::<u16>(str_, out)
}

/// Strict 32‑bit unsigned parse.
pub fn parse_uint32(str_: &str, out: *mut u32) -> bool {
    parse_integral::<u32>(str_, out)
}

/// Strict 64‑bit unsigned parse.
pub fn parse_uint64(str_: &str, out: *mut u64) -> bool {
    parse_integral::<u64>(str_, out)
}

#[cfg(test)]
mod parse_integral_tests {
    use super::*;
    use std::ptr;

    /// Helper that exercises the generic engine without panicking.
    fn assert_parse<T>(input: &str, expected: Option<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug + std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut slot: T = unsafe { std::mem::zeroed() };
        let success = parse_integral::<T>(input, &mut slot as *mut T);

        match expected {
            Some(v) => {
                assert!(
                    success,
                    "input '{input}' expected to succeed but returned false"
                );
                assert_eq!(slot, v, "input '{input}' parsed to wrong value");
            }
            None => assert!(
                !success,
                "input '{input}' expected to fail but returned true"
            ),
        }
    }

    #[traced_test]
    fn int32_success_and_failure_matrix() {
        assert_parse::<i32>("0", Some(0));
        assert_parse::<i32>("+1", Some(1));
        assert_parse::<i32>("-1", Some(-1));
        assert_parse::<i32>("2147483647", Some(2_147_483_647));

        // boundary failures
        assert_parse::<i32>("2147483648", None); // overflow
        assert_parse::<i32>("-2147483649", None); // underflow
        assert_parse::<i32>("+-5", None);         // banned prefix
        assert_parse::<i32>("5xyz", None);        // trailing garbage
    }

    #[traced_test]
    fn int64_extreme_bounds() {
        assert_parse::<i64>("9223372036854775807", Some(9_223_372_036_854_775_807));
        assert_parse::<i64>("-9223372036854775808", Some(-9_223_372_036_854_775_808));
        assert_parse::<i64>("9223372036854775808", None);  // overflow
        assert_parse::<i64>("-9223372036854775809", None); // underflow
    }

    #[traced_test]
    fn unsigned_edge_cases() {
        // u8
        assert_parse::<u8>("0", Some(0));
        assert_parse::<u8>("+255", Some(255));
        assert_parse::<u8>("256", None); // overflow
        assert_parse::<u8>("-1", None);  // negative into unsigned

        // u32: check plus handling and overflow
        assert_parse::<u32>("+4294967295", Some(4_294_967_295));
        assert_parse::<u32>("4294967296", None);
    }

    #[traced_test]
    fn null_out_pointer_still_reports_success() {
        let ok = parse_int32(&"+42".to_owned(), ptr::null_mut());
        assert!(ok, "parse must succeed even when result is discarded");
    }
}
