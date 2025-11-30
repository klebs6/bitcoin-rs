// ---------------- [ File: bitcoinleveldb-posixenv/src/new_appendable_file.rs ]
crate::ix!();

impl NewAppendableFile for PosixEnv {

    fn new_appendable_file(
        &mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::new_appendable_file";

        trace!(
            file = %filename,
            "PosixEnv::new_appendable_file: opening appendable file"
        );

        initialize_posix_env_result_slot::<dyn WritableFile>(CALLER, result);

        let flags = libc::O_APPEND | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
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
mod posix_env_new_appendable_file_tests {
    use super::*;

    fn unique_appendable_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-new-appendable-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn new_appendable_file_creates_and_opens_file_for_appending() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_appendable_file_path();

        let mut handle: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let status = env.new_appendable_file(
            &filename,
            &mut handle as *mut *mut Box<dyn WritableFile>,
        );

        assert!(
            status.is_ok(),
            "new_appendable_file should succeed for a new file: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "new_appendable_file must populate the out-parameter with a non-null handle"
        );

        unsafe {
            let boxed: Box<Box<dyn WritableFile>> = Box::from_raw(handle);
            drop(boxed);
        }

        assert!(
            std::fs::metadata(&filename).is_ok(),
            "appendable file must exist on disk after creation"
        );

        let _ = std::fs::remove_file(&filename);
    }
}
