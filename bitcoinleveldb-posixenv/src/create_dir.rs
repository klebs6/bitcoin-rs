// ---------------- [ File: bitcoinleveldb-posixenv/src/create_dir.rs ]
crate::ix!();

impl CreateDir for PosixEnv {

    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            dir = %dirname,
            "PosixEnv::create_dir: creating directory"
        );

        match std::fs::create_dir(dirname) {
            Ok(()) => {
                debug!(
                    dir = %dirname,
                    "PosixEnv::create_dir: directory created successfully"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    dir   = %dirname,
                    errno,
                    "PosixEnv::create_dir: create_dir failed"
                );
                posix_error(dirname, errno)
            }
        }
    }
}

#[cfg(test)]
mod posix_env_create_dir_tests {
    use super::*;

    fn unique_directory_name(suffix: &str) -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-create-dir-{}-{}",
            suffix,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn create_dir_creates_directory_on_filesystem() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let dirname = unique_directory_name("ok");

        let status = env.create_dir(&dirname);

        assert!(
            status.is_ok(),
            "create_dir should succeed for a fresh directory path: {}",
            status.to_string()
        );

        let metadata = std::fs::metadata(&dirname)
            .expect("created directory should exist in the filesystem");

        assert!(
            metadata.is_dir(),
            "created path should be recognized as a directory by the OS"
        );

        let _ = std::fs::remove_dir(&dirname);
    }

    #[traced_test]
    fn create_dir_on_existing_directory_returns_error_status() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let dirname = unique_directory_name("existing");

        std::fs::create_dir(&dirname)
            .expect("precondition: manual create_dir must succeed");

        let status = env.create_dir(&dirname);

        assert!(
            !status.is_ok(),
            "create_dir on an existing directory should return non-OK status"
        );

        let _ = std::fs::remove_dir(&dirname);
    }
}
