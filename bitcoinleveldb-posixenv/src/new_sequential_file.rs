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
