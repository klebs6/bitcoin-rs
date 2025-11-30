// ---------------- [ File: bitcoinleveldb-posixenv/src/new_sequential_file.rs ]
crate::ix!();

 impl NewSequentialFile for PosixEnv {

    fn new_sequential_file(
        &mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn SequentialFile>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::new_sequential_file";

        trace!(
            file = %filename,
            "PosixEnv::new_sequential_file: opening sequential file"
        );

        initialize_posix_env_result_slot::<dyn SequentialFile>(CALLER, result);

        let flags = libc::O_RDONLY | OPEN_BASE_FLAGS;

        let fd = match open_posix_file_descriptor(CALLER, filename, flags, 0) {
            Ok(fd) => fd,
            Err(status) => return status,
        };

        let seq = PosixSequentialFile::new(filename.clone(), fd);
        let inner: Box<dyn SequentialFile> = Box::new(seq);

        store_posix_env_boxed_result::<dyn SequentialFile>(CALLER, result, inner)
    }
}

#[cfg(test)]
mod posix_env_new_sequential_file_tests {
    use super::*;

    fn unique_sequential_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-new-sequential-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn new_sequential_file_opens_existing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_sequential_file_path();

        std::fs::write(&filename, b"sequential-data")
            .expect("precondition: write should succeed");

        let mut handle: *mut Box<dyn SequentialFile> = std::ptr::null_mut();

        let status = env.new_sequential_file(
            &filename,
            &mut handle as *mut *mut Box<dyn SequentialFile>,
        );

        assert!(
            status.is_ok(),
            "new_sequential_file should succeed: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "new_sequential_file must populate out-parameter with non-null handle"
        );

        unsafe {
            let boxed: Box<Box<dyn SequentialFile>> = Box::from_raw(handle);
            drop(boxed);
        }

        let _ = std::fs::remove_file(&filename);
    }
}
