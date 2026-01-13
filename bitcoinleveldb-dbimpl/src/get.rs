// ---------------- [ File: bitcoinleveldb-dbimpl/src/get.rs ]
crate::ix!();

impl DBGet for DBImpl {
    fn get(&mut self, options: &ReadOptions, key_: &Slice, value: *mut String) -> crate::Status {
        let mut s: Status = Status::ok();

        self.mutex.lock();

        let snapshot: SequenceNumber = match options.snapshot().as_ref() {
            Some(snap) => {
                // SAFETY: DBImpl only ever hands out SnapshotImpl instances via the public Snapshot
                // interface, so the trait object data pointer is a SnapshotImpl.
                let raw: *const dyn Snapshot = Arc::as_ptr(snap);
                let data: *const () = raw as *const ();
                let snap_impl: *const SnapshotImpl = data as *const SnapshotImpl;

                unsafe { *(*snap_impl).sequence_number() }
            }
            None => unsafe { (*self.versions).last_sequence() },
        };

        let mem: *mut MemTable = self.mem;
        let imm: *mut MemTable = self.imm;
        let current: *mut Version = unsafe { (*self.versions).current() };

        unsafe {
            (*mem).ref_();
            if !imm.is_null() {
                (*imm).ref_();
            }
            (*current).ref_();
        }

        let mut have_stat_update: bool = false;
        let mut stats: VersionGetStats = Default::default();

        // Unlock while reading from files and memtables
        unsafe { self.mutex.unlock() };

        {
            // First look in the memtable, then in the immutable memtable (if any).
            let lkey: LookupKey = LookupKey::new(key_, snapshot);

            if unsafe { (*mem).get(&lkey, value, &mut s) } {
                // Done
            } else if !imm.is_null() && unsafe { (*imm).get(&lkey, value, &mut s) } {
                // Done
            } else {
                s = unsafe { (*current).get(options, &lkey, value, &mut stats) };
                have_stat_update = true;
            }
        }

        self.mutex.lock();

        if have_stat_update && unsafe { (*current).update_stats(&mut stats) } {
            self.maybe_schedule_compaction();
        }

        unsafe {
            (*mem).unref();
            if !imm.is_null() {
                (*imm).unref();
            }
            (*current).unref();
        }

        unsafe { self.mutex.unlock() };

        s
    }
}

#[cfg(test)]
#[disable]
mod db_get_contract_suite {
    crate::ix!();

    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::ffi::c_void;
    use std::rc::Rc;
    use std::sync::Arc;

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
        versions: *mut VersionSet,
    }

    impl LeakedDbImplHarness {
        fn build_with_mem_and_imm() -> Self {
            let env_rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(NoopEnv::default()));
            let mut opts: Options = Options::with_env(env_rc.clone());
            opts.set_compression(CompressionType::None);

            let leaked_options: &'static mut Options = Box::leak(Box::new(opts));
            let dbname: String = "db_get_contract_suite".to_string();

            let internal_comparator: InternalKeyComparator =
                InternalKeyComparator::new(bytewise_comparator());
            let internal_filter_policy: InternalFilterPolicy = {
                static NULL_POLICY: NullFilterPolicy = NullFilterPolicy;
                let p: *const dyn FilterPolicy = &NULL_POLICY as &dyn FilterPolicy;
                InternalFilterPolicy::new(p)
            };

            let table_cache: *mut TableCache =
                Box::into_raw(Box::new(TableCache::new(&dbname, leaked_options, 10)));

            let db_lock: Rc<RefCell<dyn FileLock>> = Rc::new(RefCell::new(NoopFileLock::default()));
            let logfile: Rc<RefCell<dyn WritableFile>> =
                Rc::new(RefCell::new(NoopWritableFile::default()));

            let tmp_batch: *mut WriteBatch = Box::into_raw(Box::new(WriteBatch::default()));

            let mut db_box: Box<DBImpl> = Box::new(DBImpl {
                env: Box::new(EnvWrapper::new(env_rc.clone())),
                internal_comparator,
                internal_filter_policy,
                options: leaked_options.clone(),
                owns_info_log: false,
                owns_cache: false,
                dbname: dbname.clone(),
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
                versions: core::ptr::null(),
                bg_error: Status::ok(),
                stats: core::array::from_fn(|_| CompactionStats::default()),
                shutting_down: AtomicBool::new(false),
                mem: core::ptr::null_mut(),
                has_imm: AtomicBool::new(false),
                logfile,
                log: core::ptr::null_mut(),
            });

            let versions: *mut VersionSet = Box::into_raw(Box::new(VersionSet::new(
                &dbname,
                leaked_options as *const Options,
                table_cache,
                &db_box.internal_comparator as *const InternalKeyComparator,
            )));

            let mem: *mut MemTable =
                Box::into_raw(Box::new(MemTable::new(&db_box.internal_comparator)));
            let imm: *mut MemTable =
                Box::into_raw(Box::new(MemTable::new(&db_box.internal_comparator)));

            unsafe {
                (*mem).ref_();
                (*imm).ref_();
            }

            db_box.mem = mem;
            db_box.imm = imm;
            db_box.versions = versions as *const VersionSet;

            let db: &'static mut DBImpl = Box::leak(db_box);

            Self { db, versions }
        }

        fn set_last_sequence(&mut self, seq: u64) {
            tracing::debug!(seq, "Setting VersionSet last sequence");
            unsafe {
                (*self.versions).set_last_sequence(seq);
            }
        }

        fn mem_put(&mut self, seq: SequenceNumber, key: &str, value: &str) {
            let k = Slice::from_str(key);
            let v = Slice::from_str(value);
            tracing::debug!(seq, key = %key, value = %value, "MemTable add (TypeValue)");
            unsafe {
                (*self.db.mem).add(seq, ValueType::TypeValue, &k, &v);
            }
        }

        fn imm_put(&mut self, seq: SequenceNumber, key: &str, value: &str) {
            let k = Slice::from_str(key);
            let v = Slice::from_str(value);
            tracing::debug!(seq, key = %key, value = %value, "Imm MemTable add (TypeValue)");
            unsafe {
                (*self.db.imm).add(seq, ValueType::TypeValue, &k, &v);
            }
        }

        fn mem_delete(&mut self, seq: SequenceNumber, key: &str) {
            let k = Slice::from_str(key);
            let empty = Slice::empty();
            tracing::debug!(seq, key = %key, "MemTable add (TypeDeletion)");
            unsafe {
                (*self.db.mem).add(seq, ValueType::TypeDeletion, &k, &empty);
            }
        }
    }

    #[traced_test]
    fn db_get_uses_versions_last_sequence_when_no_explicit_snapshot() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();
        h.mem_put(100, "k", "older");
        h.mem_put(200, "k", "newer");

        h.set_last_sequence(150);

        let mut out = String::new();
        let opts = ReadOptions::default();
        let key = Slice::from_str("k");

        tracing::info!("Calling DBGet::get with default snapshot (should see seq<=150)");
        let s = <DBImpl as DBGet>::get(h.db, &opts, &key, &mut out as *mut String);
        tracing::info!(status = %s.to_string(), value = %out, "DBGet::get returned");

        assert!(s.is_ok(), "expected ok status, got: {}", s.to_string());
        assert_eq!(out, "older", "expected the newest visible value at snapshot=150");

        h.set_last_sequence(250);
        out.clear();

        tracing::info!("Calling DBGet::get with default snapshot (should see seq<=250)");
        let s2 = <DBImpl as DBGet>::get(h.db, &opts, &key, &mut out as *mut String);
        tracing::info!(status = %s2.to_string(), value = %out, "DBGet::get returned");

        assert!(s2.is_ok(), "expected ok status, got: {}", s2.to_string());
        assert_eq!(out, "newer", "expected the newest visible value at snapshot=250");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_prefers_explicit_snapshot_from_read_options_over_versions_last_sequence() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();
        h.mem_put(100, "k", "older");
        h.mem_put(200, "k", "newer");

        h.set_last_sequence(250);

        let snap: Arc<dyn Snapshot> = Arc::new(SnapshotImpl::new(150));
        let mut opts = ReadOptions::default();
        opts.set_snapshot(Some(snap));

        let mut out = String::new();
        let key = Slice::from_str("k");

        tracing::info!("Calling DBGet::get with explicit snapshot=150 (should see seq<=150)");
        let s = <DBImpl as DBGet>::get(h.db, &opts, &key, &mut out as *mut String);
        tracing::info!(status = %s.to_string(), value = %out, "DBGet::get returned");

        assert!(s.is_ok(), "expected ok status, got: {}", s.to_string());
        assert_eq!(out, "older", "explicit snapshot must override VersionSet last sequence");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_checks_immutable_memtable_after_mutable_memtable_miss() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();
        h.mem_put(100, "other", "x");
        h.imm_put(110, "k", "from_imm");

        h.set_last_sequence(200);

        let mut out = String::new();
        let opts = ReadOptions::default();
        let key = Slice::from_str("k");

        tracing::info!("Calling DBGet::get; mem miss, imm hit");
        let s = <DBImpl as DBGet>::get(h.db, &opts, &key, &mut out as *mut String);
        tracing::info!(status = %s.to_string(), value = %out, "DBGet::get returned");

        assert!(s.is_ok(), "expected ok status, got: {}", s.to_string());
        assert_eq!(out, "from_imm", "expected value to be found in immutable memtable");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_mutable_deletion_hides_immutable_value_and_returns_not_found() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        h.imm_put(100, "k", "old");
        h.mem_delete(200, "k");

        h.set_last_sequence(250);

        let mut out = "sentinel".to_string();
        let opts = ReadOptions::default();
        let key = Slice::from_str("k");

        tracing::info!("Calling DBGet::get; mem has deletion, imm has older value");
        let s = <DBImpl as DBGet>::get(h.db, &opts, &key, &mut out as *mut String);
        tracing::info!(status = %s.to_string(), value = %out, "DBGet::get returned");

        assert!(s.is_not_found(), "expected not_found status, got: {}", s.to_string());
        assert!(
            out.is_empty() || out == "sentinel",
            "output buffer should not contain a found value on deletion; got: {out}"
        );

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }
}
