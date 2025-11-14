// ---------------- [ File: bitcoin-remote/src/generate_auth_cookie.rs ]
crate::ix!();

/// Generate a brand‑new cookie and *atomically* write it to disk.  
///
/// Returns **true** on success and populates `cookie_out` with the newly‑minted value.
///
pub fn generate_auth_cookie(cookie_out: &mut String) -> bool {

    const COOKIE_SIZE: i32 = 32;

    // 1. Create random password
    let mut rand_pwd = [0u8; COOKIE_SIZE as usize];
    get_rand_bytes(rand_pwd.as_mut_slice(), COOKIE_SIZE);

    let cookie: String = format!(
        "{}:{}",
        COOKIEAUTH_USER,
        hex::encode(rand_pwd)
    );

    // 2. Write to temporary file first
    let filepath_tmp = get_auth_cookie_file(Some(true));
    if let Err(e) = {
        if let Some(parent) = filepath_tmp.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                error!(%e, "Unable to create parent directory for cookie");
                return false;
            }
        }
        let mut file = match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            // rely on process umask as per Core comment
            .open(&filepath_tmp)
        {
            Ok(f) => f,
            Err(e) => {
                error!(
                    %e,
                    "Unable to open cookie authentication file {:?} for writing",
                    filepath_tmp
                );
                return false;
            }
        };
        file.write_all(cookie.as_bytes())
            .and_then(|_| file.flush())
    } {
        error!(
            %e,
            "Failed to write temporary cookie file {:?}",
            filepath_tmp
        );
        // best‑effort clean‑up
        let _ = fs::remove_file(&filepath_tmp);
        return false;
    }

    // 3. Atomically rename to final destination
    let filepath_final = get_auth_cookie_file(Some(false));
    if let Err(e) = fs::rename(&filepath_tmp, &filepath_final) {
        error!(
            %e,
            "Unable to rename cookie authentication file {:?} to {:?}",
            filepath_tmp, filepath_final
        );
        let _ = fs::remove_file(&filepath_tmp);
        return false;
    }

    info!(
        "Generated RPC authentication cookie at {:?}",
        filepath_final
    );

    *cookie_out = cookie;
    true
}

#[cfg(test)]
mod tests_generate_auth_cookie_behavior {
    use super::*;
    use std::fs;
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

    fn assert_cookie_format(cookie: &str) {
        let mut parts = cookie.splitn(2, ':');
        let user = parts.next().unwrap_or("");
        let pass = parts.next().unwrap_or("");
        debug!(%user, %pass, "Validating cookie format");
        assert_eq!(user, COOKIEAUTH_USER, "Cookie must start with the fixed username");
        assert_eq!(pass.len(), 64, "Password hex must be 64 chars (32 bytes)");
        assert!(
            pass.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
            "Password must be lowercase hex"
        );
    }

    #[traced_test]
    fn successful_generation_persists_cookie_and_matches_roundtrip() {
        with_cookie_override("gen_cookie.roundtrip.cookie", |_expected_path| {
            let mut generated = String::new();
            assert!(generate_auth_cookie(&mut generated), "Generation should succeed");
            assert_cookie_format(&generated);

            let mut read_back = String::new();
            assert!(get_auth_cookie(&mut read_back), "Reading back cookie should succeed");
            debug!(generated=%generated, read_back=%read_back, "Comparing generated vs read-back");
            assert_eq!(generated, read_back, "Generated and read-back cookie must match");

            let final_path = get_auth_cookie_file(None);
            debug!(final_path=?final_path, exists=%final_path.exists(), "Checking on-disk presence");
            assert!(final_path.exists(), "Final cookie file must exist after generation");
        });
    }

    #[traced_test]
    fn successful_generation_removes_tmp_file() {
        with_cookie_override("gen_cookie.no_tmp_leftover.cookie", |base| {
            let mut _cookie = String::new();
            assert!(generate_auth_cookie(&mut _cookie), "Generation should succeed");

            let tmp_path = append_tmp_suffix(&base);
            debug!(tmp_path=?tmp_path, exists=%tmp_path.exists(), "Checking temporary file removal");
            assert!(!tmp_path.exists(), "Temporary cookie file must not remain after atomic rename");
        });
    }

    #[traced_test]
    fn generation_creates_missing_parent_directories() {
        with_cookie_override("nested/a/b/c/d/gen_cookie.deep.cookie", |_unused| {
            let mut cookie = String::new();
            assert!(generate_auth_cookie(&mut cookie), "Generation should succeed and create parents");

            let final_path = get_auth_cookie_file(None);
            debug!(final_path=?final_path, exists=%final_path.exists(), "Verifying deep parent creation and presence");
            assert!(final_path.exists(), "Final cookie file must exist in a newly created directory tree");

            let mut read_back = String::new();
            assert!(get_auth_cookie(&mut read_back), "Reading back cookie should succeed from deep path");
            assert_eq!(cookie, read_back, "Cookie contents must round-trip");
        });
    }

    #[traced_test]
    fn generation_fails_when_final_path_is_directory() {
        with_cookie_override("gen_cookie.final_is_dir.cookie", |final_path| {
            // Pre-create a directory occupying the final path to force rename failure.
            fs::create_dir_all(&final_path).expect("create dir at final path");
            let tmp_path = append_tmp_suffix(&final_path);

            let mut cookie = String::new();
            let ok = generate_auth_cookie(&mut cookie);
            debug!(?final_path, ?tmp_path, ok, "Generation result when final path is a directory");
            assert!(!ok, "Generation must fail if final path is a directory");

            // The temporary file must be cleaned up on failure.
            assert!(!tmp_path.exists(), "Temporary file must be removed on rename failure");

            // Reading as a cookie should fail since a directory sits at the final path.
            let mut out = String::new();
            assert!(
                !get_auth_cookie(&mut out),
                "get_auth_cookie should fail when final path is a directory"
            );
        });
    }

    #[traced_test]
    fn generation_fails_when_parent_component_is_file() {
        // Global serialisation across the entire crate's test suite.
        let _guard = crate::auth_cookie::GLOBAL_TEST_SERIAL
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let dir = TempDir::new().expect("tempdir");
        let parent_file = dir.path().join("parent_as_file");
        fs::write(&parent_file, b"not-a-directory").expect("create parent-as-file");

        // Compose a path whose parent contains a file component.
        let final_path = dir
            .path()
            .join("parent_as_file")
            .join("child")
            .join("gen_cookie.bad_parent.cookie");

        {
            let mut g = G_ARGS.lock();
            g.force_set_arg("-rpccookiefile", final_path.to_str().unwrap());
        }

        let tmp_path = append_tmp_suffix(&final_path);

        let mut cookie = String::new();
        let ok = generate_auth_cookie(&mut cookie);
        debug!(?final_path, ?tmp_path, ok, "Generation result when parent component is a file");
        assert!(!ok, "Generation must fail when a parent path element is a file");

        // We should not have left any temporary file behind.
        assert!(
            !tmp_path.exists(),
            "No temporary file should be created/left when parent creation fails"
        );
    }
}
