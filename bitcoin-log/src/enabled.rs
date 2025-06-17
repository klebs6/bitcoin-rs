// ---------------- [ File: bitcoin-log/src/enabled.rs ]
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

#[cfg(test)]
mod logger_enabled_tests {
    use super::*;

    /// Exhaustively check `enabled()` returns `true` or `false` as expected,
    /// depending on buffering, console/file booleans, and presence of callbacks.
    #[traced_test]
    #[serial]
    fn test_logger_enabled() {
        info!("Testing Logger::enabled() in various scenarios.");

        let mut logger = Logger::default();

        // 1) Everything false/empty => but buffering=true by default => enabled()=true
        assert!(logger.enabled(), "Default logger => buffering=true => enabled()=true.");

        // 2) Turn buffering off
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(false);
        }
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);
        {
            let inner = logger.cs().lock();
            assert!(
                inner.print_callbacks().is_empty(),
                "Default logger has no callbacks"
            );
        }
        assert!(
            !logger.enabled(),
            "No buffering, no console, no file, no callbacks => enabled()=false"
        );

        // 3) Turn on console => enabled()=true
        logger.set_print_to_console(true);
        assert!(
            logger.enabled(),
            "print_to_console=true => enabled()=true"
        );

        // 4) Turn off console, turn on print_to_file => enabled()=true
        logger.set_print_to_console(false);
        logger.set_print_to_file(true);
        assert!(
            logger.enabled(),
            "print_to_file=true => enabled()=true"
        );

        // 5) Turn off file, add a callback => enabled()=true
        logger.set_print_to_file(false);
        {
            let mut inner = logger.cs().lock();
            inner.print_callbacks_mut().push_back(Box::new(|msg: &String| {
                debug!("A dummy callback: {}", msg);
            }));
        }
        assert!(
            logger.enabled(),
            "No buffering, no console, no file, but callback => enabled()=true"
        );

        trace!("test_logger_enabled passed.");
    }
}
