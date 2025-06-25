// ---------------- [ File: bitcoin-univalue/src/json_isspace.rs ]
crate::ix!();

/// Return `true` when the byte (expressed as `i32`)
/// is one of the four JSON‑defined whitespace
/// characters: space, horizontal‑tab, line‑feed, or carriage‑return.
///
/// See RFC 7159 §2.
#[instrument(level = "trace", skip_all)]
pub fn json_isspace(ch: i32) -> bool {
    let is_space = matches!(ch, 0x20 | 0x09 | 0x0A | 0x0D);
    trace!(chr = ch, is_space);
    is_space
}

#[cfg(test)]
mod json_isspace_spec {
    use super::*;

    #[traced_test]
    fn recognizes_allowed_whitespace() {
        for c in [0x20, 0x09, 0x0A, 0x0D] {
            assert!(json_isspace(c));
        }
    }

    #[traced_test]
    fn rejects_other_control_chars() {
        for c in [0x00, 0x1F, 0x7F] {
            assert!(!json_isspace(c));
        }
    }

    #[traced_test]
    fn rejects_regular_printable_chars() {
        for c in ['a', 'Z', '0', '{'].into_iter().map(|c| c as i32) {
            assert!(!json_isspace(c));
        }
    }
}
