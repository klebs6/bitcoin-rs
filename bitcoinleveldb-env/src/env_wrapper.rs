// ---------------- [ File: bitcoinleveldb-env/src/env_wrapper.rs ]
crate::ix!();

/**
  | An implementation of Env that forwards all
  | calls to another Env.
  |
  | May be useful to clients who wish to override
  | just part of the functionality of another Env.
  */
pub struct EnvWrapper {
    /**
      | The wrapped target environment.
      |
      | Invariant:
      | this Rc owns the target lifetime for as long as the wrapper is live, but
      | all actual forwarding must be serialized through `target_access_gate`.
      */
    target: Rc<RefCell<dyn Env>>,

    /**
      | Shared serialization gate for every wrapper that reaches the same target.
      |
      | Invariant:
      | all forwarded calls touching the same underlying target must take this
      | gate before borrowing the RefCell so foreground and background work
      | cannot overlap at the borrow layer.
      */
    target_access_gate: Arc<Mutex<()>>,
}

impl Env for EnvWrapper { }

impl DeleteFile for EnvWrapper {
    fn delete_file(&mut self, f: &String) -> Status {
        trace!(file = %f, "EnvWrapper::delete_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "delete_file",
            |target_env| target_env.delete_file(f),
        );
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::delete_file completed");
        status
    }
}

impl CreateDir for EnvWrapper {
    fn create_dir(&mut self, d: &String) -> Status {
        trace!(dir = %d, "EnvWrapper::create_dir forwarding to target Env");
        let status = self.with_serialized_target_access(
            "create_dir",
            |target_env| target_env.create_dir(d),
        );
        debug!(dir = %d, ok = status.is_ok(), "EnvWrapper::create_dir completed");
        status
    }
}

impl DeleteDir for EnvWrapper {
    fn delete_dir(&mut self, d: &String) -> Status {
        trace!(dir = %d, "EnvWrapper::delete_dir forwarding to target Env");
        let status = self.with_serialized_target_access(
            "delete_dir",
            |target_env| target_env.delete_dir(d),
        );
        debug!(dir = %d, ok = status.is_ok(), "EnvWrapper::delete_dir completed");
        status
    }
}

impl NewSequentialFile for EnvWrapper {
    /// The following text is boilerplate that
    /// forwards all methods to target().
    ///
    fn new_sequential_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::new_sequential_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "new_sequential_file",
            |target_env| target_env.new_sequential_file(f, r),
        );
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_sequential_file completed");
        status
    }
}

impl NewRandomAccessFile for EnvWrapper {
    fn new_random_access_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::new_random_access_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "new_random_access_file",
            |target_env| target_env.new_random_access_file(f, r),
        );
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_random_access_file completed");
        status
    }
}

impl NewWritableFile for EnvWrapper {
    fn new_writable_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::new_writable_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "new_writable_file",
            |target_env| target_env.new_writable_file(f, r),
        );
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_writable_file completed");
        status
    }
}

impl NewAppendableFile for EnvWrapper {
    fn new_appendable_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::new_appendable_file forwarding to target Env");

        // Disambiguate against the Env::new_appendable_file default helper.
        let status = self.with_serialized_target_access(
            "new_appendable_file",
            |target_env| {
                bitcoinleveldb_file::NewAppendableFile::new_appendable_file(
                    target_env,
                    f,
                    r,
                )
            },
        );

        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_appendable_file completed");
        status
    }
}

impl FileExists for EnvWrapper {
    fn file_exists(&mut self, f: &String) -> bool {
        trace!(file = %f, "EnvWrapper::file_exists forwarding to target Env");
        let exists = self.with_serialized_target_access(
            "file_exists",
            |target_env| target_env.file_exists(f),
        );
        debug!(file = %f, exists, "EnvWrapper::file_exists completed");
        exists
    }
}

impl GetChildren for EnvWrapper {
    fn get_children(&mut self, dir: &String, r: *mut Vec<String>) -> Status {
        trace!(dir = %dir, "EnvWrapper::get_children forwarding to target Env");
        let status = self.with_serialized_target_access(
            "get_children",
            |target_env| target_env.get_children(dir, r),
        );
        debug!(dir = %dir, ok = status.is_ok(), "EnvWrapper::get_children completed");
        status
    }
}

impl GetFileSize for EnvWrapper {
    fn get_file_size(&mut self, f: &String, s: *mut u64) -> Status {
        trace!(file = %f, "EnvWrapper::get_file_size forwarding to target Env");
        let status = self.with_serialized_target_access(
            "get_file_size",
            |target_env| target_env.get_file_size(f, s),
        );
        let size_val = unsafe { s.as_ref().copied() };
        debug!(
            file = %f,
            ok = status.is_ok(),
            size = size_val,
            "EnvWrapper::get_file_size completed"
        );
        status
    }
}

impl RenameFile for EnvWrapper {
    fn rename_file(&mut self, src: &String, target: &String) -> Status {
        trace!(src = %src, target = %target, "EnvWrapper::rename_file forwarding");
        let status = self.with_serialized_target_access(
            "rename_file",
            |target_env| target_env.rename_file(src, target),
        );
        debug!(
            src = %src,
            target = %target,
            ok = status.is_ok(),
            "EnvWrapper::rename_file completed"
        );
        status
    }
}

impl LockFile for EnvWrapper {
    fn lock_file(
        &mut self,
        f: &String,
        l: *mut *mut Box<dyn FileLock>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::lock_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "lock_file",
            |target_env| target_env.lock_file(f, l),
        );
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::lock_file completed");
        status
    }
}

impl UnlockFile for EnvWrapper {
    fn unlock_file(&mut self, l: *mut Box<dyn FileLock>) -> Status {
        trace!("EnvWrapper::unlock_file forwarding to target Env");
        let status = self.with_serialized_target_access(
            "unlock_file",
            |target_env| target_env.unlock_file(l),
        );
        debug!(ok = status.is_ok(), "EnvWrapper::unlock_file completed");
        status
    }
}

impl Schedule for EnvWrapper {
    fn schedule(
        &mut self,
        f: fn(_0: *mut c_void) -> c_void,
        a: *mut c_void,
    ) {
        trace!("EnvWrapper::schedule forwarding to target Env");
        self.with_serialized_target_access(
            "schedule",
            |target_env| {
                target_env.schedule(f, a);
            },
        );
        debug!("EnvWrapper::schedule completed");
    }
}

impl StartThread for EnvWrapper {
    fn start_thread(
        &mut self,
        f: fn(_0: *mut c_void) -> c_void,
        a: *mut c_void,
    ) {
        trace!("EnvWrapper::start_thread forwarding to target Env");
        self.with_serialized_target_access(
            "start_thread",
            |target_env| {
                target_env.start_thread(f, a);
            },
        );
        debug!("EnvWrapper::start_thread completed");
    }
}

impl GetTestDirectory for EnvWrapper {
    fn get_test_directory(&mut self, path: *mut String) -> Status {
        trace!("EnvWrapper::get_test_directory forwarding to target Env");
        let status = self.with_serialized_target_access(
            "get_test_directory",
            |target_env| target_env.get_test_directory(path),
        );
        let path_preview = unsafe { path.as_ref().map(|s| s.as_str()).unwrap_or("") };
        debug!(
            path = %path_preview,
            ok = status.is_ok(),
            "EnvWrapper::get_test_directory completed"
        );
        status
    }
}

impl NewLogger for EnvWrapper {
    fn new_logger(
        &mut self,
        fname: &String,
        result: *mut *mut Box<dyn Logger>,
    ) -> Status {
        trace!(file = %fname, "EnvWrapper::new_logger forwarding to target Env");
        let status = self.with_serialized_target_access(
            "new_logger",
            |target_env| target_env.new_logger(fname, result),
        );
        debug!(file = %fname, ok = status.is_ok(), "EnvWrapper::new_logger completed");
        status
    }
}

impl NowMicros for EnvWrapper {
    fn now_micros(&mut self) -> u64 {
        trace!("EnvWrapper::now_micros forwarding to target Env");
        let micros = self.with_serialized_target_access(
            "now_micros",
            |target_env| target_env.now_micros(),
        );
        debug!(micros, "EnvWrapper::now_micros completed");
        micros
    }
}

impl SleepForMicroseconds for EnvWrapper {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        trace!(micros, "EnvWrapper::sleep_for_microseconds forwarding to target Env");
        self.with_serialized_target_access(
            "sleep_for_microseconds",
            |target_env| {
                target_env.sleep_for_microseconds(micros);
            },
        );
        debug!(micros, "EnvWrapper::sleep_for_microseconds completed");
    }
}

impl EnvWrapper {
    /// Initialize an EnvWrapper that delegates all
    /// calls to `t`.
    ///
    /// Invariant:
    /// wrappers created for the same underlying target share a common forwarding
    /// gate so `RefCell` borrow state is serialized across foreground and
    /// background database activity.
    pub fn new(t: Rc<RefCell<dyn Env>>) -> Self {
        trace!("EnvWrapper::new constructing wrapper");

        let target_access_gate = env_wrapper_shared_borrow_gate_for_target(&t);

        let wrapper = Self {
            target: t,
            target_access_gate,
        };

        debug!("EnvWrapper::new constructed");
        wrapper
    }

    /// Return the target to which this Env
    /// forwards all calls.
    ///
    /// Postcondition:
    /// the returned Rc aliases the original target identity preserved by this
    /// wrapper.
    pub fn target(&self) -> Rc<RefCell<dyn Env>> {
        trace!("EnvWrapper::target cloning Rc to inner Env");
        self.target.clone()
    }

    /// Returns the stable identity key for the wrapped target.
    ///
    /// Invariant:
    /// equal return values imply shared forwarding serialization.
    pub fn target_identity_key(&self) -> usize {
        env_wrapper_target_identity_key(&self.target)
    }

    /// Executes `operation` while holding the shared target-access gate and an
    /// exclusive RefCell borrow of the wrapped target.
    ///
    /// Invariant:
    /// this is the only permitted forwarding path for target calls.
    pub fn with_serialized_target_access<TOperationResult, TOperation>(
        &mut self,
        operation_label: &'static str,
        operation:       TOperation,
    ) -> TOperationResult
    where
        TOperation: FnOnce(&mut dyn Env) -> TOperationResult,
    {
        let target_identity_key = self.target_identity_key();

        trace!(
            target: "bitcoinleveldb_env::env_wrapper",
            label = "env_wrapper.with_serialized_target_access.entry",
            operation_label,
            target_identity_key,
        );

        let gate_lock_result = self.target_access_gate.lock();

        let _gate_guard = match gate_lock_result {
            Ok(gate_guard) => gate_guard,
            Err(poisoned_gate_guard) => {
                warn!(
                    target: "bitcoinleveldb_env::env_wrapper",
                    label = "env_wrapper.with_serialized_target_access.gate_poisoned",
                    operation_label,
                    target_identity_key,
                );
                poisoned_gate_guard.into_inner()
            }
        };

        let target_borrow_result = self.target.try_borrow_mut();

        let mut target_borrow = match target_borrow_result {
            Ok(target_borrow) => target_borrow,
            Err(target_borrow_error) => {
                error!(
                    target: "bitcoinleveldb_env::env_wrapper",
                    label = "env_wrapper.with_serialized_target_access.borrow_failed",
                    operation_label,
                    target_identity_key,
                    borrow_error = ?target_borrow_error,
                );
                panic!("EnvWrapper::with_serialized_target_access: target borrow failed");
            }
        };

        let operation_result = operation(&mut *target_borrow);

        trace!(
            target: "bitcoinleveldb_env::env_wrapper",
            label = "env_wrapper.with_serialized_target_access.exit",
            operation_label,
            target_identity_key,
        );

        operation_result
    }
}

#[cfg(test)]
mod env_wrapper_serialized_target_access_behavior_tests {
    use super::*;

    struct EnvWrapperSerializedTargetAccessTestDummySequentialFile;

    impl Named for EnvWrapperSerializedTargetAccessTestDummySequentialFile {
        fn name(&self) -> std::borrow::Cow<'_,str> {
            std::borrow::Cow::Borrowed(
                "env_wrapper_serialized_target_access_test_dummy_sequential_file",
            )
        }
    }

    impl SequentialFile for EnvWrapperSerializedTargetAccessTestDummySequentialFile {}

    impl SequentialFileRead for EnvWrapperSerializedTargetAccessTestDummySequentialFile {
        fn read(
            &mut self,
            _n:      usize,
            result:  *mut Slice,
            _scratch: *mut u8,
        ) -> Status {
            unsafe {
                if !result.is_null() {
                    *result = Slice::default();
                }
            }
            Status::ok()
        }
    }

    impl SequentialFileSkip for EnvWrapperSerializedTargetAccessTestDummySequentialFile {
        fn skip(&mut self, _n: u64) -> Status {
            Status::ok()
        }
    }

    struct EnvWrapperSerializedTargetAccessTestDummyRandomAccessFile;

    impl Named for EnvWrapperSerializedTargetAccessTestDummyRandomAccessFile {
        fn name(&self) -> std::borrow::Cow<'_,str> {
            std::borrow::Cow::Borrowed(
                "env_wrapper_serialized_target_access_test_dummy_random_access_file",
            )
        }
    }

    impl RandomAccessFile for EnvWrapperSerializedTargetAccessTestDummyRandomAccessFile {}

    impl RandomAccessFileRead for EnvWrapperSerializedTargetAccessTestDummyRandomAccessFile {
        fn read(
            &self,
            _offset: u64,
            _n:      usize,
            result:  *mut Slice,
            _scratch: *mut u8,
        ) -> Status {
            unsafe {
                if !result.is_null() {
                    *result = Slice::default();
                }
            }
            Status::ok()
        }
    }

    struct EnvWrapperSerializedTargetAccessTestDummyWritableFile;

    impl Named for EnvWrapperSerializedTargetAccessTestDummyWritableFile {
        fn name(&self) -> std::borrow::Cow<'_,str> {
            std::borrow::Cow::Borrowed(
                "env_wrapper_serialized_target_access_test_dummy_writable_file",
            )
        }
    }

    impl WritableFile for EnvWrapperSerializedTargetAccessTestDummyWritableFile {}

    impl WritableFileAppend for EnvWrapperSerializedTargetAccessTestDummyWritableFile {
        fn append(&mut self, _data: &Slice) -> Status {
            Status::ok()
        }
    }

    impl WritableFileClose for EnvWrapperSerializedTargetAccessTestDummyWritableFile {
        fn close(&mut self) -> Status {
            Status::ok()
        }
    }

    impl WritableFileFlush for EnvWrapperSerializedTargetAccessTestDummyWritableFile {
        fn flush(&mut self) -> Status {
            Status::ok()
        }
    }

    impl WritableFileSync for EnvWrapperSerializedTargetAccessTestDummyWritableFile {
        fn sync(&mut self) -> Status {
            Status::ok()
        }
    }

    struct EnvWrapperSerializedTargetAccessTestDummyLogger;

    impl Logger for EnvWrapperSerializedTargetAccessTestDummyLogger {}
    impl Logv for EnvWrapperSerializedTargetAccessTestDummyLogger {
        fn logv(&mut self, _: *const u8, _: &[&str]) { todo!() }
    }

    struct EnvWrapperSerializedTargetAccessTestDummyFileLock;

    impl FileLock for EnvWrapperSerializedTargetAccessTestDummyFileLock {}

    fn env_wrapper_serialized_target_access_test_install_boxed_sequential_file(
        result: *mut *mut Box<dyn SequentialFile>,
    ) {
        unsafe {
            if !result.is_null() {
                let file: Box<dyn SequentialFile> =
                    Box::new(EnvWrapperSerializedTargetAccessTestDummySequentialFile);
                *result = Box::into_raw(Box::new(file));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_install_boxed_random_access_file(
        result: *mut *mut Box<dyn RandomAccessFile>,
    ) {
        unsafe {
            if !result.is_null() {
                let file: Box<dyn RandomAccessFile> =
                    Box::new(EnvWrapperSerializedTargetAccessTestDummyRandomAccessFile);
                *result = Box::into_raw(Box::new(file));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_install_boxed_writable_file(
        result: *mut *mut Box<dyn WritableFile>,
    ) {
        unsafe {
            if !result.is_null() {
                let file: Box<dyn WritableFile> =
                    Box::new(EnvWrapperSerializedTargetAccessTestDummyWritableFile);
                *result = Box::into_raw(Box::new(file));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_install_boxed_logger(
        result: *mut *mut Box<dyn Logger>,
    ) {
        unsafe {
            if !result.is_null() {
                let logger: Box<dyn Logger> =
                    Box::new(EnvWrapperSerializedTargetAccessTestDummyLogger);
                *result = Box::into_raw(Box::new(logger));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_install_boxed_file_lock(
        result: *mut *mut Box<dyn FileLock>,
    ) {
        unsafe {
            if !result.is_null() {
                let lock: Box<dyn FileLock> =
                    Box::new(EnvWrapperSerializedTargetAccessTestDummyFileLock);
                *result = Box::into_raw(Box::new(lock));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_drop_raw_boxed_sequential_file(
        ptr: *mut Box<dyn SequentialFile>,
    ) {
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_drop_raw_boxed_random_access_file(
        ptr: *mut Box<dyn RandomAccessFile>,
    ) {
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_drop_raw_boxed_writable_file(
        ptr: *mut Box<dyn WritableFile>,
    ) {
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }

    fn env_wrapper_serialized_target_access_test_drop_raw_boxed_logger(
        ptr: *mut Box<dyn Logger>,
    ) {
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }

    struct EnvWrapperSerializedTargetAccessTestRecordingState {
        delete_file_count:             usize,
        delete_file_last_path:         String,
        create_dir_count:              usize,
        create_dir_last_path:          String,
        delete_dir_count:              usize,
        delete_dir_last_path:          String,
        file_exists_count:             usize,
        file_exists_last_path:         String,
        file_exists_result:            bool,
        get_children_count:            usize,
        get_children_last_dir:         String,
        get_children_result_entries:   Vec<String>,
        get_file_size_count:           usize,
        get_file_size_last_path:       String,
        get_file_size_result:          u64,
        rename_file_count:             usize,
        rename_file_last_src:          String,
        rename_file_last_target:       String,
        new_sequential_file_count:     usize,
        new_random_access_file_count:  usize,
        new_writable_file_count:       usize,
        new_appendable_file_count:     usize,
        lock_file_count:               usize,
        lock_file_last_path:           String,
        unlock_file_count:             usize,
        new_logger_count:              usize,
        new_logger_last_path:          String,
        get_test_directory_count:      usize,
        get_test_directory_value:      String,
        now_micros_count:              usize,
        now_micros_value:              u64,
        sleep_for_microseconds_count:  usize,
        sleep_for_microseconds_last:   i32,
        schedule_count:                usize,
        schedule_last_function:        usize,
        schedule_last_arg:             usize,
        start_thread_count:            usize,
        start_thread_last_function:    usize,
        start_thread_last_arg:         usize,
    }

    fn env_wrapper_serialized_target_access_test_build_recording_state(
    ) -> std::sync::Arc<
        std::sync::Mutex<EnvWrapperSerializedTargetAccessTestRecordingState>,
    > {
        std::sync::Arc::new(std::sync::Mutex::new(
            EnvWrapperSerializedTargetAccessTestRecordingState {
                delete_file_count:            0,
                delete_file_last_path:        String::new(),
                create_dir_count:             0,
                create_dir_last_path:         String::new(),
                delete_dir_count:             0,
                delete_dir_last_path:         String::new(),
                file_exists_count:            0,
                file_exists_last_path:        String::new(),
                file_exists_result:           false,
                get_children_count:           0,
                get_children_last_dir:        String::new(),
                get_children_result_entries:  Vec::new(),
                get_file_size_count:          0,
                get_file_size_last_path:      String::new(),
                get_file_size_result:         0,
                rename_file_count:            0,
                rename_file_last_src:         String::new(),
                rename_file_last_target:      String::new(),
                new_sequential_file_count:    0,
                new_random_access_file_count: 0,
                new_writable_file_count:      0,
                new_appendable_file_count:    0,
                lock_file_count:              0,
                lock_file_last_path:          String::new(),
                unlock_file_count:            0,
                new_logger_count:             0,
                new_logger_last_path:         String::new(),
                get_test_directory_count:     0,
                get_test_directory_value:     String::from(
                    "env-wrapper-serialized-target-access-test-directory",
                ),
                now_micros_count:             0,
                now_micros_value:             0,
                sleep_for_microseconds_count: 0,
                sleep_for_microseconds_last:  0,
                schedule_count:               0,
                schedule_last_function:       0,
                schedule_last_arg:            0,
                start_thread_count:           0,
                start_thread_last_function:   0,
                start_thread_last_arg:        0,
            },
        ))
    }

    fn env_wrapper_serialized_target_access_test_lock_recording_state<'a>(
        shared_state: &'a std::sync::Arc<
            std::sync::Mutex<EnvWrapperSerializedTargetAccessTestRecordingState>,
        >,
    ) -> std::sync::MutexGuard<
        'a,
        EnvWrapperSerializedTargetAccessTestRecordingState,
    > {
        match shared_state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    struct EnvWrapperSerializedTargetAccessTestRecordingEnv {
        shared_state: std::sync::Arc<
            std::sync::Mutex<EnvWrapperSerializedTargetAccessTestRecordingState>,
        >,
        block_first_get_test_directory_call: bool,
        get_test_directory_entry_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        get_test_directory_release_first_call:
            std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    impl Named for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn name(&self) -> std::borrow::Cow<'_,str> {
            std::borrow::Cow::Borrowed(
                "env_wrapper_serialized_target_access_test_recording_env",
            )
        }
    }

    impl Env for EnvWrapperSerializedTargetAccessTestRecordingEnv {}

    impl DeleteFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn delete_file(&mut self, fname: &String) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.delete_file_count += 1;
            state.delete_file_last_path = fname.clone();
            Status::ok()
        }
    }

    impl CreateDir for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn create_dir(&mut self, dirname: &String) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.create_dir_count += 1;
            state.create_dir_last_path = dirname.clone();
            Status::ok()
        }
    }

    impl DeleteDir for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn delete_dir(&mut self, dirname: &String) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.delete_dir_count += 1;
            state.delete_dir_last_path = dirname.clone();
            Status::ok()
        }
    }

    impl NewSequentialFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn new_sequential_file(
            &mut self,
            _fname:  &String,
            result: *mut *mut Box<dyn SequentialFile>,
        ) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.new_sequential_file_count += 1;
            drop(state);

            env_wrapper_serialized_target_access_test_install_boxed_sequential_file(
                result,
            );
            Status::ok()
        }
    }

    impl NewRandomAccessFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn new_random_access_file(
            &mut self,
            _fname:  &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.new_random_access_file_count += 1;
            drop(state);

            env_wrapper_serialized_target_access_test_install_boxed_random_access_file(
                result,
            );
            Status::ok()
        }
    }

    impl NewWritableFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn new_writable_file(
            &mut self,
            _fname:  &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.new_writable_file_count += 1;
            drop(state);

            env_wrapper_serialized_target_access_test_install_boxed_writable_file(
                result,
            );
            Status::ok()
        }
    }

    impl NewAppendableFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn new_appendable_file(
            &mut self,
            _fname:  &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.new_appendable_file_count += 1;
            drop(state);

            env_wrapper_serialized_target_access_test_install_boxed_writable_file(
                result,
            );
            Status::ok()
        }
    }

    impl FileExists for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn file_exists(&mut self, fname: &String) -> bool {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.file_exists_count += 1;
            state.file_exists_last_path = fname.clone();
            state.file_exists_result
        }
    }

    impl GetChildren for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn get_children(&mut self, dir: &String, result: *mut Vec<String>) -> Status {
            let result_entries = {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.get_children_count += 1;
                state.get_children_last_dir = dir.clone();
                state.get_children_result_entries.clone()
            };

            unsafe {
                if !result.is_null() {
                    (*result).clear();
                    for entry in result_entries.iter() {
                        (*result).push(entry.clone());
                    }
                }
            }

            Status::ok()
        }
    }

    impl GetFileSize for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn get_file_size(&mut self, fname: &String, file_size: *mut u64) -> Status {
            let size_value = {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.get_file_size_count += 1;
                state.get_file_size_last_path = fname.clone();
                state.get_file_size_result
            };

            unsafe {
                if !file_size.is_null() {
                    *file_size = size_value;
                }
            }

            Status::ok()
        }
    }

    impl RenameFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn rename_file(&mut self, src: &String, target: &String) -> Status {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.rename_file_count += 1;
            state.rename_file_last_src = src.clone();
            state.rename_file_last_target = target.clone();
            Status::ok()
        }
    }

    impl LockFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn lock_file(
            &mut self,
            fname: &String,
            lock:  *mut *mut Box<dyn FileLock>,
        ) -> Status {
            {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.lock_file_count += 1;
                state.lock_file_last_path = fname.clone();
            }

            env_wrapper_serialized_target_access_test_install_boxed_file_lock(lock);
            Status::ok()
        }
    }

    impl UnlockFile for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> Status {
            {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.unlock_file_count += 1;
            }

            unsafe {
                if !lock.is_null() {
                    drop(Box::from_raw(lock));
                }
            }

            Status::ok()
        }
    }

    impl Schedule for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn schedule(
            &mut self,
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void,
        ) {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.schedule_count += 1;
            state.schedule_last_function = function as usize;
            state.schedule_last_arg = arg as usize;
        }
    }

    impl StartThread for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn start_thread(
            &mut self,
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void,
        ) {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.start_thread_count += 1;
            state.start_thread_last_function = function as usize;
            state.start_thread_last_arg = arg as usize;
        }
    }

    impl GetTestDirectory for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn get_test_directory(&mut self, path: *mut String) -> Status {
            let entry_index = self
                .get_test_directory_entry_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                + 1;

            if self.block_first_get_test_directory_call && entry_index == 1 {
                while !self
                    .get_test_directory_release_first_call
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    std::thread::yield_now();
                }
            }

            let path_value = {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.get_test_directory_count += 1;
                state.get_test_directory_value.clone()
            };

            unsafe {
                if !path.is_null() {
                    *path = path_value;
                }
            }

            Status::ok()
        }
    }

    impl NewLogger for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn new_logger(
            &mut self,
            fname:  &String,
            result: *mut *mut Box<dyn Logger>,
        ) -> Status {
            {
                let mut state =
                    env_wrapper_serialized_target_access_test_lock_recording_state(
                        &self.shared_state,
                    );
                state.new_logger_count += 1;
                state.new_logger_last_path = fname.clone();
            }

            env_wrapper_serialized_target_access_test_install_boxed_logger(result);
            Status::ok()
        }
    }

    impl NowMicros for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn now_micros(&mut self) -> u64 {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.now_micros_count += 1;
            state.now_micros_value
        }
    }

    impl SleepForMicroseconds for EnvWrapperSerializedTargetAccessTestRecordingEnv {
        fn sleep_for_microseconds(&mut self, micros: i32) {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &self.shared_state,
                );
            state.sleep_for_microseconds_count += 1;
            state.sleep_for_microseconds_last = micros;
        }
    }

    fn env_wrapper_serialized_target_access_test_make_target(
        shared_state: std::sync::Arc<
            std::sync::Mutex<EnvWrapperSerializedTargetAccessTestRecordingState>,
        >,
    ) -> Rc<RefCell<dyn Env>> {
        Rc::new(RefCell::new(
            EnvWrapperSerializedTargetAccessTestRecordingEnv {
                shared_state,
                block_first_get_test_directory_call: false,
                get_test_directory_entry_count: std::sync::Arc::new(
                    std::sync::atomic::AtomicUsize::new(0),
                ),
                get_test_directory_release_first_call: std::sync::Arc::new(
                    std::sync::atomic::AtomicBool::new(true),
                ),
            },
        ))
    }

    fn env_wrapper_serialized_target_access_test_never_invoke_callback(
        _arg: *mut c_void,
    ) -> c_void {
        unreachable!()
    }

    #[traced_test]
    fn env_wrapper_shared_identity_key_is_stable_for_rc_clones_and_distinct_for_other_targets() {
        let first_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let first_target =
            env_wrapper_serialized_target_access_test_make_target(first_state);

        let first_target_clone = first_target.clone();

        let second_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let second_target =
            env_wrapper_serialized_target_access_test_make_target(second_state);

        let first_key = env_wrapper_target_identity_key(&first_target);
        let first_clone_key =
            env_wrapper_target_identity_key(&first_target_clone);
        let second_key = env_wrapper_target_identity_key(&second_target);

        assert_eq!(
            first_key,
            first_clone_key,
            "Rc clones must preserve the wrapped target identity key"
        );

        assert_ne!(
            first_key,
            second_key,
            "Distinct Rc allocations must not share the same target identity key"
        );
    }

    #[traced_test]
    fn env_wrapper_shared_borrow_gate_reuses_same_mutex_for_same_target_identity() {
        let first_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let first_target =
            env_wrapper_serialized_target_access_test_make_target(first_state);

        let first_target_clone = first_target.clone();

        let second_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let second_target =
            env_wrapper_serialized_target_access_test_make_target(second_state);

        let first_gate =
            env_wrapper_shared_borrow_gate_for_target(&first_target);
        let first_clone_gate =
            env_wrapper_shared_borrow_gate_for_target(&first_target_clone);
        let second_gate =
            env_wrapper_shared_borrow_gate_for_target(&second_target);

        let first_gate_ptr =
            std::sync::Arc::as_ptr(&first_gate) as usize;
        let first_clone_gate_ptr =
            std::sync::Arc::as_ptr(&first_clone_gate) as usize;
        let second_gate_ptr =
            std::sync::Arc::as_ptr(&second_gate) as usize;

        assert_eq!(
            first_gate_ptr,
            first_clone_gate_ptr,
            "Same target identity must reuse the same shared borrow gate"
        );

        assert_ne!(
            first_gate_ptr,
            second_gate_ptr,
            "Different target identities must not alias the same shared borrow gate"
        );
    }

    #[traced_test]
    fn env_wrapper_target_accessor_preserves_wrapped_target_identity() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(shared_state);

        let wrapper = EnvWrapper::new(target.clone());
        let cloned_target = wrapper.target();

        let original_identity_key = env_wrapper_target_identity_key(&target);
        let cloned_identity_key =
            env_wrapper_target_identity_key(&cloned_target);

        assert_eq!(
            original_identity_key,
            cloned_identity_key,
            "EnvWrapper::target must clone the original wrapped target allocation"
        );
    }

    #[traced_test]
    fn env_wrapper_with_serialized_target_access_returns_operation_result_and_updates_state() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();

        {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &shared_state,
                );
            state.now_micros_value = 4242;
        }

        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let micros = wrapper.with_serialized_target_access(
            "env_wrapper_serialized_target_access_test_now_micros",
            |target_env| target_env.now_micros(),
        );

        assert_eq!(
            micros,
            4242,
            "with_serialized_target_access must return the closure result"
        );

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(
            state.now_micros_count,
            1,
            "with_serialized_target_access must forward the closure into the wrapped Env exactly once"
        );
    }

    #[traced_test]
    fn env_wrapper_with_serialized_target_access_panics_when_target_is_borrowed_outside_wrapper() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target.clone());

        let _direct_borrow = target.borrow_mut();

        let panic_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = wrapper.with_serialized_target_access(
                    "env_wrapper_serialized_target_access_test_borrow_conflict",
                    |target_env| target_env.now_micros(),
                );
            }),
        );

        assert!(
            panic_result.is_err(),
            "Direct RefCell borrows that bypass EnvWrapper must still trip the borrow-failure panic boundary"
        );
    }

    #[traced_test]
    fn env_wrapper_alias_wrappers_serialize_same_target_access_across_threads() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();

        {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &shared_state,
                );
            state.get_test_directory_value =
                String::from("env-wrapper-serialized-thread-path");
        }

        let entry_count =
            std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let release_first_call =
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let target: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(
            EnvWrapperSerializedTargetAccessTestRecordingEnv {
                shared_state: shared_state.clone(),
                block_first_get_test_directory_call: true,
                get_test_directory_entry_count: entry_count.clone(),
                get_test_directory_release_first_call:
                    release_first_call.clone(),
            },
        ));

        let first_wrapper_ptr =
            Box::into_raw(Box::new(EnvWrapper::new(target.clone())));
        let second_wrapper_ptr =
            Box::into_raw(Box::new(EnvWrapper::new(target.clone())));

        let first_wrapper_addr = first_wrapper_ptr as usize;
        let second_wrapper_addr = second_wrapper_ptr as usize;

        let second_thread_attempted =
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let second_thread_attempted_clone =
            second_thread_attempted.clone();

        let first_thread = std::thread::spawn(move || {
            let wrapper_ptr = first_wrapper_addr as *mut EnvWrapper;
            let mut path = String::new();

            let status = unsafe {
                (&mut *wrapper_ptr)
                    .get_test_directory(&mut path as *mut String)
            };

            (status.is_ok(), path)
        });

        let mut saw_first_entry = false;

        for _ in 0..100_000 {
            if entry_count.load(std::sync::atomic::Ordering::SeqCst) == 1 {
                saw_first_entry = true;
                break;
            }
            std::thread::yield_now();
        }

        assert!(
            saw_first_entry,
            "The first thread must enter the wrapped Env before we test serialized contention"
        );

        let second_thread = std::thread::spawn(move || {
            second_thread_attempted_clone.store(
                true,
                std::sync::atomic::Ordering::SeqCst,
            );

            let wrapper_ptr = second_wrapper_addr as *mut EnvWrapper;
            let mut path = String::new();

            let status = unsafe {
                (&mut *wrapper_ptr)
                    .get_test_directory(&mut path as *mut String)
            };

            (status.is_ok(), path)
        });

        let mut saw_second_attempt = false;

        for _ in 0..100_000 {
            if second_thread_attempted.load(std::sync::atomic::Ordering::SeqCst)
            {
                saw_second_attempt = true;
                break;
            }
            std::thread::yield_now();
        }

        assert!(
            saw_second_attempt,
            "The second thread must attempt wrapped access before we assert that entry remains serialized"
        );

        for _ in 0..10_000 {
            std::thread::yield_now();
        }

        assert_eq!(
            entry_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "The second alias wrapper must not enter the wrapped Env while the first call still holds serialized access"
        );

        release_first_call.store(
            true,
            std::sync::atomic::Ordering::SeqCst,
        );

        let first_thread_value = match first_thread.join() {
            Ok(value) => value,
            Err(_) => unreachable!(),
        };

        let second_thread_value = match second_thread.join() {
            Ok(value) => value,
            Err(_) => unreachable!(),
        };

        unsafe {
            drop(Box::from_raw(first_wrapper_ptr));
            drop(Box::from_raw(second_wrapper_ptr));
        }

        assert!(
            first_thread_value.0,
            "The first alias wrapper call must complete successfully"
        );
        assert!(
            second_thread_value.0,
            "The second alias wrapper call must complete successfully after serialized waiting"
        );

        assert_eq!(
            first_thread_value.1,
            String::from("env-wrapper-serialized-thread-path"),
            "The first alias wrapper call must preserve the forwarded output path"
        );

        assert_eq!(
            second_thread_value.1,
            String::from("env-wrapper-serialized-thread-path"),
            "The second alias wrapper call must preserve the forwarded output path"
        );

        assert_eq!(
            entry_count.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "Both alias wrapper calls must eventually reach the wrapped Env exactly once"
        );

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(
            state.get_test_directory_count,
            2,
            "Both serialized alias wrapper calls must be observed by the wrapped Env"
        );
    }

    #[traced_test]
    fn env_wrapper_forwarding_create_delete_and_rename_operations_preserve_arguments_and_statuses() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let create_dir_name = String::from("env-wrapper-create-dir");
        let rename_src_name = String::from("env-wrapper-rename-src");
        let rename_target_name = String::from("env-wrapper-rename-target");
        let delete_file_name = String::from("env-wrapper-delete-file");
        let delete_dir_name = String::from("env-wrapper-delete-dir");

        let create_status = wrapper.create_dir(&create_dir_name);
        let rename_status =
            wrapper.rename_file(&rename_src_name, &rename_target_name);
        let delete_file_status = wrapper.delete_file(&delete_file_name);
        let delete_dir_status = wrapper.delete_dir(&delete_dir_name);

        assert!(create_status.is_ok());
        assert!(rename_status.is_ok());
        assert!(delete_file_status.is_ok());
        assert!(delete_dir_status.is_ok());

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(state.create_dir_count, 1);
        assert_eq!(state.create_dir_last_path, create_dir_name);

        assert_eq!(state.rename_file_count, 1);
        assert_eq!(state.rename_file_last_src, rename_src_name);
        assert_eq!(state.rename_file_last_target, rename_target_name);

        assert_eq!(state.delete_file_count, 1);
        assert_eq!(state.delete_file_last_path, delete_file_name);

        assert_eq!(state.delete_dir_count, 1);
        assert_eq!(state.delete_dir_last_path, delete_dir_name);
    }

    #[traced_test]
    fn env_wrapper_forwarding_new_file_factory_methods_install_non_null_output_handles() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let sequential_file_name = String::from("env-wrapper-sequential-file");
        let random_access_file_name =
            String::from("env-wrapper-random-access-file");
        let writable_file_name = String::from("env-wrapper-writable-file");
        let appendable_file_name = String::from("env-wrapper-appendable-file");

        let mut sequential_file_ptr: *mut Box<dyn SequentialFile> =
            std::ptr::null_mut();
        let mut random_access_file_ptr: *mut Box<dyn RandomAccessFile> =
            std::ptr::null_mut();
        let mut writable_file_ptr: *mut Box<dyn WritableFile> =
            std::ptr::null_mut();
        let mut appendable_file_ptr: *mut Box<dyn WritableFile> =
            std::ptr::null_mut();

        let sequential_status = wrapper.new_sequential_file(
            &sequential_file_name,
            &mut sequential_file_ptr,
        );

        let random_access_status = wrapper.new_random_access_file(
            &random_access_file_name,
            &mut random_access_file_ptr,
        );

        let writable_status = wrapper.new_writable_file(
            &writable_file_name,
            &mut writable_file_ptr,
        );

        let appendable_status = wrapper.new_appendable_file(
            &appendable_file_name,
            &mut appendable_file_ptr,
        );

        assert!(sequential_status.is_ok());
        assert!(random_access_status.is_ok());
        assert!(writable_status.is_ok());
        assert!(appendable_status.is_ok());

        assert!(
            !sequential_file_ptr.is_null(),
            "new_sequential_file must install a non-null output handle on success"
        );
        assert!(
            !random_access_file_ptr.is_null(),
            "new_random_access_file must install a non-null output handle on success"
        );
        assert!(
            !writable_file_ptr.is_null(),
            "new_writable_file must install a non-null output handle on success"
        );
        assert!(
            !appendable_file_ptr.is_null(),
            "new_appendable_file must install a non-null output handle on success"
        );

        env_wrapper_serialized_target_access_test_drop_raw_boxed_sequential_file(
            sequential_file_ptr,
        );
        env_wrapper_serialized_target_access_test_drop_raw_boxed_random_access_file(
            random_access_file_ptr,
        );
        env_wrapper_serialized_target_access_test_drop_raw_boxed_writable_file(
            writable_file_ptr,
        );
        env_wrapper_serialized_target_access_test_drop_raw_boxed_writable_file(
            appendable_file_ptr,
        );

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(state.new_sequential_file_count, 1);
        assert_eq!(state.new_random_access_file_count, 1);
        assert_eq!(state.new_writable_file_count, 1);
        assert_eq!(state.new_appendable_file_count, 1);
    }

    #[traced_test]
    fn env_wrapper_forwarding_lock_unlock_and_new_logger_install_non_null_output_handles() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let lock_name = String::from("env-wrapper-lock-file");
        let logger_name = String::from("env-wrapper-log-file");

        let mut lock_ptr: *mut Box<dyn FileLock> = std::ptr::null_mut();
        let mut logger_ptr: *mut Box<dyn Logger> = std::ptr::null_mut();

        let lock_status = wrapper.lock_file(&lock_name, &mut lock_ptr);
        assert!(lock_status.is_ok());
        assert!(
            !lock_ptr.is_null(),
            "lock_file must install a non-null FileLock handle on success"
        );

        let unlock_status = wrapper.unlock_file(lock_ptr);
        assert!(unlock_status.is_ok());

        let logger_status = wrapper.new_logger(&logger_name, &mut logger_ptr);
        assert!(logger_status.is_ok());
        assert!(
            !logger_ptr.is_null(),
            "new_logger must install a non-null Logger handle on success"
        );

        env_wrapper_serialized_target_access_test_drop_raw_boxed_logger(
            logger_ptr,
        );

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(state.lock_file_count, 1);
        assert_eq!(state.lock_file_last_path, lock_name);
        assert_eq!(state.unlock_file_count, 1);
        assert_eq!(state.new_logger_count, 1);
        assert_eq!(state.new_logger_last_path, logger_name);
    }

    #[traced_test]
    fn env_wrapper_forwarding_file_queries_and_time_calls_preserve_outputs() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();

        {
            let mut state =
                env_wrapper_serialized_target_access_test_lock_recording_state(
                    &shared_state,
                );
            state.file_exists_result = true;
            state.get_children_result_entries = vec![
                String::from("alpha"),
                String::from("beta"),
            ];
            state.get_file_size_result = 9191;
            state.get_test_directory_value =
                String::from("env-wrapper-forwarding-test-directory");
            state.now_micros_value = 777_777;
        }

        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let file_exists_name = String::from("env-wrapper-file-exists");
        let get_children_dir = String::from("env-wrapper-get-children");
        let get_file_size_name = String::from("env-wrapper-file-size");
        let mut children = vec![String::from("stale-entry")];
        let mut file_size: u64 = 0;
        let mut test_directory_path = String::from("stale-path");

        let exists = wrapper.file_exists(&file_exists_name);
        let get_children_status =
            wrapper.get_children(&get_children_dir, &mut children);
        let get_file_size_status =
            wrapper.get_file_size(&get_file_size_name, &mut file_size);
        let get_test_directory_status =
            wrapper.get_test_directory(&mut test_directory_path);
        let micros = wrapper.now_micros();

        wrapper.sleep_for_microseconds(88);

        assert!(exists);
        assert!(get_children_status.is_ok());
        assert!(get_file_size_status.is_ok());
        assert!(get_test_directory_status.is_ok());
        assert_eq!(file_size, 9191);
        assert_eq!(
            children,
            vec![String::from("alpha"), String::from("beta")],
        );
        assert_eq!(
            test_directory_path,
            String::from("env-wrapper-forwarding-test-directory"),
        );
        assert_eq!(micros, 777_777);

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(state.file_exists_count, 1);
        assert_eq!(state.file_exists_last_path, file_exists_name);

        assert_eq!(state.get_children_count, 1);
        assert_eq!(state.get_children_last_dir, get_children_dir);

        assert_eq!(state.get_file_size_count, 1);
        assert_eq!(state.get_file_size_last_path, get_file_size_name);

        assert_eq!(state.get_test_directory_count, 1);
        assert_eq!(state.now_micros_count, 1);

        assert_eq!(state.sleep_for_microseconds_count, 1);
        assert_eq!(state.sleep_for_microseconds_last, 88);
    }

    #[traced_test]
    fn env_wrapper_forwarding_schedule_and_start_thread_capture_callback_pointer_and_argument() {
        let shared_state =
            env_wrapper_serialized_target_access_test_build_recording_state();
        let target =
            env_wrapper_serialized_target_access_test_make_target(
                shared_state.clone(),
            );

        let mut wrapper = EnvWrapper::new(target);

        let mut schedule_cookie: usize = 11;
        let mut start_thread_cookie: usize = 22;

        let schedule_arg =
            &mut schedule_cookie as *mut usize as *mut c_void;
        let start_thread_arg =
            &mut start_thread_cookie as *mut usize as *mut c_void;

        wrapper.schedule(
            env_wrapper_serialized_target_access_test_never_invoke_callback,
            schedule_arg,
        );

        wrapper.start_thread(
            env_wrapper_serialized_target_access_test_never_invoke_callback,
            start_thread_arg,
        );

        let state =
            env_wrapper_serialized_target_access_test_lock_recording_state(
                &shared_state,
            );

        assert_eq!(state.schedule_count, 1);
        assert_eq!(
            state.schedule_last_function,
            env_wrapper_serialized_target_access_test_never_invoke_callback
                as usize
        );
        assert_eq!(state.schedule_last_arg, schedule_arg as usize);

        assert_eq!(state.start_thread_count, 1);
        assert_eq!(
            state.start_thread_last_function,
            env_wrapper_serialized_target_access_test_never_invoke_callback
                as usize
        );
        assert_eq!(state.start_thread_last_arg, start_thread_arg as usize);
    }
}
