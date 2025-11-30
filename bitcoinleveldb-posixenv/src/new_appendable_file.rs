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
