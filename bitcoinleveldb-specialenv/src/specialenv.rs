// ---------------- [ File: bitcoinleveldb-specialenv/src/specialenv.rs ]
crate::ix!();

/**
  | Special Env used to delay background
  | operations.
  |
  */
#[derive(Getters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct SpecialEnv {

    base:                 crate::EnvWrapper,

    /**
      | sstable/log Sync() calls are blocked
      | while this pointer is non-null.
      |
      */
    delay_data_sync:      AtomicBool,

    /**
      | sstable/log Sync() calls return an
      | error.
      |
      */
    data_sync_error:      AtomicBool,

    /**
      | Simulate no-space errors while this
      | pointer is non-null.
      |
      */
    no_space:             AtomicBool,

    /**
      | Simulate non-writable file system
      | while this pointer is non-null.
      |
      */
    non_writable:         AtomicBool,

    /**
      | Force sync of manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_sync_error:  AtomicBool,

    /**
      | Force write to manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_write_error: AtomicBool,

    count_random_reads:   bool,
    random_read_counter:  AtomicCounter,
}

impl SpecialEnv {

    pub fn new(base: Rc<RefCell<dyn crate::Env>>) -> Self {
        trace!("SpecialEnv::new");

        Self {
            base:                    crate::EnvWrapper::new(base),
            delay_data_sync:         AtomicBool::new(false),
            data_sync_error:         AtomicBool::new(false),
            no_space:                AtomicBool::new(false),
            non_writable:            AtomicBool::new(false),
            manifest_sync_error:     AtomicBool::new(false),
            manifest_write_error:    AtomicBool::new(false),
            count_random_reads:      false,
            random_read_counter:     AtomicCounter::default(),
        }
    }

    pub fn new_writable_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(file = %f, "SpecialEnv::new_writable_file");

        if self.non_writable.load(std::sync::atomic::Ordering::Acquire) {
            warn!(file = %f, "SpecialEnv: rejecting write due to non_writable flag");
            let msg = Slice::from("simulated write error");
            return crate::Status::io_error(&msg, None);
        }

        let s = self.base.new_writable_file(f, r);

        if s.is_ok() && !r.is_null() {
            let base_ptr = unsafe { *r };

            if !base_ptr.is_null() {
                if f.contains(".ldb") || f.contains(".log") {
                    debug!(file = %f, "SpecialEnv: wrapping writable file as DataFile");
                    let wrapped: Box<dyn WritableFile> = Box::new(DataFile {
                        env: self as *const SpecialEnv,
                        base: base_ptr,
                    });
                    let wrapped_ptr: *mut Box<dyn WritableFile> = Box::into_raw(Box::new(wrapped));
                    unsafe { *r = wrapped_ptr };
                } else if f.contains("MANIFEST") {
                    debug!(file = %f, "SpecialEnv: wrapping writable file as ManifestFile");
                    let wrapped: Box<dyn WritableFile> = Box::new(ManifestFile {
                        env: self as *const SpecialEnv,
                        base: base_ptr,
                    });
                    let wrapped_ptr: *mut Box<dyn WritableFile> = Box::into_raw(Box::new(wrapped));
                    unsafe { *r = wrapped_ptr };
                } else {
                    trace!(file = %f, "SpecialEnv: leaving writable file unwrapped");
                }
            } else {
                warn!(file = %f, "SpecialEnv: underlying env returned OK but null writable file pointer");
            }
        }

        s
    }

    pub fn new_random_access_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn RandomAccessFile>,
    ) -> crate::Status {
        trace!(file = %f, "SpecialEnv::new_random_access_file");

        let s = self.base.new_random_access_file(f, r);

        if s.is_ok() && self.count_random_reads && !r.is_null() {
            let base_ptr = unsafe { *r };

            if !base_ptr.is_null() {
                debug!(file = %f, "SpecialEnv: wrapping random access file as CountingFile");
                let wrapped: Box<dyn RandomAccessFile> = Box::new(CountingFile {
                    target: base_ptr,
                    counter: &self.random_read_counter as *const AtomicCounter,
                });
                let wrapped_ptr: *mut Box<dyn RandomAccessFile> = Box::into_raw(Box::new(wrapped));
                unsafe { *r = wrapped_ptr };
            } else {
                warn!(file = %f, "SpecialEnv: underlying env returned OK but null random-access file pointer");
            }
        }

        s
    }
}

#[cfg(test)]
mod special_env_contract_suite {
    crate::ix!();

    use super::*;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    struct WritableCallState {
        append_calls: AtomicUsize,
        sync_calls:   AtomicUsize,
        drop_calls:   AtomicUsize,
    }

    impl WritableCallState {
        fn new() -> Self {
            Self {
                append_calls: AtomicUsize::new(0),
                sync_calls: AtomicUsize::new(0),
                drop_calls: AtomicUsize::new(0),
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
    }

    #[derive(Clone)]
    struct TrackingWritableFile {
        state: Arc<WritableCallState>,
    }

    impl WritableFile for TrackingWritableFile {}

    impl Named for TrackingWritableFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("TrackingWritableFile")
        }
    }

    impl WritableFileAppend for TrackingWritableFile {
        fn append(&mut self, _data: &Slice) -> crate::Status {
            debug!("TrackingWritableFile::append");
            self.state.append_calls.fetch_add(1, Ordering::SeqCst);
            crate::Status::ok()
        }
    }

    impl WritableFileClose for TrackingWritableFile {
        fn close(&mut self) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl WritableFileFlush for TrackingWritableFile {
        fn flush(&mut self) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl WritableFileSync for TrackingWritableFile {
        fn sync(&mut self) -> crate::Status {
            debug!("TrackingWritableFile::sync");
            self.state.sync_calls.fetch_add(1, Ordering::SeqCst);
            crate::Status::ok()
        }
    }

    impl Drop for TrackingWritableFile {
        fn drop(&mut self) {
            trace!("TrackingWritableFile::drop");
            self.state.drop_calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    struct RandomReadState {
        read_calls:  AtomicUsize,
        drop_calls:  AtomicUsize,
        last_offset: AtomicU64,
        last_n:      AtomicUsize,
    }

    impl RandomReadState {
        fn new() -> Self {
            Self {
                read_calls: AtomicUsize::new(0),
                drop_calls: AtomicUsize::new(0),
                last_offset: AtomicU64::new(0),
                last_n: AtomicUsize::new(0),
            }
        }

        fn read_calls(&self) -> usize {
            self.read_calls.load(Ordering::SeqCst)
        }

        fn drop_calls(&self) -> usize {
            self.drop_calls.load(Ordering::SeqCst)
        }

        fn last_offset(&self) -> u64 {
            self.last_offset.load(Ordering::SeqCst)
        }

        fn last_n(&self) -> usize {
            self.last_n.load(Ordering::SeqCst)
        }
    }

    #[derive(Clone)]
    struct TrackingRandomAccessFile {
        state: Arc<RandomReadState>,
        fill:  u8,
    }

    impl RandomAccessFile for TrackingRandomAccessFile {}

    impl Named for TrackingRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("TrackingRandomAccessFile")
        }
    }

    impl RandomAccessFileRead for TrackingRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> crate::Status {
            debug!(offset, n, "TrackingRandomAccessFile::read");
            self.state.read_calls.fetch_add(1, Ordering::SeqCst);
            self.state.last_offset.store(offset, Ordering::SeqCst);
            self.state.last_n.store(n, Ordering::SeqCst);

            if !result.is_null() {
                unsafe { *result = Slice::default() };
            }

            if n > 0 && !result.is_null() && !scratch.is_null() {
                unsafe {
                    let buf = std::slice::from_raw_parts_mut(scratch, n);
                    for b in buf.iter_mut() {
                        *b = self.fill;
                    }
                    *result = Slice::from_ptr_len(scratch as *const u8, n);
                }
            }

            crate::Status::ok()
        }
    }

    impl Drop for TrackingRandomAccessFile {
        fn drop(&mut self) {
            trace!("TrackingRandomAccessFile::drop");
            self.state.drop_calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[derive(Clone, Copy)]
    enum EnvReturnMode {
        OkWithFile = 0,
        OkWithNull = 1,
        ErrorIo    = 2,
    }

    struct HarnessEnv {
        writable_mode: AtomicU8,
        random_mode:   AtomicU8,

        writable_calls: AtomicUsize,
        random_calls:   AtomicUsize,

        last_writable_name: std::sync::Mutex<Option<String>>,
        last_random_name:   std::sync::Mutex<Option<String>>,

        writable_state: Arc<WritableCallState>,
        random_state:   Arc<RandomReadState>,
    }

    impl HarnessEnv {
        fn new(writable_state: Arc<WritableCallState>, random_state: Arc<RandomReadState>) -> Self {
            Self {
                writable_mode: AtomicU8::new(EnvReturnMode::OkWithFile as u8),
                random_mode: AtomicU8::new(EnvReturnMode::OkWithFile as u8),
                writable_calls: AtomicUsize::new(0),
                random_calls: AtomicUsize::new(0),
                last_writable_name: std::sync::Mutex::new(None),
                last_random_name: std::sync::Mutex::new(None),
                writable_state,
                random_state,
            }
        }

        fn set_writable_mode(&self, mode: EnvReturnMode) {
            self.writable_mode.store(mode as u8, Ordering::SeqCst);
        }

        fn set_random_mode(&self, mode: EnvReturnMode) {
            self.random_mode.store(mode as u8, Ordering::SeqCst);
        }

        fn writable_calls(&self) -> usize {
            self.writable_calls.load(Ordering::SeqCst)
        }

        fn random_calls(&self) -> usize {
            self.random_calls.load(Ordering::SeqCst)
        }

        fn last_writable_name(&self) -> Option<String> {
            self.last_writable_name.lock().unwrap().clone()
        }

        fn last_random_name(&self) -> Option<String> {
            self.last_random_name.lock().unwrap().clone()
        }

        fn allocate_writable_file_ptr(&self) -> *mut Box<dyn WritableFile> {
            let inner: Box<dyn WritableFile> = Box::new(TrackingWritableFile {
                state: self.writable_state.clone(),
            });
            Box::into_raw(Box::new(inner))
        }

        fn allocate_random_access_file_ptr(&self) -> *mut Box<dyn RandomAccessFile> {
            let inner: Box<dyn RandomAccessFile> = Box::new(TrackingRandomAccessFile {
                state: self.random_state.clone(),
                fill: b'r',
            });
            Box::into_raw(Box::new(inner))
        }
    }

    impl Env for HarnessEnv {}

    impl DeleteFile for HarnessEnv {
        fn delete_file(&mut self, _fname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl CreateDir for HarnessEnv {
        fn create_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl DeleteDir for HarnessEnv {
        fn delete_dir(&mut self, _dirname: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl NewSequentialFile for HarnessEnv {
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

    impl NewRandomAccessFile for HarnessEnv {
        fn new_random_access_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> crate::Status {
            self.random_calls.fetch_add(1, Ordering::SeqCst);
            {
                let mut guard = self.last_random_name.lock().unwrap();
                *guard = Some(fname.clone());
            }

            let mode = self.random_mode.load(Ordering::SeqCst);
            debug!(file = %fname, mode, "HarnessEnv::new_random_access_file");

            if result.is_null() {
                return crate::Status::ok();
            }

            match mode {
                x if x == EnvReturnMode::OkWithFile as u8 => {
                    unsafe { *result = self.allocate_random_access_file_ptr() };
                    crate::Status::ok()
                }
                x if x == EnvReturnMode::OkWithNull as u8 => {
                    unsafe { *result = std::ptr::null_mut() };
                    crate::Status::ok()
                }
                _ => {
                    unsafe { *result = std::ptr::null_mut() };
                    let msg = Slice::from("base env random access io error");
                    crate::Status::io_error(&msg, None)
                }
            }
        }
    }

    impl NewWritableFile for HarnessEnv {
        fn new_writable_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> crate::Status {
            self.writable_calls.fetch_add(1, Ordering::SeqCst);
            {
                let mut guard = self.last_writable_name.lock().unwrap();
                *guard = Some(fname.clone());
            }

            let mode = self.writable_mode.load(Ordering::SeqCst);
            debug!(file = %fname, mode, "HarnessEnv::new_writable_file");

            if result.is_null() {
                return crate::Status::ok();
            }

            match mode {
                x if x == EnvReturnMode::OkWithFile as u8 => {
                    unsafe { *result = self.allocate_writable_file_ptr() };
                    crate::Status::ok()
                }
                x if x == EnvReturnMode::OkWithNull as u8 => {
                    unsafe { *result = std::ptr::null_mut() };
                    crate::Status::ok()
                }
                _ => {
                    unsafe { *result = std::ptr::null_mut() };
                    let msg = Slice::from("base env writable io error");
                    crate::Status::io_error(&msg, None)
                }
            }
        }
    }

    impl NewAppendableFile for HarnessEnv {
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

    impl FileExists for HarnessEnv {
        fn file_exists(&mut self, _fname: &String) -> bool {
            false
        }
    }

    impl GetChildren for HarnessEnv {
        fn get_children(&mut self, _dir: &String, result: *mut Vec<String>) -> crate::Status {
            if !result.is_null() {
                unsafe { (*result).clear() };
            }
            crate::Status::ok()
        }
    }

    impl GetFileSize for HarnessEnv {
        fn get_file_size(&mut self, _fname: &String, file_size: *mut u64) -> crate::Status {
            if !file_size.is_null() {
                unsafe { *file_size = 0 };
            }
            crate::Status::ok()
        }
    }

    impl RenameFile for HarnessEnv {
        fn rename_file(&mut self, _src: &String, _target: &String) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl LockFile for HarnessEnv {
        fn lock_file(&mut self, _fname: &String, lock: *mut *mut Box<dyn FileLock>) -> crate::Status {
            if !lock.is_null() {
                unsafe { *lock = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl UnlockFile for HarnessEnv {
        fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl Schedule for HarnessEnv {
        fn schedule(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl StartThread for HarnessEnv {
        fn start_thread(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            _arg: *mut std::ffi::c_void,
        ) {
        }
    }

    impl GetTestDirectory for HarnessEnv {
        fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
            if !path.is_null() {
                unsafe {
                    *path = "harness-test-dir".to_string();
                }
            }
            crate::Status::ok()
        }
    }

    impl NewLogger for HarnessEnv {
        fn new_logger(&mut self, _fname: &String, result: *mut *mut Box<dyn Logger>) -> crate::Status {
            if !result.is_null() {
                unsafe { *result = std::ptr::null_mut() };
            }
            crate::Status::ok()
        }
    }

    impl NowMicros for HarnessEnv {
        fn now_micros(&mut self) -> u64 {
            0
        }
    }

    impl SleepForMicroseconds for HarnessEnv {
        fn sleep_for_microseconds(&mut self, _micros: i32) {}
    }

    fn make_special_env_with_harness(
        harness: Rc<RefCell<HarnessEnv>>,
    ) -> crate::SpecialEnv {
        let base: Rc<RefCell<dyn crate::Env>> = harness;
        crate::SpecialEnv::new(base)
    }

    #[traced_test]
    fn special_env_new_initializes_flags_and_counters_to_defaults() {
        trace!("test: special_env_new_initializes_flags_and_counters_to_defaults");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state)));
        let env = make_special_env_with_harness(harness);

        assert!(!env.delay_data_sync().load(Ordering::Acquire));
        assert!(!env.data_sync_error().load(Ordering::Acquire));
        assert!(!env.no_space().load(Ordering::Acquire));
        assert!(!env.non_writable().load(Ordering::Acquire));
        assert!(!env.manifest_sync_error().load(Ordering::Acquire));
        assert!(!env.manifest_write_error().load(Ordering::Acquire));

        assert_eq!(env.random_read_counter().read(), 0);
        assert!(!*env.count_random_reads());
    }

    #[traced_test]
    fn special_env_rejects_new_writable_file_when_non_writable_is_set() {
        trace!("test: special_env_rejects_new_writable_file_when_non_writable_is_set");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());
        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state)));

        let mut env = make_special_env_with_harness(harness.clone());
        env.non_writable().store(true, Ordering::Release);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_io_error());
        assert!(out.is_null());

        let calls = harness.borrow().writable_calls();
        info!(calls, "base env writable call count");
        assert_eq!(calls, 0);
    }








    #[traced_test]
    fn special_env_new_random_access_file_does_not_wrap_or_increment_when_base_returns_error() {
        trace!("test: special_env_new_random_access_file_does_not_wrap_or_increment_when_base_returns_error");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());
        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state.clone())));

        {
            let borrowed = harness.borrow();
            borrowed.set_random_mode(EnvReturnMode::ErrorIo);
        }

        let mut env = make_special_env_with_harness(harness.clone());
        env.set_count_random_reads(true);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

        let s = env.new_random_access_file(&fname, &mut out as *mut *mut Box<dyn RandomAccessFile>);
        assert!(s.is_io_error());
        assert!(out.is_null());

        assert_eq!(random_state.read_calls(), 0);
        assert_eq!(env.random_read_counter().read(), 0);
        assert_eq!(harness.borrow().random_calls(), 1);
    }

    #[traced_test]
    fn special_env_new_writable_file_handles_null_result_pointer_gracefully() {
        trace!("test: special_env_new_writable_file_handles_null_result_pointer_gracefully");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());
        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state)));

        let mut env = make_special_env_with_harness(harness.clone());

        let fname = "000001.ldb".to_string();
        let s = env.new_writable_file(&fname, std::ptr::null_mut());
        assert!(s.is_ok());

        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));
    }

    #[traced_test]
    fn special_env_new_writable_file_does_not_wrap_when_base_returns_ok_but_null_file_pointer() {
        trace!("test: special_env_new_writable_file_does_not_wrap_when_base_returns_ok_but_null_file_pointer");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());
        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state)));

        {
            let borrowed = harness.borrow();
            borrowed.set_writable_mode(EnvReturnMode::OkWithNull);
        }

        let mut env = make_special_env_with_harness(harness.clone());

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(out.is_null());

        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));
    }

    #[traced_test]
    fn special_env_new_random_access_file_does_not_wrap_when_base_returns_ok_but_null_file_pointer() {
        trace!("test: special_env_new_random_access_file_does_not_wrap_when_base_returns_ok_but_null_file_pointer");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());
        let harness = Rc::new(RefCell::new(HarnessEnv::new(writable_state, random_state)));

        {
            let borrowed = harness.borrow();
            borrowed.set_random_mode(EnvReturnMode::OkWithNull);
        }

        let mut env = make_special_env_with_harness(harness.clone());
        env.set_count_random_reads(true);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

        let s = env.new_random_access_file(&fname, &mut out as *mut *mut Box<dyn RandomAccessFile>);
        assert!(s.is_ok());
        assert!(out.is_null());

        assert_eq!(harness.borrow().random_calls(), 1);
        assert_eq!(harness.borrow().last_random_name(), Some(fname));
        assert_eq!(env.random_read_counter().read(), 0);
    }

    #[traced_test]
    fn special_env_wraps_ldb_writable_files_as_data_files_and_no_space_blocks_append() {
        trace!("test: special_env_wraps_ldb_writable_files_as_data_files_and_no_space_blocks_append");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness.clone());

        let fname = "000123.ldb".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        info!(
            file = %fname,
            base_calls = harness.borrow().writable_calls(),
            "new_writable_file returned ok"
        );
        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));

        let mut file = unsafe { Box::from_raw(out) };

        env.no_space().store(false, Ordering::Release);
        let payload = Slice::from("abc");

        let s1 = (&mut *file).append(&payload);
        assert!(s1.is_ok());
        debug!(append_calls = writable_state.append_calls(), "after normal append");
        assert_eq!(writable_state.append_calls(), 1);

        env.no_space().store(true, Ordering::Release);
        let s2 = (&mut *file).append(&payload);
        assert!(s2.is_ok());
        debug!(
            append_calls = writable_state.append_calls(),
            "after no_space append (should be dropped)"
        );
        assert_eq!(writable_state.append_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_wraps_log_writable_files_as_data_files_and_data_sync_error_blocks_sync() {
        trace!("test: special_env_wraps_log_writable_files_as_data_files_and_data_sync_error_blocks_sync");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness.clone());

        let fname = "000001.log".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));

        let mut file = unsafe { Box::from_raw(out) };

        env.data_sync_error().store(true, Ordering::Release);
        let se = (&mut *file).sync();
        assert!(se.is_io_error());
        debug!(sync_calls = writable_state.sync_calls(), "after simulated data sync error");
        assert_eq!(writable_state.sync_calls(), 0);

        env.data_sync_error().store(false, Ordering::Release);
        let so = (&mut *file).sync();
        assert!(so.is_ok());
        debug!(sync_calls = writable_state.sync_calls(), "after normal sync");
        assert_eq!(writable_state.sync_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_wraps_manifest_writable_files_and_manifest_flags_control_append_and_sync() {
        trace!("test: special_env_wraps_manifest_writable_files_and_manifest_flags_control_append_and_sync");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness.clone());

        let fname = "MANIFEST-000001".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));

        let mut file = unsafe { Box::from_raw(out) };
        let payload = Slice::from("manifest");

        env.manifest_write_error().store(true, Ordering::Release);
        let s1 = (&mut *file).append(&payload);
        assert!(s1.is_io_error());
        debug!(append_calls = writable_state.append_calls(), "after simulated manifest write error");
        assert_eq!(writable_state.append_calls(), 0);

        env.manifest_write_error().store(false, Ordering::Release);
        let s2 = (&mut *file).append(&payload);
        assert!(s2.is_ok());
        debug!(append_calls = writable_state.append_calls(), "after normal manifest append");
        assert_eq!(writable_state.append_calls(), 1);

        env.manifest_sync_error().store(true, Ordering::Release);
        let se = (&mut *file).sync();
        assert!(se.is_io_error());
        debug!(sync_calls = writable_state.sync_calls(), "after simulated manifest sync error");
        assert_eq!(writable_state.sync_calls(), 0);

        env.manifest_sync_error().store(false, Ordering::Release);
        let so = (&mut *file).sync();
        assert!(so.is_ok());
        debug!(sync_calls = writable_state.sync_calls(), "after normal manifest sync");
        assert_eq!(writable_state.sync_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_does_not_wrap_unrecognized_writable_files_and_no_space_does_not_affect_append() {
        trace!("test: special_env_does_not_wrap_unrecognized_writable_files_and_no_space_does_not_affect_append");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness.clone());
        env.no_space().store(true, Ordering::Release);

        let fname = "notes.txt".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        assert_eq!(harness.borrow().writable_calls(), 1);
        assert_eq!(harness.borrow().last_writable_name(), Some(fname));

        let mut file = unsafe { Box::from_raw(out) };
        let payload = Slice::from("payload");

        let sa = (&mut *file).append(&payload);
        assert!(sa.is_ok());

        debug!(append_calls = writable_state.append_calls(), "append calls after unwrapped write");
        assert_eq!(writable_state.append_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_data_file_wrapping_takes_precedence_over_manifest_when_both_substrings_present() {
        trace!("test: special_env_data_file_wrapping_takes_precedence_over_manifest_when_both_substrings_present");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness.clone());

        env.manifest_write_error().store(true, Ordering::Release);
        env.no_space().store(false, Ordering::Release);

        let fname = "MANIFEST.log".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        let mut file = unsafe { Box::from_raw(out) };
        let payload = Slice::from("payload");

        let sa = (&mut *file).append(&payload);
        assert!(sa.is_ok());

        debug!(
            append_calls = writable_state.append_calls(),
            "append calls after MANIFEST.log append (should delegate via DataFile)"
        );
        assert_eq!(writable_state.append_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_wraps_random_access_files_with_counting_file_when_enabled() {
        trace!("test: special_env_wraps_random_access_files_with_counting_file_when_enabled");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state,
            random_state.clone(),
        )));

        let mut env = make_special_env_with_harness(harness);
        env.set_count_random_reads(true);

        assert_eq!(env.random_read_counter().read(), 0);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

        let s = env.new_random_access_file(&fname, &mut out as *mut *mut Box<dyn RandomAccessFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        let file = unsafe { Box::from_raw(out) };

        let mut result = Slice::default();
        let mut scratch = vec![0u8; 5];

        let sr = (&**file).read(77, 5, &mut result as *mut Slice, scratch.as_mut_ptr());
        assert!(sr.is_ok());

        assert_eq!(random_state.read_calls(), 1);
        assert_eq!(random_state.last_offset(), 77);
        assert_eq!(random_state.last_n(), 5);

        assert_eq!(result.to_string(), "rrrrr");
        assert_eq!(env.random_read_counter().read(), 1);

        drop(file);
        assert_eq!(random_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_does_not_wrap_random_access_files_when_counting_disabled() {
        trace!("test: special_env_does_not_wrap_random_access_files_when_counting_disabled");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state,
            random_state.clone(),
        )));

        let mut env = make_special_env_with_harness(harness);
        env.set_count_random_reads(false);

        assert_eq!(env.random_read_counter().read(), 0);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();

        let s = env.new_random_access_file(&fname, &mut out as *mut *mut Box<dyn RandomAccessFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        let file = unsafe { Box::from_raw(out) };

        let mut result = Slice::default();
        let mut scratch = vec![0u8; 2];

        let sr = (&**file).read(1, 2, &mut result as *mut Slice, scratch.as_mut_ptr());
        assert!(sr.is_ok());

        assert_eq!(random_state.read_calls(), 1);
        assert_eq!(random_state.last_offset(), 1);
        assert_eq!(random_state.last_n(), 2);
        assert_eq!(result.to_string(), "rr");

        assert_eq!(env.random_read_counter().read(), 0);

        drop(file);
        assert_eq!(random_state.drop_calls(), 1);
    }

    #[traced_test]
    fn special_env_data_file_delay_flag_can_delay_sync_via_wrapped_file() {
        trace!("test: special_env_data_file_delay_flag_can_delay_sync_via_wrapped_file");

        let writable_state = Arc::new(WritableCallState::new());
        let random_state = Arc::new(RandomReadState::new());

        let harness = Rc::new(RefCell::new(HarnessEnv::new(
            writable_state.clone(),
            random_state,
        )));

        let mut env = make_special_env_with_harness(harness);

        let fname = "000001.ldb".to_string();
        let mut out: *mut Box<dyn WritableFile> = std::ptr::null_mut();

        let s = env.new_writable_file(&fname, &mut out as *mut *mut Box<dyn WritableFile>);
        assert!(s.is_ok());
        assert!(!out.is_null());

        env.delay_data_sync().store(true, Ordering::Release);
        let delay_flag = env.delay_data_sync();

        let mut file = unsafe { Box::from_raw(out) };

        std::thread::scope(|scope| {
            scope.spawn(|| {
                trace!("delay clearer thread started");
                std::thread::sleep(Duration::from_millis(10));
                delay_flag.store(false, Ordering::Release);
                trace!("delay clearer thread cleared delay_data_sync");
            });

            let start = Instant::now();
            let ss = (&mut *file).sync();
            let elapsed = start.elapsed();

            assert!(ss.is_ok());
            info!(?elapsed, "sync completed after delay");
            assert!(elapsed >= Duration::from_millis(80));
        });

        assert_eq!(writable_state.sync_calls(), 1);

        drop(file);
        assert_eq!(writable_state.drop_calls(), 1);

        assert!(!env.delay_data_sync().load(Ordering::Acquire));
    }
}
