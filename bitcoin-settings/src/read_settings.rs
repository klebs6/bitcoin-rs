// ---------------- [ File: bitcoin-settings/src/read_settings.rs ]
crate::ix!();

/// Read settings file
pub fn read_settings(
    path:   &std::path::Path,
    values: &mut std::collections::HashMap<String, SettingsValue>,
    errors: &mut Vec<String>,
) -> bool {
    info!("read_settings: attempting to read '{}'", path.display());

    values.clear();
    errors.clear();

    // Ok for file to not exist
    if !path.exists() {
        debug!("read_settings: path does not exist; returning true");
        return true;
    }

    // Open for reading
    let file_res = std::fs::File::open(path);
    let mut file = match file_res {
        Ok(f) => f,
        Err(_) => {
            let msg = format!("{}. Please check permissions.", path.display());
            error!("read_settings: open failed – {msg}");
            errors.push(msg);
            return false;
        }
    };

    // Slurp entire file
    let mut content = String::new();
    if let Err(e) = std::io::Read::read_to_string(&mut file, &mut content) {
        let msg = format!("Failed reading settings file {} ({e})", path.display());
        error!("read_settings: {msg}");
        errors.push(msg);
        return false;
    }
    drop(file); // Done with file descriptor. Release while copying data.

    // Parse as UniValue
    let mut in_val = UniValue::null();
    let bytes = content.as_bytes();
    if !in_val.read(bytes.as_ptr(), bytes.len()) {
        let msg = format!("Unable to parse settings file {}", path.display());
        error!("read_settings: {msg}");
        errors.push(msg);
        return false;
    }

    if !in_val.is_object() {
        let msg = format!(
            "Found non-object value {} in settings file {}",
            in_val.write(None, None),
            path.display()
        );
        error!("read_settings: {msg}");
        errors.push(msg);
        return false;
    }

    let in_keys: Vec<String>     = in_val.get_keys().clone();
    let in_values: Vec<UniValue> = in_val.get_values().clone();
    debug!(
        "read_settings: parsed {} key(s) from '{}'",
        in_keys.len(),
        path.display()
    );

    for i in 0..in_keys.len() {
        let key = &in_keys[i];
        let val = &in_values[i];
        let insert_res = values.insert(key.clone(), SettingsValue(val.clone()));
        if insert_res.is_some() {
            let msg = format!(
                "Found duplicate key {} in settings file {}",
                key,
                path.display()
            );
            warn!("read_settings: {msg}");
            errors.push(msg);
        }
    }

    let ok = errors.is_empty();
    info!(
        "read_settings: completed for '{}' with {} entrie(s); ok={}",
        path.display(),
        values.len(),
        ok
    );
    ok
}

#[cfg(test)]
mod read_settings_filesystem_spec {

    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Write as _;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, info, warn};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bitcoin_settings_read_{nanos}_{suffix}.json"))
    }

    #[traced_test]
    fn nonexistent_file_is_ok_and_yields_empty_map() {
        info!("Verifying that non-existent files are treated as OK");
        let p = unique_path("nonexistent");
        let mut values = HashMap::new();
        let mut errors = Vec::new();
        let ok = read_settings(&p, &mut values, &mut errors);
        debug!("ok={}, values_len={}, errors={:?}", ok, values.len(), errors);
        assert!(ok);
        assert!(values.is_empty());
        assert!(errors.is_empty());
    }

    #[traced_test]
    fn invalid_json_yields_error_and_false() {
        info!("Verifying invalid JSON is rejected");
        let p = unique_path("invalid");
        let mut f = fs::File::create(&p).expect("create file");
        writeln!(f, "not-json").unwrap();
        drop(f);

        let mut values = HashMap::new();
        let mut errors = Vec::new();
        let ok = read_settings(&p, &mut values, &mut errors);
        debug!("ok={}, errors={:?}", ok, errors);
        assert!(!ok);
        assert!(!errors.is_empty());
        let _ = fs::remove_file(&p);
    }

    #[traced_test]
    fn non_object_json_yields_error_and_false() {
        info!("Verifying non-object top-level JSON is rejected");
        let p = unique_path("array");
        let mut f = fs::File::create(&p).expect("create file");
        writeln!(f, "[1,2,3]").unwrap();
        drop(f);

        let mut values = HashMap::new();
        let mut errors = Vec::new();
        let ok = read_settings(&p, &mut values, &mut errors);
        debug!("ok={}, errors={:?}", ok, errors);
        assert!(!ok);
        assert!(!errors.is_empty());
        let _ = fs::remove_file(&p);
    }

    #[traced_test]
    fn valid_object_parsed_into_map() {
        // (read_settings_filesystem_spec) updated to use UniValue::read(raw, size)
        info!("Verifying valid object is parsed into map");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("bitcoin_settings_read_{nanos}_valid.json"));

        let mut f = std::fs::File::create(&p).expect("create file");
        use std::io::Write as _;
        write!(f, "{{\"a\":1,\"b\":true,\"c\":\"str\"}}").unwrap();
        drop(f);

        let mut values = std::collections::HashMap::new();
        let mut errors = Vec::new();
        let ok = read_settings(&p, &mut values, &mut errors);
        debug!("ok={}, values={:?}, errors={:?}", ok, values.keys().collect::<Vec<_>>(), errors);

        assert!(ok);
        assert!(errors.is_empty());
        assert_eq!(values.len(), 3);
        assert_eq!(values.get("a").unwrap().to_string(), "1");
        assert_eq!(values.get("b").unwrap().to_string(), "true");
        assert_eq!(values.get("c").unwrap().to_string(), "\"str\"");
        let _ = std::fs::remove_file(&p);
    }
}

#[cfg(test)]
mod read_settings_duplicate_keys_spec {
    use super::*;
    use std::io::Write as _;
    use std::path::PathBuf;
    use tracing::{info, debug};

    fn unique_path(suffix: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bitcoin_settings_read_dups_{nanos}_{suffix}.json"))
    }

    #[traced_test]
    fn duplicate_object_keys_emit_error_flag_and_last_value_wins() {
        info!("When JSON contains duplicate keys, last value should win and the function should return ok=false with a warning recorded");
        let p = unique_path("dups");
        let mut f = std::fs::File::create(&p).expect("create file");
        // Duplicate key "a" on purpose.
        writeln!(f, "{{\"a\":1,\"a\":2,\"b\":true}}").unwrap();
        drop(f);

        let mut values = std::collections::HashMap::new();
        let mut errors = Vec::new();
        let ok = read_settings(&p, &mut values, &mut errors);
        debug!(
            "ok={}, values={:?}, errors={:?}",
            ok,
            values.keys().collect::<Vec<_>>(),
            errors
        );

        // Contract: we surface duplicate keys as an error -> ok=false
        assert!(!ok, "read should signal not-ok when duplicates are present");
        assert!(errors.iter().any(|e| e.contains("Found duplicate key a")), "expected duplicate key warning in errors");

        // Semantics: last duplicate wins in the resulting map
        assert_eq!(values.len(), 2);
        assert_eq!(values.get("a").unwrap().to_string(), "2");
        assert_eq!(values.get("b").unwrap().to_string(), "true");

        let _ = std::fs::remove_file(&p);
    }
}
