// ---------------- [ File: bitcoinleveldb-posixenv/src/rename_file.rs ]
crate::ix!();

impl RenameFile for PosixEnv {

    fn rename_file(
        &mut self,
        from: &String,
        to:   &String,
    ) -> crate::Status {
        trace!(
            from = %from,
            to   = %to,
            "PosixEnv::rename_file: renaming file"
        );

        match std::fs::rename(from, to) {
            Ok(()) => {
                debug!(
                    from = %from,
                    to   = %to,
                    "PosixEnv::rename_file: rename succeeded"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    from  = %from,
                    to    = %to,
                    errno,
                    "PosixEnv::rename_file: rename failed"
                );
                posix_error(from, errno)
            }
        }
    }
}

#[cfg(test)]
mod posix_env_rename_file_tests {
    use super::*;

    fn unique_rename_source_and_target() -> (String, String) {
        let base = std::env::temp_dir();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let src = base
            .join(format!(
                "bitcoinleveldb-posixenv-rename-src-{}",
                stamp
            ))
            .to_string_lossy()
            .to_string();
        let dst = base
            .join(format!(
                "bitcoinleveldb-posixenv-rename-dst-{}",
                stamp
            ))
            .to_string_lossy()
            .to_string();
        (src, dst)
    }

    #[traced_test]
    fn rename_file_moves_file_to_new_path() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let (src, dst) = unique_rename_source_and_target();

        std::fs::write(&src, b"rename-me")
            .expect("precondition: write should succeed");

        let status = env.rename_file(&src, &dst);

        assert!(
            status.is_ok(),
            "rename_file should succeed: {}",
            status.to_string()
        );

        assert!(
            std::fs::metadata(&src).is_err(),
            "source path should not exist after successful rename"
        );
        assert!(
            std::fs::metadata(&dst).is_ok(),
            "destination path must exist after successful rename"
        );

        let _ = std::fs::remove_file(&dst);
    }
}
