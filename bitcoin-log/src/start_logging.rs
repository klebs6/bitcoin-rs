crate::ix!();

impl Logger {

    /// Start actual logging (open file if needed, flush any buffered messages).
    pub fn start_logging(&mut self) -> bool {
        trace!("Logger::start_logging => acquiring lock and attempting to open file if needed");

        let guard = self.cs.borrow_mut();
        let mut inner = guard.lock();

        // Must still be buffering:
        assert!(
            inner.buffering(),
            "start_logging() called but buffering=false => logging already started!"
        );
        assert!(
            inner.fileout().is_null(),
            "start_logging => fileout should be null before opening"
        );

        if self.print_to_file {
            trace!("Logger::start_logging => Attempting to open the log file in append mode");
            if self.file_path.as_os_str().is_empty() {
                error!("Logger::start_logging => file_path is empty but print_to_file=true => fail");
                return false;
            }
            let c_path = std::ffi::CString::new(self.file_path.as_os_str().to_string_lossy().as_bytes())
                .expect("Invalid CString for file path");
            unsafe {
                let mode = std::ffi::CString::new("a").unwrap();
                let f = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                if f.is_null() {
                    error!("Logger::start_logging => failed to open file {:?}", self.file_path);
                    return false;
                }
                libc::setbuf(f, std::ptr::null_mut());
                inner.set_fileout(f);

                // Distinguish from any previous run
                file_write_str("\n\n\n\n\n", f);
            }
        }

        let queued_count = inner.msgs_before_open().len();
        trace!("Logger::start_logging => flushing buffered messages: count={}", queued_count);

        inner.set_buffering(false); // we stop buffering now

        while let Some(msg) = inner.msgs_before_open_mut().pop_front() {
            if self.print_to_file {
                unsafe {
                    file_write_str(&msg, *inner.fileout());
                }
            }
            if self.print_to_console {
                unsafe {
                    libc::fwrite(
                        msg.as_ptr() as *const libc::c_void,
                        1,
                        msg.len(),
                        c_stdout()
                    );
                    libc::fflush(c_stdout());
                }
            }
            for cb in inner.print_callbacks().iter() {
                cb(&msg);
            }
        }
        if self.print_to_console {
            unsafe { libc::fflush(c_stdout()); }
        }

        trace!("Logger::start_logging => completed with success=true");
        true
    }
}
