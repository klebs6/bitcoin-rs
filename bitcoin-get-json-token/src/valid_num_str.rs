// ---------------- [ File: bitcoin-get-json-token/src/valid_num_str.rs ]
crate::ix!();

/// Lightweight helper – returns **true** iff `s` is *exactly* one
/// syntactically valid JSON number.
#[instrument(level = "trace", skip_all)]
pub fn valid_num_str(s: &str) -> bool {
    let mut tok_val  = String::new();
    let mut consumed = 0u32;
    let tok = get_json_token(
        &mut tok_val,
        &mut consumed,
        s.as_ptr(),
        unsafe { s.as_ptr().add(s.len()) },
    );

    let ok = tok == JTokenType::JTOK_NUMBER && consumed as usize == s.len();
    trace!(input = s, ?tok, consumed, ok, "valid_num_str verdict");
    ok
}

#[cfg(test)]
mod valid_num_str_spec {
    use super::*;

    #[traced_test]
    fn accepts_valid_numbers() {
        for n in ["0", "-12.34", "3e+7"] {
            assert!(valid_num_str(n), "{n}");
        }
    }

    #[traced_test]
    fn rejects_invalid_numbers() {
        for n in ["", " 1", "--1", "01", "1e"] {
            assert!(!valid_num_str(n), "{n}");
        }
    }
}
