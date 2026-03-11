crate::ix!();

/// Invariant: this adapter restores the C++ test topology in which `SpecialEnv` is passed
/// anywhere an `Env` is required, while preserving the overridden writable-file and
/// random-access-file behaviors that drive the fault-injection tests.
///
/// Precondition: `inner` is a live `SpecialEnv` allocation owned by the enclosing `DBTest`.
/// Postcondition: all forwarded operations observe the same underlying `SpecialEnv` state.
pub struct DBTestSpecialEnvDelegatingEnvAdapter {
    inner: *mut SpecialEnv,
}

impl DBTestSpecialEnvDelegatingEnvAdapter {
    /// Precondition: `inner` is non-null and remains live for the adapter lifetime.
    /// Postcondition: constructs a non-owning adapter over that `SpecialEnv`.
    pub fn new(inner: *mut SpecialEnv) -> Self {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new",
            inner_is_null = inner.is_null()
        );

        assert!(!inner.is_null());

        Self { inner }
    }
}

impl Env for DBTestSpecialEnvDelegatingEnvAdapter {}

impl DeleteFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn delete_file(&mut self, f: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.delete_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().delete_file(f) }
    }
}

impl CreateDir for DBTestSpecialEnvDelegatingEnvAdapter {
    fn create_dir(&mut self, d: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.create_dir",
            dir_len = d.len()
        );

        unsafe { (*self.inner).base_mut().create_dir(d) }
    }
}

impl DeleteDir for DBTestSpecialEnvDelegatingEnvAdapter {
    fn delete_dir(&mut self, d: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.delete_dir",
            dir_len = d.len()
        );

        unsafe { (*self.inner).base_mut().delete_dir(d) }
    }
}

impl NewSequentialFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn new_sequential_file(&mut self, f: &String, r: *mut *mut Box<dyn SequentialFile>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new_sequential_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().new_sequential_file(f, r) }
    }
}

impl NewRandomAccessFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn new_random_access_file(&mut self, f: &String, r: *mut *mut Box<dyn RandomAccessFile>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new_random_access_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).new_random_access_file(f, r) }
    }
}

impl NewWritableFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn new_writable_file(&mut self, f: &String, r: *mut *mut Box<dyn WritableFile>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new_writable_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).new_writable_file(f, r) }
    }
}

impl NewAppendableFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn new_appendable_file(&mut self, f: &String, r: *mut *mut Box<dyn WritableFile>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new_appendable_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().new_appendable_file(f, r) }
    }
}

impl FileExists for DBTestSpecialEnvDelegatingEnvAdapter {
    fn file_exists(&mut self, f: &String) -> bool {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.file_exists",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().file_exists(f) }
    }
}

impl GetChildren for DBTestSpecialEnvDelegatingEnvAdapter {
    fn get_children(&mut self, dir: &String, r: *mut Vec<String>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.get_children",
            dir_len = dir.len()
        );

        unsafe { (*self.inner).base_mut().get_children(dir, r) }
    }
}

impl GetFileSize for DBTestSpecialEnvDelegatingEnvAdapter {
    fn get_file_size(&mut self, f: &String, s: *mut u64) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.get_file_size",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().get_file_size(f, s) }
    }
}

impl RenameFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn rename_file(&mut self, src: &String, target: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.rename_file",
            src_len = src.len(),
            target_len = target.len()
        );

        unsafe { (*self.inner).base_mut().rename_file(src, target) }
    }
}

impl LockFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn lock_file(&mut self, f: &String, l: *mut *mut Box<dyn FileLock>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.lock_file",
            file_len = f.len()
        );

        unsafe { (*self.inner).base_mut().lock_file(f, l) }
    }
}

impl UnlockFile for DBTestSpecialEnvDelegatingEnvAdapter {
    fn unlock_file(&mut self, l: *mut Box<dyn FileLock>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.unlock_file"
        );

        unsafe { (*self.inner).base_mut().unlock_file(l) }
    }
}

impl Schedule for DBTestSpecialEnvDelegatingEnvAdapter {
    fn schedule(&mut self, f: fn(arg: *mut c_void) -> c_void, a: *mut c_void) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.schedule"
        );

        unsafe {
            (*self.inner).base_mut().schedule(f, a);
        }
    }
}

impl StartThread for DBTestSpecialEnvDelegatingEnvAdapter {
    fn start_thread(&mut self, f: fn(arg: *mut c_void) -> c_void, a: *mut c_void) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.start_thread"
        );

        unsafe {
            (*self.inner).base_mut().start_thread(f, a);
        }
    }
}

impl GetTestDirectory for DBTestSpecialEnvDelegatingEnvAdapter {
    fn get_test_directory(&mut self, path: *mut String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.get_test_directory"
        );

        unsafe { (*self.inner).base_mut().get_test_directory(path) }
    }
}

impl NewLogger for DBTestSpecialEnvDelegatingEnvAdapter {
    fn new_logger(&mut self, fname: &String, result: *mut *mut Box<dyn Logger>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.new_logger",
            file_len = fname.len()
        );

        unsafe { (*self.inner).base_mut().new_logger(fname, result) }
    }
}

impl NowMicros for DBTestSpecialEnvDelegatingEnvAdapter {
    fn now_micros(&mut self) -> u64 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.now_micros"
        );

        unsafe { (*self.inner).base_mut().now_micros() }
    }
}

impl SleepForMicroseconds for DBTestSpecialEnvDelegatingEnvAdapter {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
            label = "dbtest.special_env_delegating_env_adapter.sleep_for_microseconds",
            micros = micros
        );

        unsafe {
            (*self.inner).base_mut().sleep_for_microseconds(micros);
        }
    }
}

/// Invariant: returns an `Rc<RefCell<dyn Env>>` view whose method dispatch routes through
/// the exact `SpecialEnv` instance owned by the `DBTest`.
///
/// Precondition: `inner` is non-null and live for the lifetime of any DB opened with it.
/// Postcondition: operations through the returned environment observe `SpecialEnv` flags.
pub fn dbtest_special_env_delegating_env_rc(
    inner: *mut SpecialEnv,
) -> Rc<RefCell<dyn Env>> {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
        label = "dbtest.special_env_delegating_env_rc.entry",
        inner_is_null = inner.is_null()
    );

    assert!(!inner.is_null());

    let out: Rc<RefCell<dyn Env>> =
        Rc::new(RefCell::new(DBTestSpecialEnvDelegatingEnvAdapter::new(inner)));

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::special_env_delegating_env_adapter",
        label = "dbtest.special_env_delegating_env_rc.exit"
    );

    out
}
