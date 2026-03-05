// ---------------- [ File: bitcoinleveldb-dbtest/src/dbtest.rs ]
crate::ix!();

/// Invariant: the discriminant order is relied upon by `DBTest::change_options`,
/// which advances through configurations by integer increment.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBTestOptionConfig {
    /// Invariant: baseline configuration.
    Default      = 0,
    /// Invariant: `reuse_logs` enabled.
    Reuse        = 1,
    /// Invariant: Bloom filter policy enabled.
    Filter       = 2,
    /// Invariant: compression disabled.
    Uncompressed = 3,
    /// Invariant: sentinel; not a configuration to run.
    End          = 4,
}

/// Invariant: owns the on-disk database directory at `dbname` for the lifetime of the value.
/// Invariant: `env` points to a heap-allocated `SpecialEnv` until `Drop`.
/// Invariant: `db` is either null or points to a heap-allocated DB instance returned by Open.
pub struct DBTest {
    /// Invariant: filesystem path of the database directory used by this test harness.
    dbname: String,

    /// Invariant: non-null after construction; freed exactly once in `Drop`.
    env: *mut SpecialEnv,

    /// Invariant: null when closed; otherwise points to an open DB instance.
    db: *mut dyn DB,

    /// Invariant: exact options used for the most recent successful (or attempted) open.
    last_options: Options,

    /// Invariant: shared filter policy instance used for the filter configuration.
    filter_policy: Arc<dyn FilterPolicy>,

    /// Invariant: integer index corresponding to `DBTestOptionConfig` discriminants.
    option_config: i32,
}

impl Default for DBTest {
    fn default() -> Self {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.default.enter",
            phase = "enter"
        );

        // NOTE: translated from: env_(new SpecialEnv(Env::Default()))
        let base_env = Env::default(); // defined elsewhere in codebase
        let env_box = Box::new(SpecialEnv::new(base_env));
        let env_ptr: *mut SpecialEnv = Box::into_raw(env_box);

        // NOTE: translated from: filter_policy_ = NewBloomFilterPolicy(10);
        let filter_policy: Arc<dyn FilterPolicy> = Arc::from(new_bloom_filter_policy(10));

        // NOTE: translated from: dbname_ = bitcoinleveldb_test::TmpDir() + "/db_test";
        let mut dbname = bitcoinleveldb_test::tmp_dir();
        dbname.push_str("/db_test");

        // NOTE: translated from: DestroyDB(dbname_, Options());
        // Ignore status as in the C++ original.
        let _ = destroy_db(&dbname, &Options::default()); // defined elsewhere in codebase

        let mut this = Self {
            dbname,
            env: env_ptr,
            db: null_mut::<dyn DB>(),
            last_options: Options::default(),
            filter_policy,
            option_config: DBTestOptionConfig::Default as i32,
        };

        // NOTE: translated from: db_ = nullptr; Reopen();
        this.reopen(None);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.default.exit",
            phase = "exit"
        );

        this
    }
}

impl Drop for DBTest {
    fn drop(&mut self) {
        tracing::debug!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.drop.enter",
            phase = "enter",
            dbname = self.dbname.as_str()
        );

        // NOTE: translated from: delete db_;
        unsafe {
            if !self.db.is_null() {
                drop(Box::from_raw(self.db));
                self.db = null_mut::<dyn DB>();
            }
        }

        // NOTE: translated from: DestroyDB(dbname_, Options());
        // Ignore status as in the C++ original.
        let _ = destroy_db(&self.dbname, &Options::default()); // defined elsewhere in codebase

        // NOTE: translated from: delete env_;
        unsafe {
            if !self.env.is_null() {
                drop(Box::from_raw(self.env));
                self.env = null_mut::<SpecialEnv>();
            }
        }

        tracing::debug!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.drop.exit",
            phase = "exit",
            dbname = self.dbname.as_str()
        );
    }
}

impl DBTest {

    /// Switch to a fresh database with the next option configuration to test. 
    ///
    /// Return false if there are no more configurations to test.
    /// 
    /// Precondition: none.
    ///
    /// Postcondition: advances the configuration index by exactly one; if within range,
    /// transitions the on-disk DB to a fresh instance opened under the new configuration.
    ///
    pub fn change_options(&mut self) -> bool {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.change_options.enter",
            phase = "enter",
            option_config = self.option_config
        );

        // translated from: option_config_++;
        self.option_config += 1;

        // translated from:
        // if (option_config_ >= kEnd) { return false; } else { DestroyAndReopen(); return true; }
        let ok = if self.option_config >= (DBTestOptionConfig::End as i32) {
            false
        } else {
            self.destroy_and_reopen(None);
            true
        };

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.change_options.exit",
            phase = "exit",
            option_config = self.option_config,
            ok
        );

        ok
    }

    /// Return the current option configuration.
    ///
    /// Precondition: `self.option_config` corresponds to a `DBTestOptionConfig` discriminant.
    /// Postcondition: returns an `Options` value whose fields reflect the current configuration.
    pub fn current_options(&mut self) -> Options {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.current_options.enter",
            phase = "enter",
            option_config = self.option_config
        );

        // translated from: Options options; options.reuse_logs = false;
        let mut options = Options::default();
        options.set_reuse_logs(false);

        // translated from: switch(option_config_) { ... }
        if self.option_config == (DBTestOptionConfig::Reuse as i32) {
            options.set_reuse_logs(true);
        } else if self.option_config == (DBTestOptionConfig::Filter as i32) {
            options.set_filter_policy(Arc::clone(&self.filter_policy));
        } else if self.option_config == (DBTestOptionConfig::Uncompressed as i32) {
            options.set_compression(CompressionType::None);
        } else {
            // default: break;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.current_options.exit",
            phase = "exit",
            option_config = self.option_config
        );

        options
    }
    
    /// Precondition: `self.db` is either null or points to a `DBImpl` instance.
    /// Postcondition: returns the raw pointer view of the underlying implementation.
    pub fn dbfull(&mut self) -> *mut DBImpl {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.dbfull",
            phase = "return",
            db_is_null = self.db.is_null()
        );

        self.db as *mut DBImpl
    }
    
    /// Precondition: none.
    /// Postcondition: `self.db` is an open DB (asserted), opened with either the provided options
    /// or the current configuration options.
    pub fn reopen(&mut self, mut options: Option<&mut Options>) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.reopen.enter",
            phase = "enter",
            has_options = options.is_some()
        );

        let options_ptr: *mut Options = match options.as_mut() {
            Some(o) => (*o) as *mut Options,
            None => null_mut::<Options>(),
        };

        // translated from: ASSERT_OK(TryReopen(options));
        let s = self.try_reopen(options_ptr);
        assert_ok!(&s);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.reopen.exit",
            phase = "exit",
            ok = s.is_ok()
        );
    }

    /// Precondition: none.
    /// Postcondition: `self.db` is null.
    pub fn close(&mut self) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.close.enter",
            phase = "enter",
            db_is_null = self.db.is_null()
        );

        // translated from: delete db_; db_ = nullptr;
        unsafe {
            if !self.db.is_null() {
                drop(Box::from_raw(self.db));
                self.db = null_mut::<dyn DB>();
            }
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.close.exit",
            phase = "exit",
            db_is_null = self.db.is_null()
        );
    }
   
    /// Precondition: none.
    /// Postcondition: on-disk database directory is destroyed and a fresh DB is opened (asserted).
    pub fn destroy_and_reopen(&mut self, mut options: Option<&mut Options>) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.destroy_and_reopen.enter",
            phase = "enter",
            has_options = options.is_some(),
            dbname = self.dbname.as_str()
        );

        // translated from: delete db_; db_=nullptr; DestroyDB(...); ASSERT_OK(TryReopen(...));
        unsafe {
            if !self.db.is_null() {
                drop(Box::from_raw(self.db));
                self.db = null_mut::<dyn DB>();
            }
        }

        let _ = destroy_db(&self.dbname, &Options::default()); // defined elsewhere in codebase

        let options_ptr: *mut Options = match options.as_mut() {
            Some(o) => (*o) as *mut Options,
            None => null_mut::<Options>(),
        };

        let s = self.try_reopen(options_ptr);
        assert_ok!(&s);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.destroy_and_reopen.exit",
            phase = "exit",
            ok = s.is_ok(),
            db_is_null = self.db.is_null()
        );
    }
    
    /// Precondition: `options` is either null or points to an initialized `Options`.
    /// Postcondition: `self.db` is replaced with the newly opened DB pointer (or null on failure),
    /// and `self.last_options` is set to the attempted options.
    pub fn try_reopen(&mut self, options: *mut Options) -> crate::Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.try_reopen.enter",
            phase = "enter",
            options_is_null = options.is_null(),
            dbname = self.dbname.as_str()
        );

        // translated from: delete db_; db_ = nullptr;
        unsafe {
            if !self.db.is_null() {
                drop(Box::from_raw(self.db));
                self.db = null_mut::<dyn DB>();
            }
        }

        // translated from:
        // Options opts;
        // if (options != nullptr) { opts = *options; }
        // else { opts = CurrentOptions(); opts.create_if_missing = true; }
        let mut opts: Options = if !options.is_null() {
            unsafe { (*options).clone() }
        } else {
            let mut o = self.current_options();
            o.set_create_if_missing(true);
            o
        };

        // translated from: last_options_ = opts;
        self.last_options = opts.clone();

        // translated from: return DB::Open(opts, dbname_, &db_);
        // NOTE: implemented via the `DBOpen` surface; `DBImpl::open` performs the actual open.
        let mut opener = DBImpl::new(&opts, &self.dbname);
        let s = opener.open(&opts, &self.dbname, (&mut self.db) as *mut *mut dyn DB);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.try_reopen.exit",
            phase = "exit",
            ok = s.is_ok(),
            db_is_null = self.db.is_null()
        );

        s
    }

    /// Precondition: `self.db` is non-null.
    /// Postcondition: forwards to DB `Put` and returns its `Status`.
    pub fn put(&mut self, k: &String, v: &String) -> crate::Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.put.enter",
            phase = "enter",
            db_is_null = self.db.is_null(),
            key_len = k.len(),
            val_len = v.len()
        );

        let w = WriteOptions::default();
        let ks = Slice::from(k);
        let vs = Slice::from(v);

        let s = unsafe { (*self.dbfull()).put(&w, &ks, &vs) };

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.put.exit",
            phase = "exit",
            ok = s.is_ok()
        );

        s
    }

    /// Precondition: `self.db` is non-null.
    /// Postcondition: forwards to DB `Delete` and returns its `Status`.
    pub fn delete(&mut self, k: &String) -> crate::Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.delete.enter",
            phase = "enter",
            db_is_null = self.db.is_null(),
            key_len = k.len()
        );

        let w = WriteOptions::default();
        let ks = Slice::from(k);

        let s = unsafe { (*self.dbfull()).delete(&w, &ks) };

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.delete.exit",
            phase = "exit",
            ok = s.is_ok()
        );

        s
    }

    /// Precondition: `self.db` is non-null.
    /// Postcondition: returns `"NOT_FOUND"` iff the DB reports NotFound; otherwise returns either
    /// the retrieved value or the error string from the underlying `Status`.
    pub fn get(&mut self, k: &String, snapshot: Option<&dyn Snapshot>) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.get.enter",
            phase = "enter",
            db_is_null = self.db.is_null(),
            has_snapshot = snapshot.is_some(),
            key_len = k.len()
        );

        // translated from:
        // ReadOptions options; options.snapshot = snapshot;
        let mut options = ReadOptions::default();

        // NOTE: snapshot plumbing: `ReadOptions::snapshot` is carried as `Option<Arc<dyn Snapshot>>`
        // in this codebase. The C++ surface passes `const Snapshot*`; the Rust surface passes
        // `&dyn Snapshot` where the concrete value is an `Arc<dyn Snapshot>` handle.
        let snap_arc: Option<Arc<dyn Snapshot>> = match snapshot {
            Some(s) => {
                let arc_ref: &Arc<dyn Snapshot> =
                    unsafe { &*(s as *const dyn Snapshot as *const Arc<dyn Snapshot>) };
                Some(Arc::clone(arc_ref))
            }
            None => None,
        };
        options.set_snapshot(snap_arc);

        let mut result = String::new();
        let ks = Slice::from(k);

        // translated from: Status s = db_->Get(options, k, &result);
        let s = unsafe { (*self.dbfull()).get(&options, &ks, (&mut result) as *mut String) };

        // translated from:
        // if (s.IsNotFound()) result="NOT_FOUND";
        // else if (!s.ok()) result=s.ToString();
        if s.is_not_found() {
            result = "NOT_FOUND".to_string();
        } else if !s.is_ok() {
            result = s.to_string();
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.get.exit",
            phase = "exit",
            status_ok = s.is_ok(),
            status_not_found = s.is_not_found(),
            out_len = result.len()
        );

        result
    }

    /// Return a string that contains all key,value
    /// pairs in order, formatted like "(k1->v1)(k2->v2)".
    /// 
    pub fn contents(&mut self) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.contents.enter",
            phase = "enter",
            db_is_null = self.db.is_null()
        );

        // translated from: std::vector<std::string> forward;
        let mut forward: Vec<String> = Vec::new();

        // translated from: std::string result;
        let mut result = String::new();

        // translated from: Iterator* iter = db_->NewIterator(ReadOptions());
        let iter_ptr = unsafe { (*self.dbfull()).new_iterator(&ReadOptions::default()) };

        // translated from: for (iter->SeekToFirst(); iter->Valid(); iter->Next()) { ... }
        unsafe {
            let iter = &mut *iter_ptr;
            iter.seek_to_first();
            while iter.valid() {
                let s = self.iter_status(iter_ptr);
                result.push('(');
                result.push_str(&s);
                result.push(')');
                forward.push(s);
                iter.next();
            }

            // translated from reverse iteration check
            let mut matched: usize = 0;
            iter.seek_to_last();
            while iter.valid() {
                assert_lt!(matched, forward.len());
                assert_eq!(
                    self.iter_status(iter_ptr),
                    forward[forward.len() - matched - 1]
                );
                matched += 1;
                iter.prev();
            }
            assert_eq!(matched, forward.len());
        }

        // translated from: delete iter;
        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.contents.exit",
            phase = "exit",
            out_len = result.len()
        );

        result
    }
   
    pub fn all_entries_for(&mut self, user_key_: &Slice) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.all_entries_for.enter",
            phase = "enter",
            db_is_null = self.db.is_null(),
            user_key_len = user_key_.size()
        );

        // translated from: Iterator* iter = dbfull()->TEST_NewInternalIterator();
        let iter_ptr = unsafe { (*self.dbfull()).test_new_internal_iterator() };

        // translated from: InternalKey target(user_key, kMaxSequenceNumber, kTypeValue);
        //                  iter->Seek(target.Encode());
        let target = InternalKey::new(user_key_.clone(), k_max_sequence_number(), ValueType::Value);
        unsafe { (&mut *iter_ptr).seek(&target.encode()) };

        let mut result = String::new();

        // translated from status check
        let status = unsafe { (&*iter_ptr).status() };
        if !status.is_ok() {
            result = status.to_string();
        } else {
            result.push_str("[ ");
            let mut first = true;

            unsafe {
                let iter = &mut *iter_ptr;
                while iter.valid() {
                    let mut ikey = ParsedInternalKey::default();
                    if !parse_internal_key(&iter.key(), &mut ikey) {
                        result.push_str("CORRUPTED");
                    } else {
                        // translated from comparator compare/break
                        if self
                            .last_options
                            .comparator()
                            .compare(ikey.user_key(), user_key_)
                            != 0
                        {
                            break;
                        }

                        if !first {
                            result.push_str(", ");
                        }
                        first = false;

                        match ikey.value_type() {
                            ValueType::Value => {
                                result.push_str(&iter.value().to_string());
                            }
                            ValueType::Deletion => {
                                result.push_str("DEL");
                            }
                        }
                    }
                    iter.next();
                }
            }

            if !first {
                result.push_str(" ");
            }
            result.push(']');
        }

        // translated from: delete iter;
        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.all_entries_for.exit",
            phase = "exit",
            out_len = result.len()
        );

        result
    }

    pub fn num_table_files_at_level(&mut self, level: i32) -> i32 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.num_table_files_at_level.enter",
            phase = "enter",
            level
        );

        // translated from:
        // std::string property;
        // ASSERT_TRUE(db_->GetProperty("leveldb.num-files-at-level" + NumberToString(level), &property));
        // return std::stoi(property);
        let mut property = String::new();
        let mut key = "leveldb.num-files-at-level".to_string();
        key.push_str(&level.to_string());

        let ok = unsafe { (*self.dbfull()).get_property(&key, (&mut property) as *mut String) };
        assert_true!(ok);

        let parsed = match property.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                assert_true!(false);
                0
            }
        };

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.num_table_files_at_level.exit",
            phase = "exit",
            level,
            parsed
        );

        parsed
    }
    
    pub fn total_table_files(&mut self) -> i32 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.total_table_files.enter",
            phase = "enter"
        );

        // translated from:
        // int result = 0;
        // for (int level = 0; level < config::kNumLevels; level++) { result += NumTableFilesAtLevel(level); }
        let mut result: i32 = 0;
        let mut level: i32 = 0;
        while level < (config::k_num_levels() as i32) {
            result += self.num_table_files_at_level(level);
            level += 1;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.total_table_files.exit",
            phase = "exit",
            result
        );

        result
    }

    /**
      | Return spread of files per level
      |
      */
    pub fn files_per_level(&mut self) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.files_per_level.enter",
            phase = "enter"
        );

        // translated from:
        // std::string result;
        // int last_non_zero_offset = 0;
        // for (int level = 0; level < config::kNumLevels; level++) { ... }
        let mut result = String::new();
        let mut last_non_zero_offset: usize = 0;

        let mut level: i32 = 0;
        while level < (config::k_num_levels() as i32) {
            let f = self.num_table_files_at_level(level);
            let mut buf = String::new();
            if level != 0 {
                buf.push(',');
            }
            buf.push_str(&f.to_string());
            result.push_str(&buf);
            if f > 0 {
                last_non_zero_offset = result.len();
            }
            level += 1;
        }

        // translated from: result.resize(last_non_zero_offset);
        result.truncate(last_non_zero_offset);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.files_per_level.exit",
            phase = "exit",
            out_len = result.len()
        );

        result
    }
   
    pub fn count_files(&mut self) -> i32 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.count_files.enter",
            phase = "enter",
            dbname = self.dbname.as_str()
        );

        // translated from:
        // std::vector<std::string> files;
        // env_->GetChildren(dbname_, &files);
        // return static_cast<int>(files.size());
        let mut files: Vec<String> = Vec::new();
        let s = unsafe { (*self.env).get_children(&self.dbname, &mut files) }; // defined elsewhere in codebase
        assert_ok!(&s);

        let n = files.len() as i32;

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.count_files.exit",
            phase = "exit",
            n
        );

        n
    }

    pub fn size(&mut self, start: &Slice, limit: &Slice) -> u64 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.size.enter",
            phase = "enter"
        );

        // translated from:
        // Range r(start, limit);
        // uint64_t size;
        // db_->GetApproximateSizes(&r, 1, &size);
        // return size;
        let r = Range::new(start.clone(), limit.clone());
        let mut size: u64 = 0;
        unsafe {
            (*self.dbfull()).get_approximate_sizes((&r) as *const Range, 1, (&mut size) as *mut u64);
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.size.exit",
            phase = "exit",
            size
        );

        size
    }
   
    pub fn compact(&mut self, start: &Slice, limit: &Slice) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.compact.enter",
            phase = "enter"
        );

        // translated from: db_->CompactRange(&start, &limit);
        unsafe {
            (*self.dbfull()).compact_range((start as *const Slice), (limit as *const Slice));
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.compact.exit",
            phase = "exit"
        );
    }

    /**
      | Do n memtable compactions, each of which
      | produces an sstable covering the range
      | [small_key,large_key].
      |
      */
    pub fn make_tables(&mut self, n: i32, small_key_: &String, large_key_: &String) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.make_tables.enter",
            phase = "enter",
            n
        );

        // translated from:
        // for (int i = 0; i < n; i++) {
        //   Put(small_key, "begin");
        //   Put(large_key, "end");
        //   dbfull()->TEST_CompactMemTable();
        // }
        let mut i: i32 = 0;
        while i < n {
            let begin = "begin".to_string();
            let end = "end".to_string();
            let _ = self.put(small_key_, &begin);
            let _ = self.put(large_key_, &end);
            let _ = unsafe { (*self.dbfull()).test_compact_mem_table() };
            i += 1;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.make_tables.exit",
            phase = "exit"
        );
    }

    /**
      | Prevent pushing of new sstables into
      | deeper levels by adding tables that
      | cover a specified range to all levels.
      |
      */
    pub fn fill_levels(&mut self, smallest: &String, largest: &String) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.fill_levels.enter",
            phase = "enter"
        );

        // translated from: MakeTables(config::kNumLevels, smallest, largest);
        self.make_tables(config::k_num_levels() as i32, smallest, largest);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.fill_levels.exit",
            phase = "exit"
        );
    }

    pub fn dump_file_counts(&mut self, label: *const u8) {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.dump_file_counts.enter",
            phase = "enter"
        );

        // translated from:
        // fprintf(stderr, "---\n%s:\n", label);
        // fprintf(stderr, "maxoverlap: %lld\n", ...);
        // for ... print per-level file counts
        unsafe {
            eprintln!("---");
            eprintln!("{}:", ptr_to_string!(label));
        }

        let max_overlap = unsafe { (*self.dbfull()).test_max_next_level_overlapping_bytes() };
        eprintln!("maxoverlap: {}", max_overlap);

        let mut level: i32 = 0;
        while level < (config::k_num_levels() as i32) {
            let num = self.num_table_files_at_level(level);
            if num > 0 {
                eprintln!("  level {:3} : {} files", level, num);
            }
            level += 1;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.dump_file_counts.exit",
            phase = "exit"
        );
    }
 
    pub fn dump_ss_table_list(&mut self) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.dump_ss_table_list.enter",
            phase = "enter"
        );

        // translated from:
        // std::string property;
        // db_->GetProperty("leveldb.sstables", &property);
        // return property;
        let mut property = String::new();
        let _ = unsafe {
            (*self.dbfull()).get_property("leveldb.sstables", (&mut property) as *mut String)
        };

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.dump_ss_table_list.exit",
            phase = "exit",
            out_len = property.len()
        );

        property
    }
 
    pub fn iter_status(&mut self, iter: *mut LevelDBIterator) -> String {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.iter_status.enter",
            phase = "enter",
            iter_is_null = iter.is_null()
        );

        // translated from:
        // std::string result;
        // if (iter->Valid()) { result = key->ToString() + "->" + value->ToString(); }
        // else { result = "(invalid)"; }
        let mut result = String::new();

        unsafe {
            if (&*iter).valid() {
                let k = (&*iter).key().to_string();
                let v = (&*iter).value().to_string();
                result.push_str(&k);
                result.push_str("->");
                result.push_str(&v);
            } else {
                result.push_str("(invalid)");
            }
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.iter_status.exit",
            phase = "exit",
            out_len = result.len()
        );

        result
    }
 
    pub fn delete_an_sst_file(&mut self) -> bool {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.delete_an_sst_file.enter",
            phase = "enter",
            dbname = self.dbname.as_str()
        );

        // translated from:
        // std::vector<std::string> filenames;
        // ASSERT_OK(env_->GetChildren(dbname_, &filenames));
        // uint64_t number; FileType type;
        // for (...) { if(ParseFileName(...) && type==kTableFile) { ASSERT_OK(DeleteFile(TableFileName(...))); return true; } }
        // return false;
        let mut filenames: Vec<String> = Vec::new();
        let s = unsafe { (*self.env).get_children(&self.dbname, &mut filenames) }; // defined elsewhere
        assert_ok!(&s);

        let mut i: usize = 0;
        while i < filenames.len() {
            let mut number: u64 = 0;
            let mut file_type = FileType::default();
            if parse_file_name(&filenames[i], &mut number, &mut file_type) && file_type == FileType::TableFile
            {
                let fname = table_file_name(&self.dbname, number);
                let ds = unsafe { (*self.env).delete_file(&fname) }; // defined elsewhere
                assert_ok!(&ds);

                tracing::trace!(
                    target: "bitcoinleveldb_dbtest::dbtest",
                    label = "dbtest.delete_an_sst_file.exit",
                    phase = "exit",
                    deleted = true,
                    number
                );

                return true;
            }
            i += 1;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.delete_an_sst_file.exit",
            phase = "exit",
            deleted = false
        );

        false
    }

    /**
      | Returns number of files renamed.
      |
      */
    pub fn rename_ldb_tosst(&mut self) -> i32 {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.rename_ldb_tosst.enter",
            phase = "enter",
            dbname = self.dbname.as_str()
        );

        // translated from:
        // std::vector<std::string> filenames;
        // ASSERT_OK(env_->GetChildren(dbname_, &filenames));
        // uint64_t number; FileType type;
        // int files_renamed = 0;
        // for (...) { if(ParseFileName(...) && type==kTableFile) { from=TableFileName(...); to=SSTTableFileName(...); ASSERT_OK(RenameFile(from,to)); files_renamed++; } }
        // return files_renamed;
        let mut filenames: Vec<String> = Vec::new();
        let s = unsafe { (*self.env).get_children(&self.dbname, &mut filenames) }; // defined elsewhere
        assert_ok!(&s);

        let mut files_renamed: i32 = 0;
        let mut i: usize = 0;
        while i < filenames.len() {
            let mut number: u64 = 0;
            let mut file_type = FileType::default();
            if parse_file_name(&filenames[i], &mut number, &mut file_type) && file_type == FileType::TableFile
            {
                let from = table_file_name(&self.dbname, number);
                let to = sst_table_file_name(&self.dbname, number);
                let rs = unsafe { (*self.env).rename_file(&from, &to) }; // defined elsewhere
                assert_ok!(&rs);
                files_renamed += 1;
            }
            i += 1;
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.rename_ldb_tosst.exit",
            phase = "exit",
            files_renamed
        );

        files_renamed
    }
}
