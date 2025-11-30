// ---------------- [ File: bitcoinleveldb-posixenv/src/new_random_access_file.rs ]
crate::ix!();

impl NewRandomAccessFile for PosixEnv {
 
    fn new_random_access_file(
        &mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn RandomAccessFile>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::new_random_access_file";

        trace!(
            file = %filename,
            "PosixEnv::new_random_access_file: opening random-access file"
        );

        initialize_posix_env_result_slot::<dyn RandomAccessFile>(CALLER, result);

        let flags = libc::O_RDONLY | OPEN_BASE_FLAGS;

        let fd = match open_posix_file_descriptor(CALLER, filename, flags, 0) {
            Ok(fd) => fd,
            Err(status) => return status,
        };

        if !self.mmap_limiter_mut().acquire() {
            debug!(
                file = %filename,
                fd,
                "PosixEnv::new_random_access_file: mmap limiter exhausted; \
                 using fd-based random access file"
            );

            let ra = PosixRandomAccessFile::new(
                filename.clone(),
                fd,
                &mut *self.fd_limiter_mut() as *mut Limiter,
            );
            let inner: Box<dyn RandomAccessFile> = Box::new(ra);

            return store_posix_env_boxed_result::<dyn RandomAccessFile>(
                CALLER,
                result,
                inner,
            );
        }

        let mut file_size: u64 = 0;
        let size_status = self.get_file_size(filename, &mut file_size);

        if !size_status.is_ok() {
            warn!(
                file   = %filename,
                status = %size_status.to_string(),
                "PosixEnv::new_random_access_file: GetFileSize failed"
            );

            unsafe {
                libc::close(fd);
            }

            self.mmap_limiter_mut().release();

            return size_status;
        }

        let length = file_size as usize;

        debug!(
            file   = %filename,
            fd,
            length,
            "PosixEnv::new_random_access_file: attempting mmap()"
        );

        if length == 0 {
            debug!(
                file = %filename,
                fd,
                "PosixEnv::new_random_access_file: zero-length file; \
                 falling back to fd-based random access file"
            );

            self.mmap_limiter_mut().release();

            let ra = PosixRandomAccessFile::new(
                filename.clone(),
                fd,
                &mut *self.fd_limiter_mut() as *mut Limiter,
            );
            let inner: Box<dyn RandomAccessFile> = Box::new(ra);

            return store_posix_env_boxed_result::<dyn RandomAccessFile>(
                CALLER,
                result,
                inner,
            );
        }

        let mmap_base = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                length,
                libc::PROT_READ,
                libc::MAP_SHARED,
                fd,
                0,
            )
        };

        if mmap_base == libc::MAP_FAILED {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);

            warn!(
                file  = %filename,
                fd,
                errno,
                "PosixEnv::new_random_access_file: mmap() failed"
            );

            unsafe {
                libc::close(fd);
            }

            self.mmap_limiter_mut().release();

            return posix_error(filename, errno);
        }

        unsafe {
            libc::close(fd);
        }

        debug!(
            file   = %filename,
            length,
            "PosixEnv::new_random_access_file: mmap() succeeded"
        );

        let mmap_file = PosixMmapReadableFile::new(
            filename.clone(),
            mmap_base as *mut u8,
            length,
            &mut *self.mmap_limiter_mut() as *mut Limiter,
        );

        let inner: Box<dyn RandomAccessFile> = Box::new(mmap_file);

        store_posix_env_boxed_result::<dyn RandomAccessFile>(CALLER, result, inner)
    }
}

#[cfg(test)]
mod posix_env_new_random_access_file_tests {
    use super::*;

    fn unique_random_access_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-new-random-access-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn new_random_access_file_opens_existing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_random_access_file_path();

        let contents = b"random-access-payload";
        std::fs::write(&filename, contents)
            .expect("precondition: write should succeed");

        let mut handle: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

        let status = env.new_random_access_file(
            &filename,
            &mut handle as *mut *mut Box<dyn RandomAccessFile>,
        );

        assert!(
            status.is_ok(),
            "new_random_access_file should succeed for an existing file: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "new_random_access_file must populate the out-parameter with a non-null handle"
        );

        unsafe {
            let boxed: Box<Box<dyn RandomAccessFile>> = Box::from_raw(handle);
            drop(boxed);
        }

        let _ = std::fs::remove_file(&filename);
    }
}
