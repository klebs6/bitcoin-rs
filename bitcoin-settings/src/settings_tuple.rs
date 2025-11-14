// ---------------- [ File: bitcoin-settings/src/settings_tuple.rs ]
crate::ix!();

pub struct SettingsTuple((String,SettingsValue));

impl std::fmt::Display for SettingsTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Render as a single-key JSON object identical to the C++ behavior.
        // Equivalent to:
        //   SettingsValue out(SettingsValue::VOBJ);
        //   out.__pushKV(kv.first, kv.second);
        //   os << out.write();
        let (ref k, ref v) = self.0;

        // Minimal JSON string escaping for the key.
        fn escape_json_key(s: &str) -> String {
            let mut out = String::with_capacity(s.len() + 2);
            for ch in s.chars() {
                match ch {
                    '"'  => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => {
                        use std::fmt::Write as _;
                        let _ = write!(&mut out, "\\u{:04x}", c as u32);
                    }
                    c => out.push(c),
                }
            }
            out
        }

        let key_escaped = escape_json_key(k);
        trace!("SettingsTuple::fmt – key='{}'", key_escaped);
        write!(f, "{{\"{}\": {}}}", key_escaped, v)
    }
}

#[cfg(test)]
mod settings_tuple_display_spec {

    use super::*;

    #[traced_test]
    fn display_renders_single_key_object() {
        info!("Verifying SettingsTuple Display renders as a single-key JSON object");
        let t = SettingsTuple(("k".to_string(), sv_json("\"v\"")));
        let s = t.to_string();
        debug!("Rendered: {}", s);
        assert_eq!(s, "{\"k\": \"v\"}");
    }

    #[traced_test]
    fn display_escapes_special_characters_in_key() {
        info!("Verifying SettingsTuple Display JSON-escapes key characters");
        let t = SettingsTuple(("a\"b\n".to_string(), sv_json("1")));
        let s = t.to_string();
        debug!("Rendered: {}", s);
        // The key should be escaped for JSON: quote and newline.
        assert_eq!(s, "{\"a\\\"b\\n\": 1}");
    }
}
