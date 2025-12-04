// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env.rs ]
crate::ix!();

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct InMemoryEnvInner {
    file_map: InMemoryEnvFileSystem,
}

/**
  | Map from filenames to FileState objects,
  | representing a simple file system.
  |
  */
pub type InMemoryEnvFileSystem = HashMap<String,*mut FileState>;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.cc]

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct InMemoryEnv {
    base:     EnvWrapper,
    mutex:    Mutex<InMemoryEnvInner>,
}

impl Drop for InMemoryEnv {

    fn drop(&mut self) {
        trace!("InMemoryEnv::drop: cleaning up in‑memory file system");

        // We have &mut self here, so we can bypass locking with get_mut().
        let inner: &mut InMemoryEnvInner = self.mutex.get_mut();

        let file_count = inner.file_map.len();
        debug!(
            "InMemoryEnv::drop: {} file(s) remain in file_map; unref'ing all",
            file_count
        );

        for (fname, file_ptr) in inner.file_map.drain() {
            debug!(
                "InMemoryEnv::drop: Unref FileState for '{}' (ptr={:?})",
                fname, file_ptr
            );
            unsafe {
                FileState::unref_raw(file_ptr);
            }
        }
    }
}

impl InMemoryEnv {

    pub fn new(base_env: Rc<RefCell<dyn Env>>) -> Self {
        trace!("InMemoryEnv::new: constructing in‑memory env");
        InMemoryEnv {
            base: EnvWrapper::new(base_env),
            mutex: Mutex::new(InMemoryEnvInner {
                file_map: InMemoryEnvFileSystem::new(),
            }),
        }
    }

    #[inline]
    pub(crate) fn inner_mutex(&self) -> &Mutex<InMemoryEnvInner> {
        &self.mutex
    }

    #[inline]
    pub(crate) fn inner_mutex_mut(&mut self) -> &mut Mutex<InMemoryEnvInner> {
        &mut self.mutex
    }

    #[inline]
    pub(crate) fn base_wrapper(&self) -> &EnvWrapper {
        &self.base
    }

    #[inline]
    pub(crate) fn base_wrapper_mut(&mut self) -> &mut EnvWrapper {
        &mut self.base
    }
}

/// Returns a new environment that stores its data in memory and delegates all
/// non-file-storage tasks to base_env. 
///
/// The caller must delete the result when it is no longer needed.  *base_env
/// must remain live while the result is in use.
///
pub fn new_mem_env(base_env: Rc<RefCell<dyn Env>>) -> Rc<RefCell<dyn Env>> {
    trace!("new_mem_env: creating new InMemoryEnv");
    let env = InMemoryEnv::new(base_env);
    Rc::new(RefCell::new(env))
}

// Implement the Env marker trait for InMemoryEnv so it can be used as a dyn Env.
impl Env for InMemoryEnv {}

impl Schedule for InMemoryEnv {
    fn schedule(
        &mut self,
        function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        arg: *mut std::ffi::c_void,
    ) {
        trace!("InMemoryEnv::schedule: delegating to base env");
        self.base_wrapper_mut().schedule(function, arg);
    }
}

impl StartThread for InMemoryEnv {
    fn start_thread(
        &mut self,
        function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        arg: *mut std::ffi::c_void,
    ) {
        trace!("InMemoryEnv::start_thread: delegating to base env");
        self.base_wrapper_mut().start_thread(function, arg);
    }
}

impl NowMicros for InMemoryEnv {
    fn now_micros(&mut self) -> u64 {
        let v = self.base_wrapper_mut().now_micros();
        trace!("InMemoryEnv::now_micros: delegating to base env -> {}", v);
        v
    }
}

impl SleepForMicroseconds for InMemoryEnv {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        trace!(
            "InMemoryEnv::sleep_for_microseconds: delegating to base env; micros={}",
            micros
        );
        self.base_wrapper_mut().sleep_for_microseconds(micros);
    }
}

#[cfg(test)]
pub(crate) mod in_memory_env_behavior_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        CreateDir,
        DeleteDir,
        DeleteFile,
        Env,
        FileExists,
        GetChildren,
        GetFileSize,
        GetTestDirectory,
        LockFile,
        Logger,
        NewAppendableFile,
        NewLogger,
        NewRandomAccessFile,
        NewSequentialFile,
        NewWritableFile,
        NowMicros,
        RandomAccessFile,
        RenameFile,
        Schedule,
        SequentialFile,
        SleepForMicroseconds,
        StartThread,
        UnlockFile,
        WritableFile,
        WritableFileAppend,
        WritableFileClose,
        WritableFileFlush,
        WritableFileSync,
        FileLock,
    };

    #[derive(Default)]
    pub struct TestBaseEnv;

    impl Env for TestBaseEnv {}

    impl DeleteFile for TestBaseEnv {
        fn delete_file(&mut self, _fname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl CreateDir for TestBaseEnv {
        fn create_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl DeleteDir for TestBaseEnv {
        fn delete_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl NewSequentialFile for TestBaseEnv {
        fn new_sequential_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn SequentialFile>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewRandomAccessFile for TestBaseEnv {
        fn new_random_access_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewWritableFile for TestBaseEnv {
        fn new_writable_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewAppendableFile for TestBaseEnv {
        fn new_appendable_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl FileExists for TestBaseEnv {
        fn file_exists(&mut self, _fname: &String) -> bool {
            false
        }
    }

    impl GetChildren for TestBaseEnv {
        fn get_children(
            &mut self,
            _dir: &String,
            result: *mut Vec<String>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    (*result).clear();
                }
            }
            crate::Status::ok()
        }
    }

    impl GetFileSize for TestBaseEnv {
        fn get_file_size(
            &mut self,
            _fname: &String,
            file_size: *mut u64,
        ) -> crate::Status {
            unsafe {
                if !file_size.is_null() {
                    *file_size = 0;
                }
            }
            crate::Status::ok()
        }
    }

    impl RenameFile for TestBaseEnv {
        fn rename_file(
            &mut self,
            _src: &String,
            _target: &String,
        ) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl LockFile for TestBaseEnv {
        fn lock_file(
            &mut self,
            _fname: &String,
            lock: *mut *mut Box<dyn FileLock>,
        ) -> crate::Status {
            unsafe {
                if !lock.is_null() {
                    *lock = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl UnlockFile for TestBaseEnv {
        fn unlock_file(
            &mut self,
            _lock: *mut Box<dyn FileLock>,
        ) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl Schedule for TestBaseEnv {
        fn schedule(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
            // No-op for tests; we only care that this path is callable.
        }
    }

    impl StartThread for TestBaseEnv {
        fn start_thread(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
            // No-op for tests.
        }
    }

    impl GetTestDirectory for TestBaseEnv {
        fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
            unsafe {
                if !path.is_null() {
                    *path = "/tmp/test-base-env".to_string();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewLogger for TestBaseEnv {
        fn new_logger(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn Logger>,
        ) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }
            crate::Status::ok()
        }
    }

    impl NowMicros for TestBaseEnv {
        fn now_micros(&mut self) -> u64 {
            42
        }
    }

    impl SleepForMicroseconds for TestBaseEnv {
        fn sleep_for_microseconds(&mut self, _micros: i32) {
            // No-op; we just want the call path to be valid.
        }
    }

    fn make_in_memory_env() -> InMemoryEnv {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        InMemoryEnv::new(base)
    }

    #[traced_test]
    fn new_constructs_env_with_empty_file_map() {
        crate::ix!();

        let env = make_in_memory_env();
        let guard = env.inner_mutex().lock();
        assert!(guard.file_map().is_empty());
    }

    #[traced_test]
    fn schedule_is_forwarded_without_panicking() {
        crate::ix!();

        let mut env = make_in_memory_env();

        fn dummy(_arg: *mut std::ffi::c_void) -> std::ffi::c_void {
            unsafe { core::mem::zeroed() }
        }

        env.schedule(dummy, core::ptr::null_mut());
    }

    #[traced_test]
    fn now_micros_returns_monotonic_values() {
        crate::ix!();

        let mut env = make_in_memory_env();

        let first = env.now_micros();
        let second = env.now_micros();

        // Whatever the underlying Env returns, we at least require calls
        // to succeed and not move backwards in time for this test harness.
        assert!(second >= first);
    }

    #[traced_test]
    fn sleep_for_microseconds_completes_without_error() {
        crate::ix!();

        let mut env = make_in_memory_env();
        env.sleep_for_microseconds(1_000);
    }

    #[traced_test]
    fn new_mem_env_returns_env_trait_object_backed_by_in_memory_env() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));

        let mem_env = new_mem_env(base);
        let mut env_ref = mem_env.borrow_mut();

        let fname = "new_mem_env_test_file".to_string();
        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let status = env_ref.new_writable_file(
            &fname,
            &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());

        unsafe {
            if !wf_ptr.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
            }
        }
    }
}
