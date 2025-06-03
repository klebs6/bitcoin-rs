// ---------------- [ File: bitcoin-log/src/interface.rs ]
//-------------------------------------------[.cpp/bitcoin/src/logging.cpp]
crate::ix!();

/// Belts and suspenders: make sure outgoing log messages don't contain potentially suspicious
/// characters, such as terminal control codes.
/// 
/// This escapes control characters except newline ('\n') in C syntax.
/// 
/// It escapes instead of removes them to still allow for troubleshooting issues where they
/// accidentally end up in strings.
///
/// A minimal imitation of the C++ `LogEscapeMessage`.
///
/// Replaces any control chars < 32 (except '\n') and 127 with `\x??`.
///
pub fn log_escape_message(s: &str) -> String {
    // Removed the `trace!()` call here to prevent re‐entrant logging deadlock.
    let mut ret = String::with_capacity(s.len());
    for ch in s.chars() {
        let code = ch as u32;
        if (code >= 32 && code != 127) || ch == '\n' {
            ret.push(ch);
        } else {
            ret.push_str(&format!("\\x{:02X}", code));
        }
    }
    ret
}

#[cfg(test)]
mod logger_escape_tests {
    use super::*;

    /// Thoroughly test `log_escape_message()` with control codes, newlines, and normal text.
    #[traced_test]
    #[serial]
    fn test_log_escape_message() {
        info!("Testing log_escape_message for escaping control characters.");

        // Contains normal text, newline, ASCII 0x7F, and control chars 0x00, 0x19
        let input = "Normal\nText\x7F\x00And\x19Stuff";
        let escaped = log_escape_message(input);

        debug!("Escaped output: {:?}", escaped);

        // Newline should remain a literal '\n'
        assert!(
            escaped.contains('\n'),
            "Newline should remain unescaped."
        );

        // 0x7F => \x7F
        assert!(
            escaped.contains("\\x7F"),
            "0x7F must become '\\x7F'."
        );

        // 0x00 => \x00
        assert!(
            escaped.contains("\\x00"),
            "0x00 must become '\\x00'."
        );

        // 0x19 => \x19
        assert!(
            escaped.contains("\\x19"),
            "0x19 must become '\\x19'."
        );

        // Normal letters should remain unchanged
        assert!(
            escaped.contains("Normal"),
            "Regular letters must remain as-is."
        );

        trace!("test_log_escape_message passed.");
    }
}
