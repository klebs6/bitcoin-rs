// ---------------- [ File: bitcoin-remote/src/auth_cookie.rs ]
crate::ix!();

/**
  | Username used when cookie authentication
  | is in use (arbitrary, only for recognizability
  | in debugging/logging purposes)
  |
  */
pub const COOKIEAUTH_USER: &'static str = "__cookie__";

/**
  | Default name for auth cookie file
  |
  */
pub const COOKIEAUTH_FILE: &'static str = ".cookie";

/// Resolve the absolute path for the *RPC‑auth* cookie file.
///
/// If `temp == true` the function appends **`.tmp`** to the
/// filename, mirroring Core’s strategy of writing to a
/// temporary file first and then renaming atomically.
pub fn get_auth_cookie_file(temp: Option<bool>) -> Box<Path> {
    let mut name: String = {
        // Honour `-rpccookiefile=<path>` if provided, otherwise
        // fall back to the default.
        let g_args = G_ARGS.lock().expect("ArgsManager poisoned");
        g_args.get_arg("-rpccookiefile", COOKIEAUTH_FILE)
    };

    if temp.unwrap_or(false) {
        name.push_str(".tmp");
    }

    let path = abs_path_for_config_val(&PathBuf::from(&name));

    trace!(path = ?path, "Computed auth‑cookie path");
    path.into_boxed_path()
}

#[cfg(test)]
mod tests_auth_cookie {
    use super::*;
    use std::sync::Mutex;
    use tempfile::TempDir;

    /// Serialise tests that manipulate the global `G_ARGS`.
    static SERIAL: Mutex<()> = Mutex::new(());

    fn with_temp_cookie<F: FnOnce()>(f: F) {
        let _guard = SERIAL.lock().unwrap();
        let dir = TempDir::new().expect("tempdir");
        {
            let mut g = G_ARGS.lock().unwrap();
            g.force_set_arg("-rpccookiefile", dir.path().join("test.cookie").to_str().unwrap());
        }
        f();
        // tempdir dropped -> files removed
    }

    #[traced_test]
    fn generate_and_read_roundtrip() {
        with_temp_cookie(|| {
            let mut cookie = String::new();
            assert!(
                generate_auth_cookie(&mut cookie),
                "cookie generation failed"
            );
            assert!(
                cookie.starts_with(COOKIEAUTH_USER),
                "generated cookie malformed"
            );

            let mut read_back = String::new();
            assert!(
                get_auth_cookie(&mut read_back),
                "cookie read‑back failed"
            );
            assert_eq!(cookie, read_back, "mismatching cookie contents");
        });
    }

    #[traced_test]
    fn delete_cookie_works() {
        with_temp_cookie(|| {
            let mut _c = String::new();
            assert!(generate_auth_cookie(&mut _c));
            let path = get_auth_cookie_file(None);
            assert!(path.exists(), "cookie file should exist pre‑delete");
            delete_auth_cookie();
            assert!(!path.exists(), "cookie file should be gone");
        });
    }

    #[traced_test]
    fn get_cookie_missing_returns_false() {
        with_temp_cookie(|| {
            delete_auth_cookie(); // ensure missing
            let mut out = String::new();
            assert!(
                !get_auth_cookie(&mut out),
                "expected failure for missing cookie"
            );
        });
    }
}
