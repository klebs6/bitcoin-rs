crate::ix!();

impl Logger {

    /// Returns whether logs will be written to any output
    pub fn enabled(&self) -> bool {
        let inner = self.cs().lock();
        *inner.buffering()
            || *self.print_to_console()
            || *self.print_to_file()
            || !inner.print_callbacks().is_empty()
    }
}
