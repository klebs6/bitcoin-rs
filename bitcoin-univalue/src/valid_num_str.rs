// ---------------- [ File: bitcoin-univalue/src/valid_num_str.rs ]
crate::ix!();

/// Cheap, zero‑allocation probe used by several parsers –
/// returns **true** when *s* is a syntactically valid JSON
/// number (per `get_json_token`) and *nothing else* follows.
#[instrument(level = "trace", skip_all)]
pub fn valid_num_str(s: &str) -> bool {
    let mut token_val = String::new();
    let mut consumed  = 0u32;
    let tok = get_json_token(
        &mut token_val,
        &mut consumed,
        s.as_ptr(),
        unsafe { s.as_ptr().add(s.len()) },
    );
    tok == JTokenType::JTOK_NUMBER && consumed as usize == s.len()
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
