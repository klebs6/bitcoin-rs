crate::ix!();

impl Logger {

    /// Disconnect for testing: re-enable buffering, close file, clear callbacks, etc.
    pub fn disconnect_test_logger(&mut self) {
        trace!("Logger::disconnect_test_logger => resetting logger to initial buffering state");

        let guard = self.cs.borrow_mut();
        let mut inner = guard.lock();

        inner.set_buffering(true);

        if !inner.fileout().is_null() {
            unsafe { libc::fclose(*inner.fileout()); }
            inner.set_fileout(std::ptr::null_mut());
        }

        inner.print_callbacks_mut().clear();
        inner.msgs_before_open_mut().clear();
        trace!("Logger::disconnect_test_logger => done");
    }
}
