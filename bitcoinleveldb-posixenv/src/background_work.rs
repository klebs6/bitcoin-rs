// ---------------- [ File: bitcoinleveldb-posixenv/src/background_work.rs ]
crate::ix!();

#[derive(Setters,Getters,MutGetters)]
#[getset(set="pub",get="pub",get_mut="pub")]
pub struct PosixEnvBackgroundWork {
    background_work_cv:        Condvar,
    started_background_thread: bool,
    background_work_queue:     SegQueue<PosixEnvBackgroundWorkItem>,
}

impl Default for PosixEnvBackgroundWork {
    fn default() -> Self {
        trace!(
            "PosixEnvBackgroundWork::default: initializing background work state"
        );

        Self {
            background_work_cv:        Condvar::new(),
            started_background_thread: false,
            background_work_queue:     SegQueue::new(),
        }
    }
}

/**
  | Stores the work item data in a Schedule()
  | call.
  |
  | Instances are constructed on the thread
  | calling Schedule() and used on the background
  | thread.
  |
  | This structure is thread-safe because it is
  | immutable.
  */
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct PosixEnvBackgroundWorkItem {
    function: fn(*mut c_void) -> c_void,
    arg:      *mut c_void,
}

impl PosixEnvBackgroundWorkItem {

    pub fn new(
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void
    ) -> Self {
        trace!(
            func = ?(function as *const ()),
            arg  = ?arg,
            "PosixEnvBackgroundWorkItem::new: creating background work item"
        );

        Self {
            function,
            arg,
        }
    }
}

#[cfg(test)]
mod posix_env_background_work_tests {
    use super::*;

    fn noop_background_function(_arg: *mut c_void) -> c_void {
        trace!("noop_background_function: invoked in test");
        unsafe { std::mem::zeroed() }
    }

    #[traced_test]
    fn background_work_default_initializes_empty_queue_and_flags() {
        let work = PosixEnvBackgroundWork::default();

        assert!(
            !work.started_background_thread(),
            "newly created PosixEnvBackgroundWork must not report started background thread"
        );

        assert!(
            work.background_work_queue().is_empty(),
            "newly created PosixEnvBackgroundWork must have an empty work queue"
        );
    }

    #[traced_test]
    fn background_work_item_stores_function_and_argument() {
        let arg_box = Box::new(42u8);
        let arg_ptr = Box::into_raw(arg_box) as *mut c_void;

        let item = PosixEnvBackgroundWorkItem::new(noop_background_function, arg_ptr);

        let stored_function = *item.function();
        let stored_arg      = *item.arg();

        assert!(
            stored_function as *const () == noop_background_function as *const (),
            "PosixEnvBackgroundWorkItem must store the provided callback function pointer"
        );

        assert!(
            stored_arg == arg_ptr,
            "PosixEnvBackgroundWorkItem must store the provided opaque argument pointer"
        );

        unsafe {
            drop(Box::from_raw(arg_ptr as *mut u8));
        }
    }
}
