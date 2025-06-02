crate::ix!();

impl Logger {

    pub fn log_print_str(
        &mut self,
        msg: &str,
        logging_function: &str,
        source_file: &str,
        source_line: i32
    ) {
        let mut escaped = log_escape_message(msg);

        let started_line = self
            .started_new_line()
            .load(std::sync::atomic::Ordering::Relaxed);

        if *self.log_sourcelocations() && started_line {
            let shortened = remove_prefix(source_file, "./");
            let prefix = format!("[{}:{}] [{}] ", shortened, source_line, logging_function);
            escaped.insert_str(0, &prefix);
        }

        if *self.log_threadnames() && started_line {
            let thread_name = "main"; // placeholder
            let prefix = format!("[{}] ", thread_name);
            escaped.insert_str(0, &prefix);
        }

        let final_msg = self.log_timestamp_str(&escaped);
        let ends_newline = final_msg.ends_with('\n');
        self.started_new_line()
            .store(ends_newline, std::sync::atomic::Ordering::Relaxed);

        let mut inner = self.cs().lock();

        if *inner.buffering() {
            inner.msgs_before_open_mut().push_back(final_msg);
            return;
        }

        if *self.print_to_console() {
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

        for cb in inner.print_callbacks().iter() {
            cb(&final_msg);
        }

        if *self.print_to_file() {
            if inner.fileout().is_null() {
                return;
            }
            let was_reopen = self
                .reopen_file()
                .swap(false, std::sync::atomic::Ordering::Relaxed);
            if was_reopen {
                unsafe {
                    let c_path = std::ffi::CString::new(
                        self.file_path().as_os_str().to_string_lossy().as_bytes()
                    ).expect("Invalid CString for file path");
                    let mode = std::ffi::CString::new("a").unwrap();
                    let new_file = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                    if !new_file.is_null() {
                        libc::setbuf(new_file, std::ptr::null_mut());
                        libc::fclose(*inner.fileout());
                        inner.set_fileout(new_file);
                    }
                }
            }
            unsafe {
                file_write_str(&final_msg, *inner.fileout());
            }
        }
    }
}

#[cfg(test)]
mod logger_log_print_str_tests {
    use super::*;

    /// Exhaustively tests `log_print_str` behavior with buffering, console, file, and callbacks.
    #[traced_test]
    #[serial]
    fn test_log_print_str() {
        info!("Testing log_print_str in various scenarios.");

        let mut logger = Logger::default();

        // 1) If buffering=true, the message should go to msgs_before_open, not console/file/callback.
        logger.set_print_to_console(true); // We'll test that it does NOT actually print while buffering
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(true);
        }
        let test_msg = "Hello from buffer!\n";
        logger.log_print_str(test_msg, "test_func", "test_file.rs", 123);

        {
            let inner = logger.cs().lock();
            assert_eq!(
                inner.msgs_before_open().len(),
                1,
                "Message should be buffered"
            );
            let buffered_line = inner.msgs_before_open().front().unwrap();
            assert!(
                buffered_line.contains("Hello from buffer!"),
                "Buffered line must contain original text"
            );
        }

        // 2) Turn off buffering => subsequent calls go to console, callbacks, and file if set.
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(false);
        }

        // 2a) Enable console => check that the next printed line doesn't get buffered
        logger.set_print_to_console(true);

        // We'll intercept the console output by toggling the environment if needed,
        // but let's just ensure it doesn't go to msgs_before_open anymore.
        logger.log_print_str("Now console gets this\n", "test_func", "test_file.rs", 456);
        {
            let inner = logger.cs().lock();
            assert!(
                inner.msgs_before_open().is_empty(),
                "No new lines should be buffered now"
            );
        }

        // 2b) Register a callback => confirm it's invoked
        let collected_lines = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let lines_clone = collected_lines.clone();
        logger.push_back_callback(move |text: &String| {
            let mut guard = lines_clone.lock().unwrap();
            guard.push(text.clone());
        });

        logger.log_print_str("Line to callback\n", "test_func_cb", "test_file.rs", 999);

        {
            let lines_guard = collected_lines.lock().unwrap();
            assert_eq!(
                lines_guard.len(),
                1,
                "Callback must have received exactly one line"
            );
            assert!(
                lines_guard[0].contains("Line to callback"),
                "Callback text must match"
            );
        }

        // 2c) Enable file => create a dummy file to confirm it writes out
        logger.set_print_to_file(true);
        {
            let path_cstr = std::ffi::CString::new("logger_log_print_str_mock.log").unwrap();
            let mode = std::ffi::CString::new("w").unwrap();
            let f = unsafe { libc::fopen(path_cstr.as_ptr(), mode.as_ptr()) };

            {
                let mut inner = logger.cs().lock();
                inner.set_fileout(f);
            }

            logger.log_print_str("File line\n", "test_func_file", "test_file.rs", 777);

            // Confirm the line was written to file
            {
                // Rewind and read
                let mut inner = logger.cs().lock();
                unsafe {
                    libc::fflush(*inner.fileout());
                    libc::fseek(*inner.fileout(), 0, libc::SEEK_SET);
                    let mut buffer = vec![0u8; 1024];
                    let read_bytes = libc::fread(
                        buffer.as_mut_ptr() as *mut libc::c_void,
                        1,
                        1024,
                        *inner.fileout()
                    );
                    buffer.truncate(read_bytes);
                    let contents = String::from_utf8_lossy(&buffer);

                    assert!(
                        contents.contains("File line"),
                        "Log line must appear in file contents"
                    );
                }
            }

            // Cleanup
            {
                let mut inner = logger.cs().lock();
                if !inner.fileout().is_null() {
                    unsafe { libc::fclose(*inner.fileout()); }
                    inner.set_fileout(std::ptr::null_mut());
                }
            }
        }

        trace!("test_log_print_str passed.");
    }
}
