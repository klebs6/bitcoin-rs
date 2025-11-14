// ---------------- [ File: bitcoin-remote/src/get_auth_cookie_file.rs ]
crate::ix!();

/// Resolve the absolute path for the *RPC‑auth* cookie file.
///
/// If `temp == true` the function appends **`.tmp`** to the
/// filename, mirroring Core’s strategy of writing to a
/// temporary file first and then renaming atomically.
pub fn get_auth_cookie_file(temp: Option<bool>) -> Box<Path> {
    let mut name: String = {
        // Honour `-rpccookiefile=<path>` if provided, otherwise
        // fall back to the default.
        let g_args = G_ARGS.lock();
        g_args.get_arg("-rpccookiefile", COOKIEAUTH_FILE)
    };

    if temp.unwrap_or(false) {
        name.push_str(".tmp");
    }

    let path = abs_path_for_config_val(&PathBuf::from(&name), Some(false));

    trace!(path = ?path, "Computed auth‑cookie path");
    path.into_boxed_path()
}

#[cfg(test)]
mod tests_get_auth_cookie_file_behavior {
    use super::*;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    fn append_tmp_suffix<P: AsRef<Path>>(p: P) -> PathBuf {
        let mut s = p.as_ref().as_os_str().to_owned();
        s.push(".tmp");
        PathBuf::from(s)
    }

    fn with_cookie_override<F: FnOnce(PathBuf)>(file_name: &str, f: F) {
        // Global serialisation across the entire crate's test suite.
        let _guard = crate::auth_cookie::GLOBAL_TEST_SERIAL
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let dir = TempDir::new().expect("tempdir");
        let cookie_path = dir.path().join(file_name);
        {
            let mut g = G_ARGS.lock();
            g.force_set_arg("-rpccookiefile", cookie_path.to_str().unwrap());
        }
        debug!(?cookie_path, "Set -rpccookiefile for test");
        f(cookie_path);
        // `TempDir` drops here and cleans up
    }

    #[traced_test]
    fn absolute_override_yields_exact_path_without_tmp() {
        with_cookie_override("get_file.override.cookie", |expected| {
            let actual = get_auth_cookie_file(None);
            debug!(?expected, actual=?actual, "Validating absolute override path without .tmp");
            assert_eq!(
                actual.as_ref(),
                expected.as_path(),
                "Override path must be honored exactly"
            );
        });
    }

    #[traced_test]
    fn tmp_flag_appends_suffix_to_absolute_override() {
        with_cookie_override("get_file.tmpflag.cookie", |base| {
            let want_tmp = append_tmp_suffix(&base);
            let got_tmp = get_auth_cookie_file(Some(true));
            debug!(want_tmp=?want_tmp, got_tmp=?got_tmp, "Validating .tmp suffix behavior");
            assert_eq!(
                got_tmp.as_ref(),
                want_tmp.as_path(),
                ".tmp suffix must be appended verbatim"
            );
        });
    }

    #[traced_test]
    fn tmp_none_semantics_match_false() {
        with_cookie_override("get_file.none_vs_false.cookie", |_base| {
            let p_none = get_auth_cookie_file(None);
            let p_false = get_auth_cookie_file(Some(false));
            debug!(p_none=?p_none, p_false=?p_false, "Comparing None vs Some(false) semantics");
            assert_eq!(
                p_none.as_ref(),
                p_false.as_ref(),
                "None and Some(false) must be equivalent"
            );
        });
    }
}
