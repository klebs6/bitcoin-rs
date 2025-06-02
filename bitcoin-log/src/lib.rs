// ---------------- [ File: bitcoin-log/src/lib.rs ]
#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate static_assertions;

extern crate libc;

#[macro_use] mod imports; use imports::*;

x!{c_stdout}
x!{category}
x!{disconnect_test_logger}
x!{enabled}
x!{flags}
x!{format}
x!{inner}
x!{interface}
x!{log_callbacks}
x!{log_categories_list}
x!{log_print_str}
x!{log_timestamp_str}
x!{logger}
x!{shrink_debug_file}
x!{start_logging}
x!{timer}
x!{trace}
x!{util}

#[cfg(test)]
mod bitcoin_logger_tests {
    use super::*;

    // ----------------------------------
    // 1) Tests for Log Categories
    // ----------------------------------
    #[traced_test]
    fn test_get_log_category_empty_string_returns_all() {
        info!("Testing get_log_category with empty string => LogFlags::ALL");
        let mut flag = LogFlags::NONE;
        let ok = get_log_category(&mut flag, &"".to_string());
        assert!(ok, "get_log_category should succeed for empty string");
        assert_eq!(flag, LogFlags::ALL, "Empty string => ALL");
        trace!("test_get_log_category_empty_string_returns_all passed.");
    }

    #[traced_test]
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
    fn test_get_log_category_unknown() {
        info!("Testing get_log_category with unknown category => should fail");
        let mut flag = LogFlags::NONE;
        let bad = get_log_category(&mut flag, &"randomstuff".to_string());
        assert!(!bad, "Should fail for unknown category");
        // The `flag` remains unchanged:
        assert_eq!(flag, LogFlags::NONE, "Flag remains NONE on failure");
        trace!("test_get_log_category_unknown passed.");
    }

    // ----------------------------------
    // 2) Test log_accept_category
    // ----------------------------------
    #[traced_test]
    fn test_log_accept_category() {
        info!("Testing log_accept_category calls log_instance().will_log_category");
        // By default, log_instance() => categories = 0 (LogFlags::NONE)
        // So net => false
        assert_eq!(log_accept_category(LogFlags::NET), false, "Should not accept 'NET' by default");

        // Enable NET
        {
            let logger = log_instance();
            logger.enable_category_with_flags(LogFlags::NET);
        }
        assert_eq!(log_accept_category(LogFlags::NET), true, "Now we accept 'NET' after enabling it");

        // Disable NET
        {
            let logger = log_instance();
            logger.disable_category_with_flags(LogFlags::NET);
        }
        assert_eq!(log_accept_category(LogFlags::NET), false, "Disabled again => false");

        trace!("test_log_accept_category passed.");
    }

    #[traced_test]
    fn test_logger_basic_behavior() {
        info!("Testing basic Logger creation and category manipulation.");

        let logger = log_instance();

        // Start fresh
        {
            let mut guard = logger.cs().borrow_mut();
            let mut inner = guard.lock();
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

    #[traced_test]
    fn test_logger_buffering() {
        info!("Testing logger buffering until start_logging is called.");

        let logger = log_instance();
        // Reset
        {
            let mut guard = logger.cs().borrow_mut();
            let mut inner = guard.lock();
            inner.set_buffering(true);
            inner.msgs_before_open_mut().clear();
        }

        // If we log something now, it goes to msgs_before_open
        logger.log_print_str("Hello from buffer\n", "test_func", "test_file.rs", 123);

        {
            let guard = logger.cs().borrow();
            let inner = guard.lock();
            assert!(
                !inner.msgs_before_open().is_empty(),
                "Message should be buffered"
            );
        }

        // Now call start_logging() => it should flush
        let ok = logger.start_logging();
        assert!(ok, "start_logging() should succeed if debug.log can open or is disabled.");

        {
            let guard = logger.cs().borrow();
            let inner = guard.lock();
            assert_eq!(inner.msgs_before_open().len(), 0, "Should have flushed the buffer");
            assert!(!inner.buffering(), "buffering = false after start_logging");
        }

        trace!("test_logger_buffering passed.");
    }

    #[traced_test]
    fn test_logger_print_callback() {
        info!("Testing logger's print callback mechanism.");

        let logger = log_instance();
        // Reset
        {
            let mut guard = logger.cs().borrow_mut();
            let mut inner = guard.lock();
            inner.set_buffering(false);
            inner.msgs_before_open_mut().clear();
            inner.print_callbacks_mut().clear();
        }

        // We'll store logs in a shared Vec
        let logs_collected = Arc::new(StdMutex::new(Vec::<String>::new()));
        let logs_clone = logs_collected.clone();

        // Create a closure capturing logs_clone => must store as Box<dyn Fn(&String) + ...>
        let cb = Box::new(move |msg: &String| {
            let mut guard = logs_clone.lock().unwrap();
            guard.push(msg.clone());
        });

        // Register the callback
        {
            let mut guard = logger.cs().borrow_mut();
            let mut inner = guard.lock();
            inner.print_callbacks_mut().push_back(cb);
        }

        // Now print
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
    fn test_timer_lifetime_message() {
        info!("Testing Timer RAII logging.");
        let logger = log_instance();

        // We'll check logs in real time with a callback:
        let lines = Arc::new(StdMutex::new(Vec::<String>::new()));
        let lines_clone = lines.clone();

        // Register a callback that captures log lines
        {
            let mut guard = logger.cs().borrow_mut();
            let mut inner = guard.lock();
            // Clear prior callbacks
            inner.print_callbacks_mut().clear();

            let cb = move |msg: &String| {
                let mut g = lines_clone.lock().unwrap();
                g.push(msg.clone());
            };
            inner.print_callbacks_mut().push_back(Box::new(cb));
        }

        // We'll create a scope so the Timer's Drop is triggered inside:
        {
            let _timer = TimerBuilder::default()
                .start_t(Instant::now())
                .prefix("test_timer_lifetime_message".to_string())
                .title("Sample Timer".to_string())
                .log_category(LogFlags::ALL)
                .build()
                .unwrap();

            // On creation => logs "Sample Timer started"
            // On drop => logs "Sample Timer completed"
        }

        let locked_lines = lines.lock().unwrap();
        assert!(
            locked_lines.iter().any(|line| line.contains("Sample Timer started")),
            "Should have a line with 'started'"
        );
        assert!(
            locked_lines.iter().any(|line| line.contains("Sample Timer completed")),
            "Should have a line with 'completed'"
        );

        trace!("test_timer_lifetime_message passed.");
    }

    // ----------------------------------
    // 5) Tests for Trace Macros
    // ----------------------------------
    #[traced_test]
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
}
