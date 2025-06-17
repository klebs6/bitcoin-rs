// ---------------- [ File: bitcoin-log/src/inner.rs ]
crate::ix!();

#[derive(MutGetters, Getters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct LoggerInner {
    pub(crate) fileout: *mut libc::FILE, // default = nullptr

    pub(crate) msgs_before_open: LinkedList<String>,

    pub(crate) buffering: bool, // default = true

    // Changed from `LinkedList<fn(&String) -> ()>`
    // to `LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>`
    pub(crate) print_callbacks: LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>,
}

#[cfg(test)]
mod logger_inner_tests {
    use super::*;

    /// Tests for `LoggerInner` fields and their getters/setters.
    #[traced_test]
    #[serial]
    fn test_logger_inner_defaults() {
        info!("Testing default state of LoggerInner.");

        let inner = LoggerInner {
            fileout: std::ptr::null_mut(),
            msgs_before_open: LinkedList::new(),
            buffering: true,
            print_callbacks: LinkedList::new(),
        };

        // Check default field values
        assert!(inner.fileout().is_null(), "fileout should be null by default");
        assert!(inner.msgs_before_open().is_empty(), "msgs_before_open should be empty at start");
        assert!(*inner.buffering(), "buffering should be true by default");
        assert!(inner.print_callbacks().is_empty(), "print_callbacks should be empty by default");

        trace!("test_logger_inner_defaults passed.");
    }

    #[traced_test]
    #[serial]
    fn test_logger_inner_mutations() {
        info!("Testing mutations on LoggerInner (fileout, buffering, messages, callbacks).");

        let mut inner = LoggerInner {
            fileout: std::ptr::null_mut(),
            msgs_before_open: LinkedList::new(),
            buffering: true,
            print_callbacks: LinkedList::new(),
        };

        // 1) Set fileout
        let path_cstr = std::ffi::CString::new("logger_inner_tests_mock.log").unwrap();
        let mode = std::ffi::CString::new("w").unwrap();
        let f = unsafe { libc::fopen(path_cstr.as_ptr(), mode.as_ptr()) };
        inner.set_fileout(f);
        assert!(!inner.fileout().is_null(), "fileout should now be non‐null after setting");

        // 2) Toggle buffering => false
        inner.set_buffering(false);
        assert!(!*inner.buffering(), "buffering is now false");

        // 3) Add a message to msgs_before_open
        inner.msgs_before_open_mut().push_back("hello buffer".to_string());
        assert_eq!(
            inner.msgs_before_open().len(),
            1,
            "Should contain exactly one buffered message"
        );

        // 4) Add a callback
        inner.print_callbacks_mut().push_back(Box::new(|msg: &String| {
            debug!("Callback called with: {}", msg);
        }));
        assert_eq!(
            inner.print_callbacks().len(),
            1,
            "Should contain exactly one callback"
        );

        // Cleanup
        if !inner.fileout().is_null() {
            unsafe {
                libc::fclose(*inner.fileout());
            }
            inner.set_fileout(std::ptr::null_mut());
        }

        trace!("test_logger_inner_mutations passed.");
    }
}
