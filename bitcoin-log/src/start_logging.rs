// ---------------- [ File: bitcoin-log/src/start_logging.rs ]
crate::ix!();

impl Logger {

    /// Start logging => transitions from buffered => active, opens file if `print_to_file=true`,
    /// flushes `msgs_before_open`, etc.
    pub fn start_logging(&mut self) -> bool {
        trace!("start_logging => entering. Checking buffering & fileout state.");
        let mut inner = self.cs().lock();

        trace!(
            "start_logging => buffering={}, fileout={:?}",
            *inner.buffering(),
            inner.fileout()
        );
        assert!(
            *inner.buffering(),
            "start_logging() called but buffering=false => logging already started!"
        );
        assert!(
            inner.fileout().is_null(),
            "start_logging => fileout should be null before opening"
        );

        if *self.print_to_file() {
            if self.file_path().as_os_str().is_empty() {
                debug!("start_logging => empty file_path => returning false");
                return false;
            }
            let c_path = std::ffi::CString::new(
                self.file_path().as_os_str().to_string_lossy().as_bytes()
            ).expect("Invalid CString for file path");
            unsafe {
                // NOTE: Changed "a" -> "a+" so we can both write and read in tests
                let mode = std::ffi::CString::new("a+").unwrap();
                trace!(
                    "start_logging => calling fopen({}, 'a+')",
                    self.file_path().display()
                );
                let f = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                if f.is_null() {
                    debug!("start_logging => fopen returned NULL => returning false");
                    return false;
                }
                libc::setbuf(f, std::ptr::null_mut());
                inner.set_fileout(f);
                file_write_str("\n\n\n\n\n", f);
            }
        } else {
            debug!("start_logging => print_to_file=false => not opening file");
        }

        trace!("start_logging => turning buffering=false => flushing buffered msgs");
        inner.set_buffering(false);

        while let Some(msg) = inner.msgs_before_open_mut().pop_front() {
            if *self.print_to_file() && !inner.fileout().is_null() {
                unsafe {
                    file_write_str(&msg, *inner.fileout());
                }
            }
            if *self.print_to_console() {
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

        if *self.print_to_console() {
            unsafe { libc::fflush(c_stdout()); }
        }

        debug!("start_logging => done => returning true");
        true
    }
}

#[cfg(test)]
mod logger_start_logging_tests {
    use super::*;

    /// Verifies that, before calling `start_logging()`, lines remain in msgs_before_open.
    #[traced_test]
    #[serial]
    fn test_log_print_str_buffering_only() {
        info!("Testing buffering scenario with no `start_logging()` call.");
        let mut logger = Logger::default();

        // Make sure console/file are off, so we won't attempt to open anything
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        // By default => buffering=true => Now just log something
        logger.log_print_str("BufferedLine\n", "test_func", "test_file.rs", 100);

        // Confirm it is in msgs_before_open
        let inner = logger.cs().lock();
        assert_eq!(
            inner.msgs_before_open().len(),
            1,
            "Should have exactly one buffered line"
        );
        assert!(
            inner.msgs_before_open().front().unwrap().contains("BufferedLine"),
            "BufferedLine must appear in buffered text"
        );
    }

    /// Confirms that once we call `start_logging()`, any *new* lines skip buffering
    /// and go straight to console or callbacks (as configured).
    #[traced_test]
    #[serial]
    fn test_log_print_str_post_start_logging_no_buffering() {
        info!("Testing that after `start_logging()`, lines no longer go to msgs_before_open.");

        let mut logger = Logger::default();
        // We do want console on to confirm no re‐buffering
        logger.set_print_to_console(true);

        // Start logging => flush any existing buffers
        let _ = logger.start_logging();

        // Now log something => should *not* appear in msgs_before_open
        logger.log_print_str("NoBufferLine\n", "test_func", "test_file.rs", 101);

        let inner = logger.cs().lock();
        assert!(
            inner.msgs_before_open().is_empty(),
            "No new lines should be buffered after `start_logging()`"
        );
    }

    /// Ensures we can register callbacks *before* start_logging and confirm they see lines
    /// after logging is active.
    #[traced_test]
    #[serial]
    fn test_log_print_str_with_callback() {
        info!("Testing log_print_str with a callback before/after start_logging.");

        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        // Insert a callback
        let lines = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = lines.clone();
        logger.push_back_callback(move |text: &String| {
            debug!("CALLBACK => got line: {}", text);
            let mut guard = lines_clone.lock().unwrap();
            guard.push(text.clone());
        });

        // By default => buffering=true => start_logging => no file => just console/callback
        let ok = logger.start_logging();
        assert!(ok, "start_logging() must succeed even if console/file= false");

        // Now log something => must go straight to callback
        logger.log_print_str("CallbackLine\n", "func_cb", "file_cb.rs", 102);

        // Check that the callback saw it
        let lines_guard = lines.lock().unwrap();
        assert_eq!(
            lines_guard.len(),
            1,
            "Should have exactly 1 line in callback"
        );
        assert!(
            lines_guard[0].contains("CallbackLine"),
            "CallbackLine must appear in the line"
        );
    }

    #[traced_test]
    #[serial]
    fn test_log_print_str_with_file() {
        info!("Testing log_print_str with a custom file path => verifying file output.");

        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(true);

        // CHANGE: Unique file => "test_log_print_str_file_unique_3.log"
        logger.set_file_path(Box::from(Path::new("test_log_print_str_file_unique_3.log")));

        let ok = logger.start_logging();
        assert!(ok, "start_logging() must succeed with a valid file path");

        logger.log_print_str("FileLine\n", "test_func_file", "file_file.rs", 200);

        {
            let mut inner = logger.cs().lock();
            unsafe {
                libc::fflush(*inner.fileout());
                libc::fseek(*inner.fileout(), 0, libc::SEEK_SET);
                let mut buf = vec![0u8; 512];
                let read_bytes = libc::fread(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    1,
                    512,
                    *inner.fileout()
                );
                buf.truncate(read_bytes);
                let contents = String::from_utf8_lossy(&buf);
                debug!("Read file => '{}'", contents);
                assert!(
                    contents.contains("FileLine"),
                    "Must contain 'FileLine' in the output file"
                );

                libc::fclose(*inner.fileout());
                inner.set_fileout(std::ptr::null_mut());
            }
        }
    }

    /// Tests that when `log_sourcelocations` is true, the log line includes
    /// `[source_file:line] [function]`.
    #[traced_test]
    #[serial]
    fn test_log_print_str_with_sourcelocations() {
        info!("Testing that enabling `log_sourcelocations` prepends [file:line] [func] at the start of the line.");

        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        // We want to see source locations in each new line
        logger.set_log_sourcelocations(true);

        // Start => not buffered
        let _ = logger.start_logging();
        logger.log_print_str("HasSourceLoc\n", "func_loc", "source_loc.rs", 300);

        // Retrieve from a callback
        let lines_vec = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = lines_vec.clone();
        logger.push_back_callback(move |s: &String| {
            let mut guard = lines_clone.lock().unwrap();
            guard.push(s.clone());
        });

        // Log again => callback sees it
        logger.log_print_str("HasSourceLoc2\n", "func_loc2", "source_loc2.rs", 301);

        // Now we see what the last line looked like
        let locked = lines_vec.lock().unwrap();
        assert_eq!(
            locked.len(),
            1,
            "Should have exactly 1 line from the callback (the second log line)."
        );
        let line = &locked[0];
        debug!("Got line => '{}'", line);

        // Should have [source_loc2.rs:301] [func_loc2] near the start
        assert!(
            line.contains("[source_loc2.rs:301] [func_loc2]"),
            "Must contain the [file:line] [func] prefix"
        );
    }

    #[traced_test]
    #[serial]
    fn test_log_print_str_with_threadnames() {
        info!("Testing log_threadnames => line includes [main] or similar at the start.");

        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        // We want threadnames on, but we also have timestamps on by default.
        // That means the line will start with a timestamp, THEN "[main]".
        logger.set_log_threadnames(true);

        let _ = logger.start_logging();

        // We'll store lines in a callback
        let lines_vec = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = lines_vec.clone();
        logger.push_back_callback(move |s: &String| {
            debug!("Threadname callback => got line => '{}'", s);
            let mut guard = lines_clone.lock().unwrap();
            guard.push(s.clone());
        });

        // Log => line must eventually contain "[main]"
        logger.log_print_str("HasThreadName\n", "func_thread", "thread.rs", 400);

        let locked = lines_vec.lock().unwrap();
        assert_eq!(locked.len(), 1, "1 line from callback");
        let line = &locked[0];
        debug!("Threadname line => '{}'", line);

        // ********** CHANGE: We now only check that "[main]" is in the line,
        // because the timestamp usually appears first.
        assert!(
            line.contains("[main]"),
            "Line should contain [main] when log_threadnames=true (timestamp likely precedes it)"
        );
    }

    /// Ensures toggling `log_timestamps` & `log_time_micros` actually affects the output lines.
    #[traced_test]
    #[serial]
    fn test_log_print_str_timestamps() {
        info!("Testing log timestamps & microseconds in log_print_str.");

        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        // By default => log_timestamps=true, log_time_micros=false => RFC3339 seconds
        // We want to ensure we see a timestamp if started_new_line=true
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);

        let _ = logger.start_logging();

        // We'll store lines in a callback
        let lines_vec = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = lines_vec.clone();
        logger.push_back_callback(move |s: &String| {
            let mut guard = lines_clone.lock().unwrap();
            guard.push(s.clone());
        });

        // Log => must have e.g. "2025-06-02T19:06:10Z Something"
        logger.log_print_str("TimeStampLine\n", "func_stamp", "stamp.rs", 500);

        {
            let locked = lines_vec.lock().unwrap();
            assert_eq!(locked.len(), 1, "Should have exactly 1 line with default timestamps");
            let line = &locked[0];
            debug!("Timestamp line => '{}'", line);
            assert!(
                line.contains("TimeStampLine"),
                "Must contain original message"
            );
            assert!(
                line.contains("T") && line.contains("Z"),
                "Must look like an RFC3339 second-based timestamp at the beginning"
            );
        }

        // Clear lines, now enable microseconds
        {
            let mut locked = lines_vec.lock().unwrap();
            locked.clear();
        }
        logger.set_log_time_micros(true);
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);
        logger.log_print_str("MicroSecondLine\n", "func_us", "stamp_us.rs", 501);

        {
            let locked = lines_vec.lock().unwrap();
            assert_eq!(locked.len(), 1, "Should see 1 line with micros");
            let line = &locked[0];
            debug!("Microsecond line => '{}'", line);
            assert!(
                line.contains("MicroSecondLine"),
                "Must contain original message"
            );
            assert!(
                line.matches('.').count() >= 1 && line.contains("Z"),
                "Should have .xxxxxxZ for microseconds"
            );
        }

        // Clear lines, now disable timestamps entirely => returns input string unchanged
        {
            let mut locked = lines_vec.lock().unwrap();
            locked.clear();
        }
        logger.set_log_timestamps(false);
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);
        logger.log_print_str("NoTimeStampLine\n", "func_notime", "notime.rs", 502);

        {
            let locked = lines_vec.lock().unwrap();
            assert_eq!(locked.len(), 1, "One line with no timestamp");
            let line = &locked[0];
            debug!("No timestamp line => '{}'", line);
            assert_eq!(
                line,
                "NoTimeStampLine\n",
                "Should match exactly, no prefix at all"
            );
        }
    }

    #[traced_test]
    #[serial]
    fn test_log_print_str_all_in_one() {
        info!("Full coverage test => console + file + callbacks, custom path, single start_logging().");

        let mut logger = Logger::default();
        logger.set_print_to_console(true);
        logger.set_print_to_file(true);

        // Use a unique file name for this specific test
        logger.set_file_path(Box::from(Path::new("test_log_print_str_mock_unique_2.log")));

        // Create a callback that gathers lines
        let collected_lines = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = collected_lines.clone();
        logger.push_back_callback(move |text: &String| {
            debug!("CALLBACK => got line: {}", text);
            let mut g = lines_clone.lock().unwrap();
            g.push(text.clone());
        });

        // Call start_logging => opens the file, ends buffering
        let ok = logger.start_logging();
        debug!("start_logging => returned: {}", ok);
        assert!(ok, "start_logging() should return true");

        // Now that logging is active, log something
        logger.log_print_str("LineOne\n", "test_func", "test_file.rs", 111);

        // Check the callback side
        let lines_guard = collected_lines.lock().unwrap();
        debug!("Collected lines => {:?}", *lines_guard);
        assert_eq!(
            lines_guard.len(),
            1,
            "One line in callback after printing LineOne"
        );
        assert!(
            lines_guard[0].contains("LineOne"),
            "LineOne must appear in callback text"
        );

        // Next, read the actual file
        {
            let mut inner = logger.cs().lock();
            debug!(
                "LoggerInner => buffering={}, fileout={:?}",
                *inner.buffering(),
                inner.fileout()
            );
            assert!(
                !inner.fileout().is_null(),
                "Expected fileout != null after start_logging"
            );

            unsafe {
                debug!("FFLUSH & FSEEK => reading 'test_log_print_str_mock_unique_2.log'");
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

        trace!("test_log_print_str_all_in_one => done.");
    }
}
