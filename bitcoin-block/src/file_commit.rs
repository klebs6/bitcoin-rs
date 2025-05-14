// ---------------- [ File: bitcoin-block/src/file_commit.rs ]
crate::ix!();

/**
  | Ensure file contents are fully committed
  | to disk, using a platform-specific
  | feature analogous to fsync().
  |
  */
pub fn file_commit(file: *mut libc::FILE) -> bool {

    unsafe {

        if libc::fflush(file) != 0 {

            // harmless if redundantly called
            log_printf!{
                "{}: fflush failed: {}\n",
                func,
                errno
            };

            return false;
        }

        #[cfg(WIN32)]
        {
            let h_file: HANDLE = get_osfhandle(fileno(file)) as HANDLE;

            if flush_file_buffers(h_file) == 0 {

                log_printf!{
                    "{}: FlushFileBuffers failed: {}\n", 
                    func, 
                    get_last_error()
                };

                return false;
            }

            return true;
        }

        #[cfg(all(MAC_OSX,F_FULLFSYNC))]
        {
            if fcntl(fileno(file),fullfsync,0) == -1 {

                // Manpage says "value other than -1"
                // is returned on success
                log_printf!{
                    "{}: fcntl F_FULLFSYNC failed: {}\n",
                    func,
                    errno
                };

                return false;
            }

            return true;
        }

        #[cfg(HAVE_FDATASYNC)]
        {
            if fdatasync(fileno(file)) != 0 && errno != einval {

                // Ignore EINVAL for filesystems that
                // don't support sync
                log_printf!{
                    "{}: fdatasync failed: {}\n",
                    func,
                    errno
                };

                return false;
            }

            return true;
        }

        if libc::fsync(libc::fileno(file)) != 0 && errno().0 != libc::EINVAL {

            log_printf!{
                "{}: fsync failed: {}\n", 
                func, 
                errno
            };

            return false;
        }
    }

    true
}
