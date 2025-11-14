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

#[cfg(test)]
lazy_static! {
    /// Global serialiser for tests that manipulate the shared `G_ARGS` state.
    /// This prevents inter-test races across *all* modules in this crate.
    pub static ref GLOBAL_TEST_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

#[cfg(test)]
mod tests_auth_cookie {
    use super::*;
    use tempfile::TempDir;

    fn with_temp_cookie<F: FnOnce()>(f: F) {
        // Global serialisation across the entire crate's test suite.
        let _guard = crate::auth_cookie::GLOBAL_TEST_SERIAL
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let dir = TempDir::new().expect("tempdir");
        {
            let mut g = G_ARGS.lock();
            g.force_set_arg(
                "-rpccookiefile",
                dir.path().join("test.cookie").to_str().unwrap(),
            );
        }
        debug!(path = ?dir.path(), "Configured -rpccookiefile for test");
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
            assert!(get_auth_cookie(&mut read_back), "cookie read‑back failed");
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
