crate::ix!();

impl Logger {

    /// Returns whether logs will be written to any output
    pub fn enabled(&self) -> bool {
        trace!("Logger::enabled => acquiring lock on cs to check logging status");
        let guard = self.cs.borrow();
        let inner = guard.lock();

        let result = *inner.buffering()
            || self.print_to_console
            || self.print_to_file
            || !inner.print_callbacks().is_empty();

        trace!(
            "Logger::enabled => buffering={} print_to_console={} print_to_file={} callbacks_len={} => result={}",
            inner.buffering(),
            self.print_to_console,
            self.print_to_file,
            inner.print_callbacks().len(),
            result
        );
        result
    }
}
