// ---------------- [ File: bitcoin-log/src/lib.rs ]
#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

extern crate libc;

#[macro_use] mod imports; use imports::*;

x!{c_stdout}
x!{category}
x!{defaults}
x!{disconnect_test_logger}
x!{enabled}
x!{escape_message}
x!{flags}
x!{format}
x!{inner}
x!{instance}
x!{linked_list_ext}
x!{log_callbacks}
x!{log_categories_list}
x!{log_print_str}
x!{log_timestamp_str}
x!{logger}
x!{printf}
x!{shrink_debug_file}
x!{start_logging}
x!{timer}
x!{trace}
x!{util}

#[cfg(test)]
mod bitcoin_logger_tests {
    use super::*;

    // Helper: create a brand-new Logger instance (not the global!)
    fn make_test_logger() -> Logger {
        let mut logger = Logger::default();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(true);
            inner.msgs_before_open_mut().clear();
            inner.print_callbacks_mut().clear();
        }
        logger
    }

    // ----------------------------------
    // 1) Tests for Log Categories
    // ----------------------------------
    #[traced_test]
    #[serial]
    fn test_get_log_category_empty_string_returns_all() {
        info!("Testing get_log_category with empty string => LogFlags::ALL");
        let mut flag = LogFlags::NONE;
        let ok = get_log_category(&mut flag, &"".to_string());
        assert!(ok, "get_log_category should succeed for empty string");
        assert_eq!(flag, LogFlags::ALL, "Empty string => ALL");
        trace!("test_get_log_category_empty_string_returns_all passed.");
    }

    #[traced_test]
    #[serial]
    fn test_get_log_category_known_values() {
        info!("Testing get_log_category with known category strings.");
        let mut flag = LogFlags::NONE;

        // For example, "net" => LogFlags::NET
        let ok_net = get_log_category(&mut flag, &"net".to_string());
        assert!(ok_net, "Parsing 'net' should succeed");
        assert_eq!(flag, LogFlags::NET, "'net' => LogFlags::NET");

        // "tor" => LogFlags::TOR
        let ok_tor = get_log_category(&mut flag, &"tor".to_string());
        assert!(ok_tor, "Parsing 'tor' should succeed");
        assert_eq!(flag, LogFlags::TOR, "'tor' => LogFlags::TOR");

        // "1" => LogFlags::ALL
        let ok_all = get_log_category(&mut flag, &"1".to_string());
        assert!(ok_all, "Parsing '1' should succeed");
        assert_eq!(flag, LogFlags::ALL, "'1' => LogFlags::ALL");

        trace!("test_get_log_category_known_values passed.");
    }

    #[traced_test]
    #[serial]
    fn test_get_log_category_unknown() {
        info!("Testing get_log_category with unknown category => should fail");
        let mut flag = LogFlags::NONE;
        let bad = get_log_category(&mut flag, &"randomstuff".to_string());
        assert!(!bad, "Should fail for unknown category");
        // The `flag` remains unchanged:
        assert_eq!(flag, LogFlags::NONE, "Flag remains NONE on failure");
        trace!("test_get_log_category_unknown passed.");
    }

    #[traced_test]
    #[serial]
    fn test_log_accept_category() {
        info!("Testing log_accept_category calls the global log_instance().will_log_category.");

        // First, ensure the global logger has no categories set:
        log_instance().categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // Confirm 'NET' is initially disabled:
        assert_eq!(
            log_accept_category(LogFlags::NET),
            false,
            "Should not accept 'NET' by default"
        );

        // Enable NET on the *global logger*:
        {
            let logger = log_instance();
            logger.enable_category_with_flags(LogFlags::NET);
        }
        assert_eq!(
            log_accept_category(LogFlags::NET),
            true,
            "Now we accept 'NET' after enabling it"
        );

        // Disable NET on the *global logger*:
        {
            let logger = log_instance();
            logger.disable_category_with_flags(LogFlags::NET);
        }
        assert_eq!(
            log_accept_category(LogFlags::NET),
            false,
            "Disabled again => no longer accept 'NET'"
        );

        trace!("test_log_accept_category passed.");
    }

    #[traced_test]
    #[serial]
    fn test_logger_basic_behavior() {
        info!("Testing basic Logger creation and category manipulation.");

        let mut logger = make_test_logger();

        // Start fresh
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(true);
            inner.msgs_before_open_mut().clear();
        }

        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // By default => no categories
        assert_eq!(logger.get_category_mask(), 0, "No categories initially");

        // Enable 'mempool'
        logger.enable_category_with_flags(LogFlags::MEMPOOL);
        assert_ne!(logger.get_category_mask(), 0, "Now have some categories set");
        assert!(
            logger.will_log_category(LogFlags::MEMPOOL),
            "Should log 'MEMPOOL' now"
        );

        // Disable 'mempool'
        logger.disable_category_with_flags(LogFlags::MEMPOOL);
        assert_eq!(
            logger.will_log_category(LogFlags::MEMPOOL),
            false,
            "Disabled => no longer logs"
        );

        trace!("test_logger_basic_behavior passed.");
    }


    // ----------------------------------
    // 5) Tests for Trace Macros
    // ----------------------------------
    #[traced_test]
    #[serial]
    fn test_trace_macros_compile() {
        info!("Testing trace macros compile & run without error. They do nothing unless ENABLE_TRACING is set.");

        // If compiled with tracing disabled, these macros won't do anything. But let's just ensure they run:
        trace0!("my_context", "my_event");
        trace1!("my_context", "my_event", 123u64);
        trace2!("my_context", "my_event", 123u64, true);

        // No panic => success
        trace!("trace macros test completed");
        trace!("test_trace_macros_compile passed");
    }

    #[traced_test]
    #[serial]
    fn test_logger_print_callback() {
        info!("Testing logger's print callback mechanism.");

        let mut logger = make_test_logger();

        // Reset to a known state
        logger.disconnect_test_logger();
        logger.set_print_to_console(false);
        logger.set_print_to_file(false);

        {
            let mut inner = logger.cs().lock();
            // Make sure we start in buffered mode so that `start_logging()` can transition
            // to non-buffered.
            inner.set_buffering(true);
            inner.msgs_before_open_mut().clear();
            inner.print_callbacks_mut().clear();
        }

        // We'll store logs in a shared Vec
        let logs_collected = Arc::new(StdMutex::new(Vec::<String>::new()));
        let logs_clone = logs_collected.clone();

        // Create a closure capturing logs_clone => store as Box<dyn Fn(&String) + ...>
        let cb = Box::new(move |msg: &String| {
            let mut guard = logs_clone.lock().unwrap();
            guard.push(msg.clone());
        });

        // Register the callback
        {
            let mut inner = logger.cs().lock();
            inner.print_callbacks_mut().push_back(cb);
        }

        // Now call start_logging() => transitions from buffering to active logging
        let _ = logger.start_logging();

        // Print something => it should go directly to the callback
        logger.log_print_str("Callback test line\n", "test_func", "test_file.rs", 999);

        // Confirm callback saw it
        let lines = logs_collected.lock().unwrap();
        assert_eq!(lines.len(), 1, "Should have exactly 1 log line from callback");
        assert!(
            lines[0].contains("Callback test line"),
            "Line must contain the correct text"
        );

        trace!("test_logger_print_callback passed.");
    }

    #[traced_test]
    #[serial]
    fn test_logger_buffering() {
        let mut logger = make_test_logger();

        // Now your test is guaranteed a fresh logger, no concurrency issues:
        {
            let mut inner = logger.cs().lock();
            inner.set_buffering(true);
        }
        logger.log_print_str("Hello from buffer\n", "test_func", "test_file.rs", 123);
        {
            let inner = logger.cs().lock();
            assert!(!inner.msgs_before_open().is_empty(), "Message should be buffered");
        }
        let ok = logger.start_logging();
        assert!(ok, "start_logging() should succeed");
        {
            let inner = logger.cs().lock();
            assert_eq!(inner.msgs_before_open().len(), 0);
            assert!(!inner.buffering(), "buffering = false after start_logging");
        }
    }

    #[traced_test]
    #[serial]
    fn test_timer_lifetime_message() {
        info!("Testing timer_lifetime_message with the global logger.");

        // 1) Reset the global logger
        let global_logger = log_instance();
        trace!("Calling disconnect_test_logger() => resetting global_logger");
        global_logger.disconnect_test_logger();
        global_logger.set_print_to_console(false);
        global_logger.set_print_to_file(false);

        // 2) Insert a callback
        let lines = Arc::new(StdMutex::new(Vec::<String>::new()));
        {
            let lines_clone = lines.clone();
            let cb = move |msg: &String| {
                debug!("GLOBAL_LOGGER CALLBACK => Received line: {}", msg);
                let mut guard = lines_clone.lock().unwrap();
                guard.push(msg.clone());
            };
            let mut inner = global_logger.cs().lock();
            trace!("Installing callback, ensuring buffering=true for now");
            inner.print_callbacks_mut().push_back(Box::new(cb));
            inner.set_buffering(true);
        }

        // 3) start_logging => no longer buffered
        trace!("Calling start_logging() => transitioning from buffered to active");
        let ok = global_logger.start_logging();
        debug!("start_logging => returned: {}", ok);

        // 4) Create the Timer => calls new => logs "<title> started"
        trace!("Constructing Timer => it will log 'Sample Timer started'");
        {
            let _timer = TimerBuilder::default()
                .start_t(Instant::now())
                .prefix("test_timer_lifetime_message".to_string())
                .title("Sample Timer".to_string())
                .log_category(LogFlags::ALL)
                .build()
                .unwrap();
            trace!("Timer created => Will drop at scope end => logs 'Sample Timer completed'");
        }

        // 5) Confirm we see both “Sample Timer started” & “... completed”
        let lines_guard = lines.lock().unwrap();
        debug!("LINES COLLECTED => {:?}", *lines_guard);

        assert!(
            lines_guard.iter().any(|line| line.contains("Sample Timer started")),
            "Should have a line with 'Sample Timer started'"
        );
        assert!(
            lines_guard.iter().any(|line| line.contains("Sample Timer completed")),
            "Should have a line with 'Sample Timer completed'"
        );

        trace!("test_timer_lifetime_message passed.");
    }
}
