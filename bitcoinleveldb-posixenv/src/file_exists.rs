// ---------------- [ File: bitcoinleveldb-posixenv/src/file_exists.rs ]
crate::ix!();

impl FileExists for PosixEnv {

    fn file_exists(&mut self, filename: &String) -> bool {
        let exists = std::fs::metadata(filename).is_ok();
        trace!(
            file   = %filename,
            exists,
            "PosixEnv::file_exists: checked file existence"
        );
        exists
    }
}

#[cfg(test)]
mod posix_env_file_exists_tests {
    use super::*;

    fn unique_file_for_existence_check() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-file-exists-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn file_exists_reports_true_for_existing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_file_for_existence_check();

        std::fs::write(&filename, b"probe")
            .expect("precondition: write should succeed");

        assert!(
            env.file_exists(&filename),
            "file_exists should return true for a file that exists"
        );

        let _ = std::fs::remove_file(&filename);
    }

    #[traced_test]
    fn file_exists_reports_false_for_missing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_file_for_existence_check();

        assert!(
            !env.file_exists(&filename),
            "file_exists should return false for a path that does not exist"
        );
    }
}
