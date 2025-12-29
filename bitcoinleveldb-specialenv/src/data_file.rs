// ---------------- [ File: bitcoinleveldb-specialenv/src/data_file.rs ]
crate::ix!();

pub struct DataFile {
    pub(crate) env:  *const SpecialEnv,
    pub(crate) base: *mut Box<dyn WritableFile>,
}

impl Drop for DataFile {
    fn drop(&mut self) {
        trace!("DataFile::drop");
        unsafe {
            if !self.base.is_null() {
                drop(Box::from_raw(self.base));
                self.base = std::ptr::null_mut();
            }
        }
    }
}

impl WritableFile for DataFile {}

impl Named for DataFile {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

impl WritableFileAppend for DataFile {
    fn append(&mut self, data: &Slice) -> crate::Status {
        let no_space = unsafe { (*self.env).no_space().load(std::sync::atomic::Ordering::Acquire) };
        if no_space {
            debug!("DataFile::append dropping write due to no_space flag");
            return crate::Status::ok();
        }

        unsafe { (&mut *self.base).append(data) }
    }
}

impl WritableFileClose for DataFile {
    fn close(&mut self) -> crate::Status {
        unsafe { (&mut *self.base).close() }
    }
}

impl WritableFileFlush for DataFile {
    fn flush(&mut self) -> crate::Status {
        unsafe { (&mut *self.base).flush() }
    }
}

impl WritableFileSync for DataFile {
    fn sync(&mut self) -> crate::Status {
        let data_sync_error =
            unsafe { (*self.env).data_sync_error().load(std::sync::atomic::Ordering::Acquire) };

        if data_sync_error {
            warn!("DataFile::sync returning simulated data sync error");
            let msg = Slice::from("simulated data sync error");
            return crate::Status::io_error(&msg, None);
        }

        while unsafe { (*self.env).delay_data_sync().load(std::sync::atomic::Ordering::Acquire) } {
            debug!("DataFile::sync delaying due to delay_data_sync flag");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        unsafe { (&mut *self.base).sync() }
    }
}

#[cfg(test)]
mod data_file_contract_suite {
    crate::ix!();

    use super::*;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    struct WritableCallState {
        append_calls:      AtomicUsize,
        close_calls:       AtomicUsize,
        flush_calls:       AtomicUsize,
        sync_calls:        AtomicUsize,
        drop_calls:        AtomicUsize,
        last_append_bytes: AtomicUsize,
        force_append_io:   AtomicBool,
        force_sync_io:     AtomicBool,
    }

    impl WritableCallState {
        fn new() -> Self {
            Self {
                append_calls: AtomicUsize::new(0),
                close_calls: AtomicUsize::new(0),
                flush_calls: AtomicUsize::new(0),
                sync_calls: AtomicUsize::new(0),
                drop_calls: AtomicUsize::new(0),
                last_append_bytes: AtomicUsize::new(0),
                force_append_io: AtomicBool::new(false),
                force_sync_io: AtomicBool::new(false),
            }
        }

        fn append_calls(&self) -> usize {
            self.append_calls.load(Ordering::SeqCst)
        }

        fn sync_calls(&self) -> usize {
            self.sync_calls.load(Ordering::SeqCst)
        }

        fn flush_calls(&self) -> usize {
            self.flush_calls.load(Ordering::SeqCst)
        }

        fn close_calls(&self) -> usize {
            self.close_calls.load(Ordering::SeqCst)
        }

        fn drop_calls(&self) -> usize {
            self.drop_calls.load(Ordering::SeqCst)
        }

        fn last_append_bytes(&self) -> usize {
            self.last_append_bytes.load(Ordering::SeqCst)
        }

        fn set_force_append_io(&self, v: bool) {
            self.force_append_io.store(v, Ordering::SeqCst);
        }

        fn set_force_sync_io(&self, v: bool) {
            self.force_sync_io.store(v, Ordering::SeqCst);
        }
    }

    #[derive(Clone)]
    struct CallTrackingWritableFile {
        state: Arc<WritableCallState>,
    }

    impl WritableFile for CallTrackingWritableFile {}

    impl Named for CallTrackingWritableFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("CallTrackingWritableFile")
        }
    }

    impl WritableFileAppend for CallTrackingWritableFile {
        fn append(&mut self, data: &Slice) -> crate::Status {
            let n = *data.size();
            debug!(bytes = n, "CallTrackingWritableFile::append");
            self.state.append_calls.fetch_add(1, Ordering::SeqCst);
            self.state.last_append_bytes.store(n, Ordering::SeqCst);

            if self.state.force_append_io.load(Ordering::SeqCst) {
                let msg = Slice::from("forced append io error");
                return crate::Status::io_error(&msg, None);
            }

            crate::Status::ok()
        }
    }

    impl WritableFileClose for CallTrackingWritableFile {
        fn close(&mut self) -> crate::Status {
            debug!("CallTrackingWritableFile::close");
            self.state.close_calls.fetch_add(1, Ordering::SeqCst);
            crate::Status::ok()
        }
    }

    impl WritableFileFlush for CallTrackingWritableFile {
        fn flush(&mut self) -> crate::Status {
            debug!("CallTrackingWritableFile::flush");
            self.state.flush_calls.fetch_add(1, Ordering::SeqCst);
            crate::Status::ok()
        }
    }

    impl WritableFileSync for CallTrackingWritableFile {
        fn sync(&mut self) -> crate::Status {
            debug!("CallTrackingWritableFile::sync");
            self.state.sync_calls.fetch_add(1, Ordering::SeqCst);

            if self.state.force_sync_io.load(Ordering::SeqCst) {
                let msg = Slice::from("forced sync io error");
                return crate::Status::io_error(&msg, None);
            }

            crate::Status::ok()
        }
    }

    impl Drop for CallTrackingWritableFile {
        fn drop(&mut self) {
            trace!("CallTrackingWritableFile::drop");
            self.state.drop_calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn allocate_writable_file_ptr(state: Arc<WritableCallState>) -> *mut Box<dyn WritableFile> {
        let inner: Box<dyn WritableFile> = Box::new(CallTrackingWritableFile { state });
        Box::into_raw(Box::new(inner))
    }

    struct NoopEnvForDataFileSuite;

    impl Env for NoopEnvForDataFileSuite {}

    impl DeleteFile for NoopEnvForDataFileSuite {
        fn delete_file(&mut self, _fname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl CreateDir for NoopEnvForDataFileSuite {
        fn create_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl DeleteDir for NoopEnvForDataFileSuite {
        fn delete_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl NewSequentialFile for NoopEnvForDataFileSuite {
        fn new_sequential_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn SequentialFile>,
        ) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NewRandomAccessFile for NoopEnvForDataFileSuite {
        fn new_random_access_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NewWritableFile for NoopEnvForDataFileSuite {
        fn new_writable_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NewAppendableFile for NoopEnvForDataFileSuite {
        fn new_appendable_file(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl FileExists for NoopEnvForDataFileSuite {
        fn file_exists(&mut self, _fname: &String) -> bool {
            false
        }
    }

    impl GetChildren for NoopEnvForDataFileSuite {
        fn get_children(&mut self, _dir: &String, result: *mut Vec<String>) -> crate::Status {
            if !result.is_null() {
                unsafe { (*result).clear() };
            }
            crate::Status::ok()
        }
    }

    impl GetFileSize for NoopEnvForDataFileSuite {
        fn get_file_size(&mut self, _fname: &String, file_size: *mut u64) -> crate::Status {
            if !file_size.is_null() {
                unsafe { *file_size = 0 };
            }
            crate::Status::ok()
        }
    }

    impl RenameFile for NoopEnvForDataFileSuite {
        fn rename_file(&mut self, _src: &String, _target: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl LockFile for NoopEnvForDataFileSuite {
        fn lock_file(&mut self, _fname: &String, lock: *mut *mut Box<dyn FileLock>) -> crate::Status {
            if !lock.is_null() {
                unsafe { *lock = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl UnlockFile for NoopEnvForDataFileSuite {
        fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl Schedule for NoopEnvForDataFileSuite {
        fn schedule(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl StartThread for NoopEnvForDataFileSuite {
        fn start_thread(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl GetTestDirectory for NoopEnvForDataFileSuite {
        fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
            if !path.is_null() {
                unsafe {
                    *path = "noop-test-dir".to_string();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewLogger for NoopEnvForDataFileSuite {
        fn new_logger(&mut self, _fname: &String, result: *mut *mut Box<dyn Logger>) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NowMicros for NoopEnvForDataFileSuite {
        fn now_micros(&mut self) -> u64 {
            0
        }
    }

    impl SleepForMicroseconds for NoopEnvForDataFileSuite {
        fn sleep_for_microseconds(&mut self, _micros: i32) {}
    }

    fn make_special_env_for_data_file_suite() -> crate::SpecialEnv {
        let base: Rc<RefCell<dyn crate::Env>> = Rc::new(RefCell::new(NoopEnvForDataFileSuite));
        crate::SpecialEnv::new(base)
    }

    #[traced_test]
    fn data_file_name_is_empty_string_as_documented() {
        trace!("test: data_file_name_is_empty_string_as_documented");

        let mut env = make_special_env_for_data_file_suite();
        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state);

        let df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let name = Named::name(&df);
        debug!(name = %name, "DataFile::name");
        assert_eq!(name.as_ref(), "");
    }

    #[traced_test]
    fn data_file_append_delegates_when_no_space_flag_is_clear() {
        trace!("test: data_file_append_delegates_when_no_space_flag_is_clear");

        let mut env = make_special_env_for_data_file_suite();
        env.no_space().store(false, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("abc");
        let s = WritableFileAppend::append(&mut df, &payload);
        assert!(s.is_ok());

        info!(
            append_calls = state.append_calls(),
            last_append_bytes = state.last_append_bytes(),
            "append delegated"
        );
        assert_eq!(state.append_calls(), 1);
        assert_eq!(state.last_append_bytes(), 3);
    }

    #[traced_test]
    fn data_file_append_drops_write_and_returns_ok_when_no_space_flag_is_set() {
        trace!("test: data_file_append_drops_write_and_returns_ok_when_no_space_flag_is_set");

        let mut env = make_special_env_for_data_file_suite();
        env.no_space().store(true, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("drop-me");
        let s = WritableFileAppend::append(&mut df, &payload);
        assert!(s.is_ok());

        info!(append_calls = state.append_calls(), "append calls after no_space write");
        assert_eq!(state.append_calls(), 0);
        assert_eq!(state.last_append_bytes(), 0);
    }

    #[traced_test]
    fn data_file_sync_returns_simulated_io_error_when_data_sync_error_flag_is_set() {
        trace!("test: data_file_sync_returns_simulated_io_error_when_data_sync_error_flag_is_set");

        let mut env = make_special_env_for_data_file_suite();
        env.data_sync_error().store(true, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let s = WritableFileSync::sync(&mut df);
        assert!(s.is_io_error());

        info!(sync_calls = state.sync_calls(), "sync calls after simulated error");
        assert_eq!(state.sync_calls(), 0);
    }

    #[traced_test]
    fn data_file_sync_delays_until_delay_data_sync_flag_is_cleared_then_delegates() {
        trace!("test: data_file_sync_delays_until_delay_data_sync_flag_is_cleared_then_delegates");

        let mut env = make_special_env_for_data_file_suite();
        env.delay_data_sync().store(true, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let delay_flag = env.delay_data_sync();

        std::thread::scope(|scope| {
            scope.spawn(|| {
                trace!("delay clearer thread started");
                std::thread::sleep(Duration::from_millis(10));
                delay_flag.store(false, Ordering::Release);
                trace!("delay clearer thread cleared delay_data_sync");
            });

            let start = Instant::now();
            let s = WritableFileSync::sync(&mut df);
            let elapsed = start.elapsed();

            assert!(s.is_ok());
            info!(?elapsed, "sync completed");
            assert_eq!(state.sync_calls(), 1);
            assert!(elapsed >= Duration::from_millis(80));
        });

        assert!(!env.delay_data_sync().load(Ordering::Acquire));
    }

    #[traced_test]
    fn data_file_flush_and_close_delegate_to_base_file() {
        trace!("test: data_file_flush_and_close_delegate_to_base_file");

        let mut env = make_special_env_for_data_file_suite();
        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let sf = WritableFileFlush::flush(&mut df);
        assert!(sf.is_ok());

        let sc = WritableFileClose::close(&mut df);
        assert!(sc.is_ok());

        info!(
            flush_calls = state.flush_calls(),
            close_calls = state.close_calls(),
            "observed flush/close delegation"
        );
        assert_eq!(state.flush_calls(), 1);
        assert_eq!(state.close_calls(), 1);
    }

    #[traced_test]
    fn data_file_drop_releases_underlying_writable_file_once() {
        trace!("test: data_file_drop_releases_underlying_writable_file_once");

        let mut env = make_special_env_for_data_file_suite();
        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        {
            let _df = DataFile {
                env: &env as *const crate::SpecialEnv,
                base: base_ptr,
            };
        }

        info!(drop_calls = state.drop_calls(), "underlying writable file drop count");
        assert_eq!(state.drop_calls(), 1);
    }

    #[traced_test]
    fn data_file_propagates_underlying_append_and_sync_errors_when_flags_do_not_override() {
        trace!("test: data_file_propagates_underlying_append_and_sync_errors_when_flags_do_not_override");

        let mut env = make_special_env_for_data_file_suite();
        env.no_space().store(false, Ordering::Release);
        env.data_sync_error().store(false, Ordering::Release);
        env.delay_data_sync().store(false, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        state.set_force_append_io(true);
        state.set_force_sync_io(true);

        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut df = DataFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("io");
        let sa = WritableFileAppend::append(&mut df, &payload);
        assert!(sa.is_io_error());

        let ss = WritableFileSync::sync(&mut df);
        assert!(ss.is_io_error());

        info!(
            append_calls = state.append_calls(),
            sync_calls = state.sync_calls(),
            "observed propagated errors"
        );
        assert_eq!(state.append_calls(), 1);
        assert_eq!(state.sync_calls(), 1);
    }
}
