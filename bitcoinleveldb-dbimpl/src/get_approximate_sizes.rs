// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_approximate_sizes.rs ]
crate::ix!();

impl DBGetApproximateSizes for DBImpl {
    fn get_approximate_sizes(&mut self, range: *const Range, n: i32, sizes: *mut u64) {
        // TODO(opt): better implementation
        self.mutex.lock();
        let v: *mut Version = unsafe { (*self.versions).current() };
        unsafe {
            (*v).ref_();
        }

        for i in 0..n {
            let r: &Range = unsafe { &*range.add(i as usize) };

            // Convert user_key into a corresponding internal key.
            let k1: InternalKey =
                InternalKey::new(r.start(), MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            let k2: InternalKey =
                InternalKey::new(r.limit(), MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);

            let start: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k1) };
            let limit: u64 = unsafe { (*self.versions).approximate_offset_of(v, &k2) };

            unsafe {
                *sizes.add(i as usize) = if limit >= start { limit - start } else { 0 };
            }
        }

        unsafe {
            (*v).unref();
        }
        unsafe { self.mutex.unlock() };
    }
}

#[cfg(test)]
#[disable]
mod db_get_approximate_sizes_contract_suite {
    crate::ix!();

    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::ffi::c_void;
    use std::rc::Rc;

    #[derive(Default)]
    struct NoopLogger;
    impl Logger for NoopLogger {}

    #[derive(Default)]
    struct NoopFileLock;
    impl FileLock for NoopFileLock {}

    #[derive(Default)]
    struct NoopSequentialFile;
    impl SequentialFile for NoopSequentialFile {}
    impl SequentialFileRead for NoopSequentialFile {
        fn read(&mut self, _n: usize, result: *mut Slice, _scratch: *mut u8) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = Slice::empty();
                }
            }
            Status::ok()
        }
    }
    impl SequentialFileSkip for NoopSequentialFile {
        fn skip(&mut self, _n: u64) -> crate::Status {
            Status::ok()
        }
    }

    #[derive(Default)]
    struct NoopRandomAccessFile;
    impl RandomAccessFile for NoopRandomAccessFile {}
    impl RandomAccessFileRead for NoopRandomAccessFile {
        fn read(&self, _offset: u64, _n: usize, result: *mut Slice, _scratch: *mut u8) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    *result = Slice::empty();
                }
            }
            Status::ok()
        }
    }

    #[derive(Default)]
    struct NoopWritableFile;
    impl WritableFile for NoopWritableFile {}
    impl WritableFileAppend for NoopWritableFile {
        fn append(&mut self, _data: &Slice) -> crate::Status {
            Status::ok()
        }
    }
    impl WritableFileClose for NoopWritableFile {
        fn close(&mut self) -> crate::Status {
            Status::ok()
        }
    }
    impl WritableFileFlush for NoopWritableFile {
        fn flush(&mut self) -> crate::Status {
            Status::ok()
        }
    }
    impl WritableFileSync for NoopWritableFile {
        fn sync(&mut self) -> crate::Status {
            Status::ok()
        }
    }

    #[derive(Default)]
    struct NoopEnv {
        _files: HashMap<String, Vec<u8>>,
    }

    impl Env for NoopEnv {}

    impl DeleteFile for NoopEnv {
        fn delete_file(&mut self, _fname: &String) -> crate::Status {
            Status::ok()
        }
    }
    impl CreateDir for NoopEnv {
        fn create_dir(&mut self, _dirname: &String) -> crate::Status {
            Status::ok()
        }
    }
    impl DeleteDir for NoopEnv {
        fn delete_dir(&mut self, _dirname: &String) -> crate::Status {
            Status::ok()
        }
    }
    impl NewSequentialFile for NoopEnv {
        fn new_sequential_file(&mut self, _fname: &String, result: *mut *mut Box<dyn SequentialFile>) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    let f: Box<dyn SequentialFile> = Box::new(NoopSequentialFile::default());
                    let ptr: *mut Box<dyn SequentialFile> = Box::into_raw(Box::new(f));
                    *result = ptr;
                }
            }
            Status::ok()
        }
    }
    impl NewRandomAccessFile for NoopEnv {
        fn new_random_access_file(&mut self, _fname: &String, result: *mut *mut Box<dyn RandomAccessFile>) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    let f: Box<dyn RandomAccessFile> = Box::new(NoopRandomAccessFile::default());
                    let ptr: *mut Box<dyn RandomAccessFile> = Box::into_raw(Box::new(f));
                    *result = ptr;
                }
            }
            Status::ok()
        }
    }
    impl NewWritableFile for NoopEnv {
        fn new_writable_file(&mut self, _fname: &String, result: *mut *mut Box<dyn WritableFile>) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    let f: Box<dyn WritableFile> = Box::new(NoopWritableFile::default());
                    let ptr: *mut Box<dyn WritableFile> = Box::into_raw(Box::new(f));
                    *result = ptr;
                }
            }
            Status::ok()
        }
    }
    impl NewAppendableFile for NoopEnv {
        fn new_appendable_file(&mut self, _fname: &String, result: *mut *mut Box<dyn WritableFile>) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    let f: Box<dyn WritableFile> = Box::new(NoopWritableFile::default());
                    let ptr: *mut Box<dyn WritableFile> = Box::into_raw(Box::new(f));
                    *result = ptr;
                }
            }
            Status::ok()
        }
    }
    impl FileExists for NoopEnv {
        fn file_exists(&mut self, _fname: &String) -> bool {
            false
        }
    }
    impl GetChildren for NoopEnv {
        fn get_children(&mut self, _dir: &String, result: *mut Vec<String>) -> crate::Status {
            unsafe {
                if !result.is_null() {
                    (*result).clear();
                }
            }
            Status::ok()
        }
    }
    impl GetFileSize for NoopEnv {
        fn get_file_size(&mut self, _fname: &String, file_size: *mut u64) -> crate::Status {
            unsafe {
                if !file_size.is_null() {
                    *file_size = 0;
                }
            }
            Status::ok()
        }
    }
    impl RenameFile for NoopEnv {
        fn rename_file(&mut self, _src: &String, _target: &String) -> crate::Status {
            Status::ok()
        }
    }
    impl LockFile for NoopEnv {
        fn lock_file(&mut self, _fname: &String, lock: *mut *mut Box<dyn FileLock>) -> crate::Status {
            unsafe {
                if !lock.is_null() {
                    let l: Box<dyn FileLock> = Box::new(NoopFileLock::default());
                    let ptr: *mut Box<dyn FileLock> = Box::into_raw(Box::new(l));
                    *lock = ptr;
                }
            }
            Status::ok()
        }
    }
    impl UnlockFile for NoopEnv {
        fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
            unsafe {
                if !lock.is_null() {
                    drop(Box::from_raw(lock));
                }
            }
            Status::ok()
        }
    }
    impl Schedule for NoopEnv {
        fn schedule(&mut self, _function: fn(arg: *mut c_void) -> c_void, _arg: *mut c_void) {}
    }
    impl StartThread for NoopEnv {
        fn start_thread(&mut self, _function: fn(arg: *mut c_void) -> c_void, _arg: *mut c_void) {}
    }
    impl GetTestDirectory for NoopEnv {
        fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
            unsafe {
                if !path.is_null() {
                    *path = "noop_test_dir".to_string();
                }
            }
            Status::ok()
        }
    }
    impl NewLogger for NoopEnv {
        fn new_logger(&mut self, _fname: &String, result: *mut *mut Box<dyn Logger>) -> Status {
            unsafe {
                if !result.is_null() {
                    let l: Box<dyn Logger> = Box::new(NoopLogger::default());
                    let ptr: *mut Box<dyn Logger> = Box::into_raw(Box::new(l));
                    *result = ptr;
                }
            }
            Status::ok()
        }
    }
    impl NowMicros for NoopEnv {
        fn now_micros(&mut self) -> u64 {
            0
        }
    }
    impl SleepForMicroseconds for NoopEnv {
        fn sleep_for_microseconds(&mut self, _micros: i32) {}
    }

    struct LeakedDbImplHarness {
        db: &'static mut DBImpl,
    }

    impl LeakedDbImplHarness {
        fn build_empty() -> Self {
            let env_rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(NoopEnv::default()));
            let mut opts: Options = Options::with_env(env_rc.clone());
            opts.set_compression(CompressionType::None);

            let leaked_options: &'static mut Options = Box::leak(Box::new(opts));
            let dbname: String = "db_get_approximate_sizes_contract_suite".to_string();

            let internal_comparator: InternalKeyComparator =
                InternalKeyComparator::new(bytewise_comparator());
            let internal_filter_policy: InternalFilterPolicy = {
                static NULL_POLICY: NullFilterPolicy = NullFilterPolicy;
                let p: *const dyn FilterPolicy = &NULL_POLICY as &dyn FilterPolicy;
                InternalFilterPolicy::new(p)
            };

            let table_cache: *mut TableCache =
                Box::into_raw(Box::new(TableCache::new(&dbname, leaked_options, 10)));

            let versions: *mut VersionSet = Box::into_raw(Box::new(VersionSet::new(
                &dbname,
                leaked_options as *const Options,
                table_cache,
                &internal_comparator as *const InternalKeyComparator,
            )));

            let db_lock: Rc<RefCell<dyn FileLock>> = Rc::new(RefCell::new(NoopFileLock::default()));
            let logfile: Rc<RefCell<dyn WritableFile>> =
                Rc::new(RefCell::new(NoopWritableFile::default()));
            let tmp_batch: *mut WriteBatch = Box::into_raw(Box::new(WriteBatch::default()));

            let mem: *mut MemTable =
                Box::into_raw(Box::new(MemTable::new(&internal_comparator)));
            unsafe {
                (*mem).ref_();
            }

            let db: &'static mut DBImpl = Box::leak(Box::new(DBImpl {
                env: Box::new(EnvWrapper::new(env_rc.clone())),
                internal_comparator,
                internal_filter_policy,
                options: leaked_options.clone(),
                owns_info_log: false,
                owns_cache: false,
                dbname,
                table_cache: table_cache as *const TableCache,
                db_lock,
                mutex: RawMutex::default(),
                background_work_finished_signal: Condvar::new(),
                imm: core::ptr::null_mut(),
                logfile_number: 0,
                seed: 0,
                writers: VecDeque::new(),
                tmp_batch,
                snapshots: SnapshotList::default(),
                pending_outputs: HashSet::new(),
                background_compaction_scheduled: false,
                manual_compaction: core::ptr::null_mut(),
                versions: versions as *const VersionSet,
                bg_error: Status::ok(),
                stats: core::array::from_fn(|_| CompactionStats::default()),
                shutting_down: AtomicBool::new(false),
                mem,
                has_imm: AtomicBool::new(false),
                logfile,
                log: core::ptr::null_mut(),
            }));

            Self { db }
        }
    }

    #[traced_test]
    fn db_get_approximate_sizes_noop_when_n_is_zero() {
        let mut h = LeakedDbImplHarness::build_empty();

        let ranges: [Range; 1] = [Range::new(Slice::from_str("a"), Slice::from_str("z"))];
        let mut sizes: [u64; 2] = [0xA5A5_A5A5_A5A5_A5A5, 0x5A5A_5A5A_5A5A_5A5A];

        tracing::info!("Calling DBGetApproximateSizes::get_approximate_sizes with n=0");
        <DBImpl as DBGetApproximateSizes>::get_approximate_sizes(
            h.db,
            ranges.as_ptr(),
            0,
            sizes.as_mut_ptr(),
        );

        assert_eq!(
            sizes[0], 0xA5A5_A5A5_A5A5_A5A5,
            "sizes[0] must remain unchanged when n=0"
        );
        assert_eq!(
            sizes[1], 0x5A5A_5A5A_5A5A_5A5A,
            "sizes[1] must remain unchanged when n=0"
        );

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_approximate_sizes_writes_zero_for_empty_version_set() {
        let mut h = LeakedDbImplHarness::build_empty();

        let ranges: [Range; 3] = [
            Range::new(Slice::from_str("a"), Slice::from_str("b")),
            Range::new(Slice::from_str("k"), Slice::from_str("k")),
            Range::new(Slice::from_str("x"), Slice::from_str("z")),
        ];

        let mut sizes: [u64; 3] = [u64::MAX, u64::MAX, u64::MAX];

        tracing::info!("Calling DBGetApproximateSizes::get_approximate_sizes with an empty VersionSet");
        <DBImpl as DBGetApproximateSizes>::get_approximate_sizes(
            h.db,
            ranges.as_ptr(),
            ranges.len() as i32,
            sizes.as_mut_ptr(),
        );

        tracing::debug!(sizes = ?sizes, "Approximate sizes returned");

        for (i, s) in sizes.iter().copied().enumerate() {
            assert_eq!(
                s, 0,
                "empty VersionSet should produce 0 approximate size (index={i})"
            );
        }

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }
}
