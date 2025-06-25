// ---------------- [ File: bitcoin-univalue/src/json_token_is_value.rs ]
crate::ix!();

/// Return `true` when *jtt* represents one of the five
/// terminal JSON value token types (null / true / false / number / string).
#[inline]
#[instrument(level = "trace", skip_all)]
pub fn json_token_is_value(jtt: JTokenType) -> bool {
    matches!(
        jtt,
        JTokenType::JTOK_KW_NULL
            | JTokenType::JTOK_KW_TRUE
            | JTokenType::JTOK_KW_FALSE
            | JTokenType::JTOK_NUMBER
            | JTokenType::JTOK_STRING
    )
}

#[cfg(test)]
mod json_token_is_value_spec {
    use super::*;

    #[traced_test]
    fn value_tokens_are_true() {
        for t in [
            JTokenType::JTOK_KW_NULL,
            JTokenType::JTOK_KW_TRUE,
            JTokenType::JTOK_KW_FALSE,
            JTokenType::JTOK_NUMBER,
            JTokenType::JTOK_STRING,
        ] {
            assert!(json_token_is_value(t));
        }
    }

    #[traced_test]
    fn structural_tokens_are_false() {
        for t in [
            JTokenType::JTOK_OBJ_OPEN,
            JTokenType::JTOK_ARR_CLOSE,
            JTokenType::JTOK_COMMA,
            JTokenType::JTOK_COLON,
        ] {
            assert!(!json_token_is_value(t));
        }
    }
}
