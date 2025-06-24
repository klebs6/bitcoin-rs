// ---------------- [ File: bitcoin-univalue/src/init_json_escape.rs ]
crate::ix!();

/// Up‑stream C++ performs an expensive, one‑time construction of the
/// `escapes` table.  In Rust we already ship a *const* lookup‑table in
/// `escapes.rs`, therefore the helper becomes a cheap, idempotent
/// no‑op.  It remains public only to satisfy the legacy interface and
/// associated unit‑tests.
#[instrument(level = "trace", skip_all)]
pub fn init_json_escape() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        trace!("init_json_escape called – nothing to do (table baked‑in)");
    });
}

#[cfg(test)]
mod init_json_escape_spec {
    use super::*;

    /// After calling `init_json_escape` the helper should be idempotent:
    /// re‑invocation must neither panic nor alter already‑initialised
    /// state.  We therefore call it twice.
    #[traced_test]
    fn idempotent_initialisation() {
        init_json_escape();
        init_json_escape(); // second call must be harmless
    }

    /// The low‑ASCII control characters (U+0000 … U+001F) should be
    /// converted to their canonical `\u00XX` escape sequences, while
    /// the printable ASCII subset receives specific short‑hand escapes.
    #[traced_test]
    fn control_and_quote_escapes_match_reference() {
        init_json_escape();

        // core control escapes
        let cases = [
            ("\u{0008}", "\\b"),   // backspace
            ("\u{000C}", "\\f"),   // form‑feed
            ("\n",       "\\n"),   // line‑feed
            ("\r",       "\\r"),   // carriage‑return
            ("\t",       "\\t"),   // horizontal tab
            ("\"",       "\\\""),  // quotation mark
            ("\\",       "\\\\"),
            ("\u{007F}", "\\u007f"),
        ];

        for (plain, escaped) in cases {
            assert_eq!(json_escape(plain), escaped);
        }

        // generic U+0001 should map to \u0001
        assert_eq!(json_escape("\u{0001}"), "\\u0001");
    }
}
