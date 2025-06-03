crate::ix!();

pub fn log_instance() -> &'static mut Logger {
    /**
     * We create a single global Logger instance
     * that is leaked on exit. This imitates the C++ code's
     * approach of "static global pointer, never freed."
     */
    use std::sync::Once;

    static mut LOGGER_PTR: *mut Logger = std::ptr::null_mut();
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            // Construct a default logger
            let logger = Logger::default();
            // Leak it by turning it into a raw pointer
            LOGGER_PTR = Box::into_raw(Box::new(logger));
        });
        &mut *LOGGER_PTR
    }
}

#[cfg(test)]
mod log_instance_tests {
    use super::*;

    /// Verifies that `log_instance()` always returns the same global Logger pointer.
    #[traced_test]
    #[serial]
    fn test_log_instance_persistence() {
        info!("Testing that log_instance() always returns the same pointer.");

        let logger1 = log_instance();
        let logger2 = log_instance();
        assert!(
            std::ptr::eq(logger1, logger2),
            "log_instance() must return the same logger pointer every time"
        );

        trace!("test_log_instance_persistence passed.");
    }

    #[traced_test]
    #[serial]
    fn test_log_instance_default_state() {
        info!("Testing the default state of the global logger from log_instance().");

        // Forcefully reset the global logger so it's in the *original* default state
        let logger = log_instance();
        logger.disconnect_test_logger();

        // Now check expectations
        {
            let inner = logger.cs().lock();
            assert!(
                *inner.buffering(),
                "Default global logger => buffering should be true"
            );
            assert!(
                inner.fileout().is_null(),
                "fileout should be null by default in the global logger"
            );
        }

        let category_mask = logger.get_category_mask();
        assert_eq!(
            category_mask, 0,
            "Default global logger => categories() should be 0 (NONE)"
        );

        trace!("test_log_instance_default_state passed.");
    }

    /// Tests changing state on the global logger and retrieving it again to confirm persistent changes.
    #[traced_test]
    #[serial]
    fn test_log_instance_state_changes() {
        info!("Testing state changes on the global logger persist across calls to log_instance().");

        {
            let logger = log_instance();
            // Enable the NET category
            logger.enable_category_with_flags(LogFlags::NET);
        }

        {
            // Retrieve the logger again and confirm the category remains enabled
            let logger_again = log_instance();
            assert!(
                logger_again.will_log_category(LogFlags::NET),
                "NET category must remain enabled across calls to log_instance()"
            );

            // Cleanup: disable NET
            logger_again.disable_category_with_flags(LogFlags::NET);
        }

        trace!("test_log_instance_state_changes passed.");
    }
}
