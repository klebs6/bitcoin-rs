// ---------------- [ File: bitcoinleveldb-specialenv/src/manifest_file.rs ]
crate::ix!();

pub struct ManifestFile {
    pub(crate) env:  *const SpecialEnv,
    pub(crate) base: *mut Box<dyn WritableFile>,
}

impl Drop for ManifestFile {
    fn drop(&mut self) {
        trace!("ManifestFile::drop");
        unsafe {
            if !self.base.is_null() {
                drop(Box::from_raw(self.base));
                self.base = std::ptr::null_mut();
            }
        }
    }
}

impl WritableFile for ManifestFile {}

impl Named for ManifestFile {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

impl WritableFileAppend for ManifestFile {
    fn append(&mut self, data: &Slice) -> crate::Status {
        let manifest_write_error =
            unsafe { (*self.env).manifest_write_error().load(std::sync::atomic::Ordering::Acquire) };

        if manifest_write_error {
            warn!("ManifestFile::append returning simulated writer error");
            let msg = Slice::from("simulated writer error");
            return crate::Status::io_error(&msg, None);
        }

        unsafe { (&mut *self.base).append(data) }
    }
}

impl WritableFileClose for ManifestFile {
    fn close(&mut self) -> crate::Status {
        unsafe { (&mut *self.base).close() }
    }
}

impl WritableFileFlush for ManifestFile {
    fn flush(&mut self) -> crate::Status {
        unsafe { (&mut *self.base).flush() }
    }
}

impl WritableFileSync for ManifestFile {
    fn sync(&mut self) -> crate::Status {
        let manifest_sync_error =
            unsafe { (*self.env).manifest_sync_error().load(std::sync::atomic::Ordering::Acquire) };

        if manifest_sync_error {
            warn!("ManifestFile::sync returning simulated sync error");
            let msg = Slice::from("simulated sync error");
            return crate::Status::io_error(&msg, None);
        }

        unsafe { (&mut *self.base).sync() }
    }
}

#[cfg(test)]
mod manifest_file_contract_suite {
    crate::ix!();

    use super::*;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    struct WritableCallState {
        append_calls:    AtomicUsize,
        close_calls:     AtomicUsize,
        flush_calls:     AtomicUsize,
        sync_calls:      AtomicUsize,
        drop_calls:      AtomicUsize,
        force_append_io: AtomicBool,
        force_sync_io:   AtomicBool,
    }

    impl WritableCallState {
        fn new() -> Self {
            Self {
                append_calls: AtomicUsize::new(0),
                close_calls: AtomicUsize::new(0),
                flush_calls: AtomicUsize::new(0),
                sync_calls: AtomicUsize::new(0),
                drop_calls: AtomicUsize::new(0),
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

        fn drop_calls(&self) -> usize {
            self.drop_calls.load(Ordering::SeqCst)
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
        fn append(&mut self, _data: &Slice) -> crate::Status {
            debug!("CallTrackingWritableFile::append");
            self.state.append_calls.fetch_add(1, Ordering::SeqCst);

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

    struct NoopEnvForManifestFileSuite;

    impl Env for NoopEnvForManifestFileSuite {}

    impl DeleteFile for NoopEnvForManifestFileSuite {
        fn delete_file(&mut self, _fname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl CreateDir for NoopEnvForManifestFileSuite {
        fn create_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl DeleteDir for NoopEnvForManifestFileSuite {
        fn delete_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl NewSequentialFile for NoopEnvForManifestFileSuite {
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

    impl NewRandomAccessFile for NoopEnvForManifestFileSuite {
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

    impl NewWritableFile for NoopEnvForManifestFileSuite {
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

    impl NewAppendableFile for NoopEnvForManifestFileSuite {
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

    impl FileExists for NoopEnvForManifestFileSuite {
        fn file_exists(&mut self, _fname: &String) -> bool {
            false
        }
    }

    impl GetChildren for NoopEnvForManifestFileSuite {
        fn get_children(&mut self, _dir: &String, result: *mut Vec<String>) -> crate::Status {
            if !result.is_null() {
                unsafe { (*result).clear() };
            }
            crate::Status::ok()
        }
    }

    impl GetFileSize for NoopEnvForManifestFileSuite {
        fn get_file_size(&mut self, _fname: &String, file_size: *mut u64) -> crate::Status {
            if !file_size.is_null() {
                unsafe { *file_size = 0 };
            }
            crate::Status::ok()
        }
    }

    impl RenameFile for NoopEnvForManifestFileSuite {
        fn rename_file(&mut self, _src: &String, _target: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl LockFile for NoopEnvForManifestFileSuite {
        fn lock_file(&mut self, _fname: &String, lock: *mut *mut Box<dyn FileLock>) -> crate::Status {
            if !lock.is_null() {
                unsafe { *lock = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl UnlockFile for NoopEnvForManifestFileSuite {
        fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl Schedule for NoopEnvForManifestFileSuite {
        fn schedule(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl StartThread for NoopEnvForManifestFileSuite {
        fn start_thread(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl GetTestDirectory for NoopEnvForManifestFileSuite {
        fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
            if !path.is_null() {
                unsafe {
                    *path = "noop-test-dir".to_string();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewLogger for NoopEnvForManifestFileSuite {
        fn new_logger(&mut self, _fname: &String, result: *mut *mut Box<dyn Logger>) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NowMicros for NoopEnvForManifestFileSuite {
        fn now_micros(&mut self) -> u64 {
            0
        }
    }

    impl SleepForMicroseconds for NoopEnvForManifestFileSuite {
        fn sleep_for_microseconds(&mut self, _micros: i32) {}
    }

    fn make_special_env_for_manifest_file_suite() -> crate::SpecialEnv {
        let base: Rc<RefCell<dyn crate::Env>> = Rc::new(RefCell::new(NoopEnvForManifestFileSuite));
        crate::SpecialEnv::new(base)
    }

    #[traced_test]
    fn manifest_file_name_is_empty_string_as_documented() {
        trace!("test: manifest_file_name_is_empty_string_as_documented");

        let mut env = make_special_env_for_manifest_file_suite();
        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state);

        let mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let name = Named::name(&mf);
        debug!(name = %name, "ManifestFile::name");
        assert_eq!(name.as_ref(), "");
    }

    #[traced_test]
    fn manifest_file_append_delegates_when_manifest_write_error_flag_is_clear() {
        trace!("test: manifest_file_append_delegates_when_manifest_write_error_flag_is_clear");

        let mut env = make_special_env_for_manifest_file_suite();
        env.manifest_write_error().store(false, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("manifest-bytes");
        let s = WritableFileAppend::append(&mut mf, &payload);
        assert!(s.is_ok());
        assert_eq!(state.append_calls(), 1);
    }

    #[traced_test]
    fn manifest_file_append_returns_simulated_io_error_when_manifest_write_error_flag_is_set() {
        trace!("test: manifest_file_append_returns_simulated_io_error_when_manifest_write_error_flag_is_set");

        let mut env = make_special_env_for_manifest_file_suite();
        env.manifest_write_error().store(true, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("manifest-bytes");
        let s = WritableFileAppend::append(&mut mf, &payload);
        assert!(s.is_io_error());
        assert_eq!(state.append_calls(), 0);
    }

    #[traced_test]
    fn manifest_file_sync_delegates_when_manifest_sync_error_flag_is_clear() {
        trace!("test: manifest_file_sync_delegates_when_manifest_sync_error_flag_is_clear");

        let mut env = make_special_env_for_manifest_file_suite();
        env.manifest_sync_error().store(false, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let s = WritableFileSync::sync(&mut mf);
        assert!(s.is_ok());
        assert_eq!(state.sync_calls(), 1);
    }

    #[traced_test]
    fn manifest_file_sync_returns_simulated_io_error_when_manifest_sync_error_flag_is_set() {
        trace!("test: manifest_file_sync_returns_simulated_io_error_when_manifest_sync_error_flag_is_set");

        let mut env = make_special_env_for_manifest_file_suite();
        env.manifest_sync_error().store(true, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let s = WritableFileSync::sync(&mut mf);
        assert!(s.is_io_error());
        assert_eq!(state.sync_calls(), 0);
    }

    #[traced_test]
    fn manifest_file_drop_releases_underlying_writable_file_once() {
        trace!("test: manifest_file_drop_releases_underlying_writable_file_once");

        let mut env = make_special_env_for_manifest_file_suite();
        let state = Arc::new(WritableCallState::new());
        let base_ptr = allocate_writable_file_ptr(state.clone());

        {
            let _mf = ManifestFile {
                env: &env as *const crate::SpecialEnv,
                base: base_ptr,
            };
        }

        assert_eq!(state.drop_calls(), 1);
    }

    #[traced_test]
    fn manifest_file_propagates_underlying_append_and_sync_errors_when_flags_do_not_override() {
        trace!("test: manifest_file_propagates_underlying_append_and_sync_errors_when_flags_do_not_override");

        let mut env = make_special_env_for_manifest_file_suite();
        env.manifest_write_error().store(false, Ordering::Release);
        env.manifest_sync_error().store(false, Ordering::Release);

        let state = Arc::new(WritableCallState::new());
        state.set_force_append_io(true);
        state.set_force_sync_io(true);

        let base_ptr = allocate_writable_file_ptr(state.clone());

        let mut mf = ManifestFile {
            env: &env as *const crate::SpecialEnv,
            base: base_ptr,
        };

        let payload = Slice::from("io");
        let sa = WritableFileAppend::append(&mut mf, &payload);
        assert!(sa.is_io_error());

        let ss = WritableFileSync::sync(&mut mf);
        assert!(ss.is_io_error());

        assert_eq!(state.append_calls(), 1);
        assert_eq!(state.sync_calls(), 1);
    }
}
