crate::ix!();

impl Logger {

    /// Low-level function that prints a log message. We handle all 
    /// "timestamp / sourceloc / threading" prefix logic **before** locking.
    pub fn log_print_str(
        &mut self,
        msg: &str,
        logging_function: &str,
        source_file: &str,
        source_line: i32
    ) {
        // 1) Escape suspicious chars (no lock needed)
        let mut escaped = log_escape_message(msg);

        // 2) Check if it's a "new line" => we read from `started_new_line` below
        let started_line = self
            .started_new_line
            .load(std::sync::atomic::Ordering::Relaxed);

        // 3) Possibly insert source locations
        if self.log_sourcelocations && started_line {
            let shortened = remove_prefix(source_file, "./");
            let prefix = format!("[{}:{}] [{}] ", shortened, source_line, logging_function);
            escaped.insert_str(0, &prefix);
        }

        // 4) Possibly insert threadname
        if self.log_threadnames && started_line {
            let thread_name = "main"; // placeholder
            let prefix = format!("[{}] ", thread_name);
            escaped.insert_str(0, &prefix);
        }

        // 5) Possibly add timestamp
        // Make `log_timestamp_str` take `&self` instead of `&mut self`, 
        // so we can call it here without a second mutable borrow.
        let final_msg = self.log_timestamp_str(&escaped);

        // 6) Figure out whether it ends with a newline
        let ends_newline = final_msg.ends_with('\n');
        self.started_new_line
            .store(ends_newline, std::sync::atomic::Ordering::Relaxed);

        // 7) Now lock the logger to see if we are buffering or printing
        let guard = self.cs.borrow_mut();
        let mut inner = guard.lock();

        if *inner.buffering() {
            trace!("Logger::log_print_str => currently buffering => push_back to msgs_before_open");
            inner.msgs_before_open_mut().push_back(final_msg);
            return;
        }

        // Not buffering => write to console if enabled
        if self.print_to_console {
            unsafe {
                libc::fwrite(
                    final_msg.as_ptr() as *const libc::c_void,
                    1,
                    final_msg.len(),
                    c_stdout()
                );
                libc::fflush(c_stdout());
            }
        }

        // Now all callbacks
        for cb in inner.print_callbacks().iter() {
            cb(&final_msg);
        }

        // Possibly write to file
        if self.print_to_file {
            if inner.fileout().is_null() {
                error!("Logger::log_print_str => print_to_file but fileout is null? ignoring.");
                return;
            }
            let was_reopen = self
                .reopen_file
                .swap(false, std::sync::atomic::Ordering::Relaxed);
            if was_reopen {
                unsafe {
                    let c_path = std::ffi::CString::new(self.file_path.as_os_str().to_string_lossy().as_bytes())
                        .expect("Invalid CString for file path");
                    let mode = std::ffi::CString::new("a").unwrap();
                    let new_file = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                    if !new_file.is_null() {
                        libc::setbuf(new_file, std::ptr::null_mut());
                        libc::fclose(*inner.fileout());
                        inner.set_fileout(new_file);
                        trace!("Logger::log_print_str => reopened log file successfully");
                    } else {
                        error!("Logger::log_print_str => reopen attempt failed => continuing with old file");
                    }
                }
            }
            unsafe {
                file_write_str(&final_msg, *inner.fileout());
            }
        }
    }
}
