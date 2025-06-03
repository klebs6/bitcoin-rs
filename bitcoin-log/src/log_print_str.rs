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

    #[traced_test]
    #[serial]
    fn test_log_print_str() {
        info!("Testing log_print_str with a custom file path.");

        let mut logger = Logger::default();
        logger.set_print_to_console(true);
        logger.set_print_to_file(true);

        // CHANGE: Use a *unique* filename so no concurrency issues or leftover data
        logger.set_file_path(Box::from(Path::new("test_log_print_str_mock_unique_1.log")));

        // We'll collect lines from the callback
        let collected_lines = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = collected_lines.clone();
        logger.push_back_callback(move |text: &String| {
            debug!("CALLBACK => got line: {}", text);
            let mut g = lines_clone.lock().unwrap();
            g.push(text.clone());
        });

        // start_logging => opens the file
        debug!("About to call start_logging => should open 'test_log_print_str_mock_unique_1.log' in append");
        let ok = logger.start_logging();
        debug!("start_logging => returned: {}", ok);
        assert!(ok, "start_logging() should return true");

        // Now that logging is active, log something
        logger.log_print_str("LineOne\n", "test_func", "test_file.rs", 111);

        // Check the callback
        let lines_guard = collected_lines.lock().unwrap();
        debug!("Collected lines => {:?}", *lines_guard);
        assert_eq!(lines_guard.len(), 1, "One line in callback after printing LineOne");
        assert!(
            lines_guard[0].contains("LineOne"),
            "LineOne must appear in callback text"
        );

        // Next, read the actual file
        {
            let mut inner = logger.cs().lock();
            debug!("LoggerInner => buffering={}, fileout={:?}",
                *inner.buffering(),
                inner.fileout()
            );
            assert!(
                !inner.fileout().is_null(),
                "Expected fileout != null after start_logging"
            );

            unsafe {
                debug!("FFLUSH & FSEEK => reading 'test_log_print_str_mock_unique_1.log'");
                libc::fflush(*inner.fileout());
                libc::fseek(*inner.fileout(), 0, libc::SEEK_SET);
                let mut buffer = vec![0u8; 512];
                let read_bytes = libc::fread(
                    buffer.as_mut_ptr() as *mut libc::c_void,
                    1,
                    512,
                    *inner.fileout()
                );
                buffer.truncate(read_bytes);
                let contents = String::from_utf8_lossy(&buffer);
                debug!("File contents => '{}'", contents);

                assert!(
                    contents.contains("LineOne"),
                    "File must contain 'LineOne'"
                );

                // Cleanup
                libc::fclose(*inner.fileout());
                inner.set_fileout(std::ptr::null_mut());
            }
        }

        trace!("test_log_print_str => done.");
    }
}
