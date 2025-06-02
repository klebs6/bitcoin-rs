crate::ix!();

impl Logger {

    /// Disconnect for testing: re-enable buffering, close file, clear callbacks, etc.
    pub fn disconnect_test_logger(&mut self) {
        let mut inner = self.cs().lock();
        inner.set_buffering(true);

        if !inner.fileout().is_null() {
            unsafe { libc::fclose(*inner.fileout()); }
            inner.set_fileout(std::ptr::null_mut());
        }
        inner.print_callbacks_mut().clear();
        inner.msgs_before_open_mut().clear();
    }
}

#[cfg(test)]
mod logger_disconnect_tests {
    use super::*;

    /// Confirm that `disconnect_test_logger()` re‐enables buffering,
    /// clears callbacks, clears `msgs_before_open`, and closes the file.
    #[traced_test]
    #[serial]
    fn test_disconnect_test_logger() {
        info!("Testing disconnect_test_logger() behavior.");

        let mut logger = Logger::default();
        logger.set_print_to_console(true);
        logger.set_print_to_file(true);

        {
            let mut inner = logger.cs().lock();
            // Simulate an opened file
            let path_cstr = std::ffi::CString::new("test_disconnect_mock.log").unwrap();
            let mode = std::ffi::CString::new("w").unwrap();
            let f = unsafe { libc::fopen(path_cstr.as_ptr(), mode.as_ptr()) };
            inner.set_fileout(f);

            // Add a dummy callback
            inner.print_callbacks_mut().push_back(Box::new(|_| {}));

            // Add a buffered message
            inner.msgs_before_open_mut().push_back("Buffered message".to_string());
        }

        // Disconnect
        logger.disconnect_test_logger();

        // Verify
        let inner = logger.cs().lock();
        assert!(
            *inner.buffering(),
            "disconnect_test_logger must re‐enable buffering"
        );
        assert!(
            inner.fileout().is_null(),
            "disconnect_test_logger must close the file => fileout is null"
        );
        assert!(
            inner.print_callbacks().is_empty(),
            "disconnect_test_logger must clear print_callbacks"
        );
        assert!(
            inner.msgs_before_open().is_empty(),
            "disconnect_test_logger must clear msgs_before_open"
        );

        trace!("test_disconnect_test_logger passed.");
    }
}
