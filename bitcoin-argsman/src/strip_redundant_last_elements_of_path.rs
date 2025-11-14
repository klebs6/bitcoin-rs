// ---------------- [ File: bitcoin-argsman/src/strip_redundant_last_elements_of_path.rs ]
crate::ix!();

pub fn strip_redundant_last_elements_of_path(path: &mut PathBuf) {
    let mut result = path.clone();

    while result.file_name() == Some(OsStr::new(".")) {
        result.pop();
    }

    // If paths don't exist yet, is_same_file may return Err — don't crash.
    match is_same_file(&result, &path) {
        Ok(true) => { /* unchanged, great */ }
        Ok(false) => {
            // This should not happen (we only strip trailing ".").
            debug_assert!(false, "strip_redundant_last_elements_of_path altered path");
        }
        Err(_) => {
            // Path may not exist yet; skip the identity check.
        }
    }

    *path = result;
}
