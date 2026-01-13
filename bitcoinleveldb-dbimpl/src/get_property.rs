// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_property.rs ]
crate::ix!();

impl DBGetProperty for DBImpl {
    fn get_property(&mut self, property: &str, value: *mut String) -> bool {
        unsafe {
            (*value).clear();
        }

        self.mutex.lock();

        let mut input: Slice = Slice::from_str(property);
        let prefix: Slice = Slice::from_str("leveldb.");

        if !input.starts_with(&prefix) {
            unsafe { self.mutex.unlock() };
            return false;
        }

        let prefix_len: usize = *prefix.size();
        input.remove_prefix(prefix_len);

        let num_files_prefix: Slice = Slice::from_str("num-files-at-level");
        let stats_key:        Slice = Slice::from_str("stats");
        let sstables_key:     Slice = Slice::from_str("sstables");
        let approx_mem_key:   Slice = Slice::from_str("approximate-memory-usage");

        if input.starts_with(&num_files_prefix) {
            let nfp_len: usize = *num_files_prefix.size();
            input.remove_prefix(nfp_len);

            let mut level: u64 = 0;
            let ok: bool = consume_decimal_number(&mut input, &mut level) && input.empty();

            if !ok || level >= (NUM_LEVELS as u64) {
                unsafe { self.mutex.unlock() };
                return false;
            }

            let files: i32 = unsafe { (*self.versions).num_level_files(level as i32) };
            unsafe {
                (*value) = files.to_string();
            }

            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&stats_key) == 0 {
            unsafe {
                (*value).push_str(
                    "                               Compactions\n\
                     Level  Files Size(MB) Time(sec) Read(MB) Write(MB)\n\
                     --------------------------------------------------\n",
                );
            }

            for level in 0..NUM_LEVELS {
                let level_i32: i32 = level as i32;
                let files: i32 = unsafe { (*self.versions).num_level_files(level_i32) };

                if *self.stats[level].micros() > 0 || files > 0 {
                    let line = format!(
                        "{:3} {:8} {:8.0} {:9.0} {:8.0} {:9.0}\n",
                        level,
                        files,
                        unsafe { (*self.versions).num_level_bytes(level_i32) } as f64 / 1048576.0,
                        *self.stats[level].micros() as f64 / 1e6,
                        *self.stats[level].bytes_read() as f64 / 1048576.0,
                        *self.stats[level].bytes_written() as f64 / 1048576.0
                    );
                    unsafe {
                        (*value).push_str(&line);
                    }
                }
            }

            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&sstables_key) == 0 {
            let dbg = unsafe { (*(*self.versions).current()).debug_string() };
            unsafe {
                (*value) = dbg;
            }
            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&approx_mem_key) == 0 {
            let mut total_usage: usize = (*self.options.block_cache()).total_charge();

            if !self.mem.is_null() {
                total_usage += unsafe { (*self.mem).approximate_memory_usage() };
            }

            if !self.imm.is_null() {
                total_usage += unsafe { (*self.imm).approximate_memory_usage() };
            }

            unsafe {
                (*value).push_str(&format!("{}", total_usage as u64));
            }

            unsafe { self.mutex.unlock() };
            return true;
        }

        unsafe { self.mutex.unlock() };
        false
    }
}

#[cfg(test)]
#[disable]
mod db_get_property_contract_suite {
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
        fn build_with_mem_and_imm() -> Self {
            let env_rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(NoopEnv::default()));
            let mut opts: Options = Options::with_env(env_rc.clone());
            opts.set_compression(CompressionType::None);

            let leaked_options: &'static mut Options = Box::leak(Box::new(opts));
            let dbname: String = "db_get_property_contract_suite".to_string();

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
            let imm: *mut MemTable =
                Box::into_raw(Box::new(MemTable::new(&internal_comparator)));

            unsafe {
                (*mem).ref_();
                (*imm).ref_();
                (*versions).set_last_sequence(250);
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
                imm,
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
    }

    #[traced_test]
    fn db_get_property_rejects_non_leveldb_prefix_and_clears_value() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out = "not empty".to_string();
        let ok = <DBImpl as DBGetProperty>::get_property(h.db, "rocksdb.stats", &mut out as *mut String);

        tracing::info!(ok, value = %out, "DBGetProperty::get_property returned");

        assert!(!ok, "non leveldb.* properties must return false");
        assert!(out.is_empty(), "value must be cleared for non leveldb.* properties");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_num_files_at_level_parses_and_returns_value() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(
            h.db,
            "leveldb.num-files-at-level0",
            &mut out as *mut String,
        );

        tracing::info!(ok, value = %out, "num-files-at-level0");

        assert!(ok, "expected true for leveldb.num-files-at-level0");
        assert_eq!(out, "0", "fresh VersionSet should have 0 files at level 0");

        let mut out2 = String::new();
        let ok2 = <DBImpl as DBGetProperty>::get_property(
            h.db,
            &format!("leveldb.num-files-at-level{}", NUM_LEVELS - 1),
            &mut out2 as *mut String,
        );

        tracing::info!(ok = ok2, value = %out2, "num-files-at-level(last)");

        assert!(ok2, "expected true for last valid level");
        assert_eq!(out2, "0", "fresh VersionSet should have 0 files at last level");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_num_files_at_level_rejects_out_of_range_and_malformed_inputs() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out = "sentinel".to_string();
        let ok_missing_digits = <DBImpl as DBGetProperty>::get_property(
            h.db,
            "leveldb.num-files-at-level",
            &mut out as *mut String,
        );
        tracing::info!(ok = ok_missing_digits, value = %out, "missing digits");
        assert!(!ok_missing_digits, "missing digits must be rejected");
        assert!(out.is_empty(), "value must be cleared even when rejected");

        let mut out2 = String::new();
        let ok_trailing = <DBImpl as DBGetProperty>::get_property(
            h.db,
            "leveldb.num-files-at-level0x",
            &mut out2 as *mut String,
        );
        tracing::info!(ok = ok_trailing, value = %out2, "trailing characters");
        assert!(!ok_trailing, "trailing characters must be rejected");

        let mut out3 = String::new();
        let ok_negative = <DBImpl as DBGetProperty>::get_property(
            h.db,
            "leveldb.num-files-at-level-1",
            &mut out3 as *mut String,
        );
        tracing::info!(ok = ok_negative, value = %out3, "negative level");
        assert!(!ok_negative, "negative levels must be rejected");

        let mut out4 = String::new();
        let ok_oob = <DBImpl as DBGetProperty>::get_property(
            h.db,
            &format!("leveldb.num-files-at-level{}", NUM_LEVELS),
            &mut out4 as *mut String,
        );
        tracing::info!(ok = ok_oob, value = %out4, "out of bounds level");
        assert!(!ok_oob, "levels >= NUM_LEVELS must be rejected");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_stats_outputs_header_and_includes_nonzero_stats_levels() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out_empty = String::new();
        let ok_empty = <DBImpl as DBGetProperty>::get_property(h.db, "leveldb.stats", &mut out_empty as *mut String);
        tracing::info!(ok = ok_empty, len = out_empty.len(), "stats (all zero) output");
        assert!(ok_empty, "stats must return true");
        assert!(out_empty.contains("Compactions"), "stats output must include header");
        assert!(
            !out_empty.contains("\n  0 "),
            "with all-zero stats and no files, level lines should not appear"
        );

        h.db.stats[2].set_micros(5_000_000);
        h.db.stats[2].set_bytes_read(64 * 1024 * 1024);
        h.db.stats[2].set_bytes_written(32 * 1024 * 1024);

        let mut out = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(h.db, "leveldb.stats", &mut out as *mut String);
        tracing::info!(ok, "stats output:\n{}", out);

        assert!(ok, "stats must return true");
        assert!(out.contains("Compactions"), "stats output must include header");
        assert!(
            out.contains("\n  2 ") || out.contains("\n  2"),
            "stats output must include an entry for level 2 when micros>0"
        );

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_sstables_returns_true() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(h.db, "leveldb.sstables", &mut out as *mut String);

        tracing::info!(ok, len = out.len(), "sstables output");

        assert!(ok, "sstables must return true");
        assert!(!out.is_empty(), "sstables output should not be empty for a valid VersionSet");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_approximate_memory_usage_matches_memtable_components_when_cache_is_null() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        h.mem_put(1, "k1", "v1");
        h.mem_put(2, "k2", "v2");
        h.imm_put(3, "k3", "v3");

        let mem_usage: usize = unsafe { (*h.db.mem).approximate_memory_usage() };
        let imm_usage: usize = unsafe { (*h.db.imm).approximate_memory_usage() };
        let expected_total: u64 = (mem_usage + imm_usage) as u64;

        tracing::debug!(
            mem_usage,
            imm_usage,
            expected_total,
            "Expected approximate-memory-usage components"
        );

        let mut out = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(
            h.db,
            "leveldb.approximate-memory-usage",
            &mut out as *mut String,
        );

        tracing::info!(ok, value = %out, "approximate-memory-usage");

        assert!(ok, "approximate-memory-usage must return true");

        let parsed: u64 = out.parse().unwrap_or(0);
        assert_eq!(
            parsed, expected_total,
            "approximate-memory-usage must equal cache_charge(0) + mem + imm"
        );

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }

    #[traced_test]
    fn db_get_property_unknown_leveldb_property_returns_false() {
        let mut h = LeakedDbImplHarness::build_with_mem_and_imm();

        let mut out = "sentinel".to_string();
        let ok = <DBImpl as DBGetProperty>::get_property(h.db, "leveldb.unknown-property", &mut out as *mut String);

        tracing::info!(ok, value = %out, "unknown property");

        assert!(!ok, "unknown leveldb.* property must return false");
        assert!(out.is_empty(), "value must be cleared on entry");

        h.db.mutex.lock();
        h.db.mutex.unlock();
    }
}
