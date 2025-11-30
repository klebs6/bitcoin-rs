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
