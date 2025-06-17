// ---------------- [ File: bitcoin-log/src/log_callbacks.rs ]
crate::ix!();

impl Logger {

    /// Insert a new print-callback (generic) so it can accept closures capturing environment.
    pub fn push_back_callback<F>(&mut self, fun: F)
    where
        F: Fn(&String) + Send + Sync + 'static,
    {
        let mut inner = self.cs().lock();
        inner.print_callbacks_mut().push_back(Box::new(fun));
    }

    /// Remove *all* callbacks
    pub fn delete_callback(&mut self) {
        let mut inner = self.cs().lock();
        inner.print_callbacks_mut().clear();
    }
}

#[cfg(test)]
mod logger_push_back_delete_callback_tests {
    use super::*;
    use traced_test::traced_test;
    use serial_test::serial;
    use tracing::*;

    /// Tests `push_back_callback` adds callbacks to the logger,
    /// and `delete_callback` removes them all.
    #[traced_test]
    #[serial]
    fn test_push_back_and_delete_callback() {
        info!("Testing push_back_callback and delete_callback on Logger.");

        let mut logger = Logger::default();
        {
            let inner = logger.cs().lock();
            assert!(
                inner.print_callbacks().is_empty(),
                "New logger must have no callbacks"
            );
        }

        // 1) push_back_callback => add two callbacks
        logger.push_back_callback(|msg: &String| {
            debug!("First callback => got: {}", msg);
        });
        logger.push_back_callback(|msg: &String| {
            debug!("Second callback => got: {}", msg);
        });

        {
            let inner = logger.cs().lock();
            assert_eq!(
                inner.print_callbacks().len(),
                2,
                "Should have exactly two callbacks now"
            );
        }

        // 2) delete_callback => remove them
        logger.delete_callback();
        {
            let inner = logger.cs().lock();
            assert!(
                inner.print_callbacks().is_empty(),
                "Callbacks should be cleared"
            );
        }

        trace!("test_push_back_and_delete_callback passed.");
    }
}
