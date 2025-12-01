// ---------------- [ File: bitcoinleveldb-posixenv/src/new_writable_file.rs ]
crate::ix!();

impl NewWritableFile for PosixEnv {

    fn new_writable_file(
        &mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::new_writable_file";

        trace!(
            file = %filename,
            "PosixEnv::new_writable_file: opening writable file (truncate)"
        );

        initialize_posix_env_result_slot::<dyn WritableFile>(CALLER, result);

        let flags = libc::O_TRUNC | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
        let mode: libc::mode_t = 0o644;

        let fd = match open_posix_file_descriptor(CALLER, filename, flags, mode) {
            Ok(fd) => fd,
            Err(status) => return status,
        };

        let wf = PosixWritableFile::new(filename.clone(), fd);
        let inner: Box<dyn WritableFile> = Box::new(wf);

        store_posix_env_boxed_result::<dyn WritableFile>(CALLER, result, inner)
    }
}

#[cfg(test)]
mod posix_env_new_writable_file_tests {
    use super::*;

    fn unique_writable_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-new-writable-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn new_writable_file_creates_or_truncates_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_writable_file_path();

        std::fs::write(&filename, b"existing-data")
            .expect("precondition: write should succeed");

        let mut handle: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let status = env.new_writable_file(
            &filename,
            &mut handle as *mut *mut Box<dyn WritableFile>,
        );

        assert!(
            status.is_ok(),
            "new_writable_file should succeed: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "new_writable_file must populate out-parameter with non-null handle"
        );

        unsafe {
            let boxed: Box<Box<dyn WritableFile>> = Box::from_raw(handle);
            drop(boxed);
        }

        let metadata = std::fs::metadata(&filename)
            .expect("writable file should exist after creation");

        assert_eq!(
            metadata.len(),
            0,
            "new_writable_file should truncate any existing file contents"
        );

        let _ = std::fs::remove_file(&filename);
    }
}
