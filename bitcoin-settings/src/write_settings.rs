// ---------------- [ File: bitcoin-settings/src/write_settings.rs ]
crate::ix!();

/**
  | Write settings file.
  |
  */
pub fn write_settings(
    path:   &std::path::Path,
    values: &std::collections::HashMap<String, SettingsValue>,
    errors: &mut Vec<String>,
) -> bool {
    info!(
        "write_settings: writing {} entrie(s) to '{}'",
        values.len(),
        path.display()
    );

    // Build a pretty-printed JSON object with indentLevel=4 (to mirror C++).
    // Note: We intentionally avoid altering control flow or semantics.
    let mut body = String::new();
    body.push_str("{\n");

    // Deterministic order is not mandated by the original code, but aids debugging.
    let mut entries: Vec<(&String, &SettingsValue)> = values.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    for (idx, (k, v)) in entries.iter().enumerate() {
        // Escape key for JSON and indent by 4 spaces.
        let mut key_escaped = String::new();
        for ch in k.chars() {
            match ch {
                '"'  => key_escaped.push_str("\\\""),
                '\\' => key_escaped.push_str("\\\\"),
                '\n' => key_escaped.push_str("\\n"),
                '\r' => key_escaped.push_str("\\r"),
                '\t' => key_escaped.push_str("\\t"),
                c if (c as u32) < 0x20 => {
                    use std::fmt::Write as _;
                    let _ = write!(&mut key_escaped, "\\u{:04x}", c as u32);
                }
                c => key_escaped.push(c),
            }
        }
        let comma = if idx + 1 == entries.len() { "" } else { "," };
        body.push_str("    \"");
        body.push_str(&key_escaped);
        body.push_str("\": ");
        body.push_str(&format!("{}", v));
        body.push_str(comma);
        body.push('\n');
    }
    body.push('}');
    body.push('\n');

    // Open the file for writing (truncate).
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path);

    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            let msg = format!(
                "Error: Unable to open settings file {} for writing",
                path.display()
            );
            error!("write_settings: {msg}");
            errors.push(msg);
            return false;
        }
    };

    if let Err(e) = std::io::Write::write_all(&mut file, body.as_bytes()) {
        let msg = format!(
            "Error: Unable to write settings file {} ({e})",
            path.display()
        );
        error!("write_settings: {msg}");
        errors.push(msg);
        return false;
    }

    if let Err(e) = std::io::Write::flush(&mut file) {
        let msg = format!(
            "Error: Unable to flush settings file {} ({e})",
            path.display()
        );
        error!("write_settings: {msg}");
        errors.push(msg);
        return false;
    }

    trace!("write_settings: successfully wrote '{}'", path.display());
    true
}


#[cfg(test)]
mod write_settings_filesystem_spec {

    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read as _;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, info};

    fn unique_path_file(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bitcoin_settings_write_{nanos}_{suffix}.json"))
    }

    fn unique_path_dir(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bitcoin_settings_write_dir_{nanos}_{suffix}"))
    }

    #[traced_test]
    fn write_then_read_roundtrip_preserves_keys_and_values() {
        // (write_settings_filesystem_spec) updated to use UniValue::read(raw, size)
        info!("Verifying write_settings produces a readable JSON object that round-trips");

        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("bitcoin_settings_write_{nanos}_roundtrip.json"));

        let mut map = std::collections::HashMap::new();
        map.insert("a".into(), sv_json("1"));
        map.insert("b".into(), SettingsValue::from(true));
        map.insert("c".into(), sv_json("\"str\""));

        let mut errors = Vec::new();
        let ok_write = write_settings(&p, &map, &mut errors);
        debug!("write ok={}, errors={:?}", ok_write, errors);
        assert!(ok_write);
        assert!(errors.is_empty());

        // Read file string and parse with UniValue
        let mut s = String::new();
        std::fs::File::open(&p).unwrap().read_to_string(&mut s).unwrap();
        debug!("file content:\n{}", s);

        let mut u = UniValue::null();
        let bytes = s.as_bytes();
        assert!(u.read(bytes.as_ptr(), bytes.len()));

        assert!(u.is_object());
        let keys = u.get_keys();
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"b".to_string()));
        assert!(keys.contains(&"c".to_string()));

        let vals = u.get_values();
        // Sanity check: we can reconstruct the map from UniValue
        let mut reconstructed = std::collections::HashMap::new();
        for (i, k) in keys.iter().enumerate() {
            reconstructed.insert(k.clone(), SettingsValue(vals[i].clone()));
        }
        assert_eq!(reconstructed.get("a").unwrap().to_string(), "1");
        assert_eq!(reconstructed.get("b").unwrap().to_string(), "true");
        assert_eq!(reconstructed.get("c").unwrap().to_string(), "\"str\"");

        let _ = std::fs::remove_file(&p);
    }

    #[traced_test]
    fn write_to_directory_path_returns_error() {
        info!("Verifying write_settings fails when path is a directory");

        let dir = unique_path_dir("as_dir");
        fs::create_dir_all(&dir).unwrap();

        let mut errors = Vec::new();
        let ok = write_settings(&dir, &HashMap::new(), &mut errors);
        debug!("ok={}, errors={:?}", ok, errors);
        assert!(!ok);
        assert!(!errors.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }
}

#[cfg(test)]
mod write_settings_deterministic_key_ordering_spec {
    use super::*;
    use std::io::Read as _;
    use tracing::{info, debug};

    fn sv_json(j: &str) -> SettingsValue {
        SettingsValue(UniValue::from(j))
    }

    fn unique_path(suffix: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bitcoin_settings_write_order_{nanos}_{suffix}.json"))
    }

    #[traced_test]
    fn keys_are_serialized_in_lexicographic_order() {
        info!("write_settings should serialize keys in deterministic lexicographic order");
        let p = unique_path("order");

        let mut map = std::collections::HashMap::new();
        // Insert out of order on purpose
        map.insert("b".into(), sv_json("2"));
        map.insert("c".into(), sv_json("3"));
        map.insert("a".into(), sv_json("1"));

        let mut errors = Vec::new();
        let ok = write_settings(&p, &map, &mut errors);
        assert!(ok);
        assert!(errors.is_empty());

        let mut s = String::new();
        std::fs::File::open(&p).unwrap().read_to_string(&mut s).unwrap();
        debug!("file content:\n{}", s);

        let ia = s.find("\"a\"").unwrap();
        let ib = s.find("\"b\"").unwrap();
        let ic = s.find("\"c\"").unwrap();
        assert!(ia < ib && ib < ic, "expected a < b < c in the file");

        let _ = std::fs::remove_file(&p);
    }
}
