// ---------------- [ File: bitcoinleveldb-dbimpl/src/create.rs ]
crate::ix!();

impl DBImpl {
    
    /// Extra methods (for testing) that are
    /// not in the public DB interface
    /// 
    pub fn user_comparator(&self) -> Box<dyn SliceComparator> {
        Box::new(DbImplUserComparatorAdapter::new(
                self.internal_comparator.user_comparator(),
        ))
    }

    pub fn new(raw_options: &Options, dbname: &String) -> Self {
        let internal_comparator = InternalKeyComparator::new(raw_options.comparator().as_ref());
        let internal_filter_policy =
            InternalFilterPolicy::new(raw_options.filter_policy().as_ref());

        let options = sanitize_options(
            dbname,
            &internal_comparator,
            &internal_filter_policy,
            raw_options,
        );

        let env_rc: Rc<RefCell<dyn Env>> = match options.env().as_ref() {
            Some(e) => e.clone(),
            None => match raw_options.env().as_ref() {
                Some(e) => e.clone(),
                None => {
                    tracing::error!("Options.env was None in DBImpl::new");
                    panic!();
                }
            },
        };

        let env: Box<dyn Env> = Box::new(EnvWrapper::new(env_rc));

        let owns_info_log: bool = options.info_log() != raw_options.info_log();
        let owns_cache: bool = options.block_cache() != raw_options.block_cache();

        let dbname: String = dbname.clone();

        // IMPORTANT:
        // VersionSet stores a raw pointer to Options. DBImpl moves by value in tests (and generally),
        // so taking &options (stack/local) would dangle. Instead, derive a stable Options pointer
        // from TableCache's Rc<Options> allocation.
        let table_cache_box: Box<TableCache> = Box::new(TableCache::new(
                &dbname,
                &options,
                table_cache_size(&options),
        ));

        let stable_options_ptr: *const Options = table_cache_box.options_ref() as *const Options;

        tracing::debug!(
            dbname = %dbname,
            options_stack_ptr = core::ptr::addr_of!(options) as usize,
            stable_options_ptr = stable_options_ptr as usize,
            "DBImpl::new: derived stable Options pointer for VersionSet from TableCache"
        );

        let table_cache: *const TableCache = Box::into_raw(table_cache_box) as *const TableCache;

        let mutex: RawMutex = RawMutex::INIT;

        // Dedicated Condvar wait mutex (see DBImpl.background_work_finished_mutex docs).
        let background_work_finished_mutex: Mutex<()> = Mutex::new(());
        let background_work_finished_signal: Condvar = Condvar::new();

        let versions_box: Box<VersionSet> = Box::new(VersionSet::new(
                &dbname,
                stable_options_ptr,
                table_cache as *mut TableCache,
                &internal_comparator,
        ));
        let versions: *mut VersionSet = Box::into_raw(versions_box);

        // Non-null iff successfully acquired (DBImpl::recover / DBOpen path).
        let db_lock: *mut Box<dyn FileLock> = core::ptr::null_mut();

        // Placeholder handle; real logfile is established during Open()/recovery paths.
        let logfile: Rc<RefCell<dyn WritableFile>> =
            Rc::new(RefCell::new(StdoutPrinter {}));

        Self {
            env,
            internal_comparator,
            internal_filter_policy,
            options,
            owns_info_log,
            owns_cache,
            dbname,
            table_cache,

            db_lock,
            mutex,
            shutting_down: core::sync::atomic::AtomicBool::new(false),

            background_work_finished_mutex,
            background_work_finished_signal,

            mem: core::ptr::null_mut(),
            imm: core::ptr::null_mut(),
            has_imm: core::sync::atomic::AtomicBool::new(false),

            logfile,
            logfile_number: 0,
            log: core::ptr::null_mut(),

            seed: 0,
            tmp_batch: Box::into_raw(Box::new(WriteBatch::default())),

            background_compaction_scheduled: false,
            manual_compaction: core::ptr::null_mut(),

            versions,

            writers: Default::default(),
            snapshots: Default::default(),
            pending_outputs: Default::default(),
            bg_error: Status::ok(),
            stats: Default::default(),
        }
    }
}

#[cfg(test)]
mod dbimpl_construction_and_user_comparator_suite {
    use super::*;

    fn build_temp_db_path_for_dbimpl_create_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dir = std::env::temp_dir();
        let path = dir.join(format!("bitcoinleveldb_dbimpl_create_suite_{}", nanos));
        let s = path.to_string_lossy().to_string();

        tracing::info!(path = %s, "Allocated temp db path for DBImpl::new suite");
        s
    }

    fn build_options_with_explicit_bytewise_comparator_for_dbimpl_create_suite() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        // Ensure we have an Env. If the default did not provide one, fail loudly:
        // DBImpl::new requires an Env to be present either in sanitized options or raw options.
        if options.env().is_none() {
            tracing::error!("Options::default() did not supply an Env; DBImpl::new suite cannot proceed");
            panic!();
        }

        // Use explicit comparator + filter policy to make comparator behavior deterministic.
        options.set_comparator(std::sync::Arc::new(BytewiseComparatorImpl::default()));
        options.set_filter_policy(std::sync::Arc::new(NullFilterPolicy::default()));

        tracing::info!(
            comparator = %options.comparator().as_ref().name(),
            filter_policy = %options.filter_policy().as_ref().name(),
            "Built Options for DBImpl::new suite"
        );

        options
    }

    #[traced_test]
    fn dbimpl_new_initializes_core_invariants_and_non_null_pointers() {
        let dbname = build_temp_db_path_for_dbimpl_create_suite();
        let options = build_options_with_explicit_bytewise_comparator_for_dbimpl_create_suite();

        let _ = std::fs::create_dir_all(&dbname);

        let db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        tracing::info!(
            dbname = %db.dbname,
            owns_info_log = db.owns_info_log,
            owns_cache = db.owns_cache,
            "Constructed DBImpl via DBImpl::new"
        );

        assert_eq!(db.dbname, dbname, "DBImpl must retain dbname verbatim");

        assert!(!db.table_cache.is_null(), "DBImpl must allocate a TableCache");
        assert!(!db.versions.is_null(), "DBImpl must allocate a VersionSet");

        assert!(db.mem.is_null(), "DBImpl::new must not allocate memtable before Open()");
        assert!(db.imm.is_null(), "DBImpl::new must start with no immutable memtable");
        assert!(!db.has_imm.load(core::sync::atomic::Ordering::Acquire), "has_imm must be false at construction");

        assert_eq!(db.logfile_number, 0, "logfile_number must start at 0");
        assert!(db.log.is_null(), "log writer pointer must start null");

        assert!(!db.tmp_batch.is_null(), "tmp_batch must be allocated");
        assert!(db.manual_compaction.is_null(), "manual_compaction must start null");
        assert!(!db.background_compaction_scheduled, "background_compaction_scheduled must start false");
        assert!(db.bg_error.is_ok(), "bg_error must start OK");

        // Ensure returned user comparator is usable as an interface object.
        let user_cmp = db.user_comparator();
        let a = Slice::from_str("a");
        let b = Slice::from_str("b");

        let ordering = user_cmp.compare(&a, &b);
        tracing::debug!(
            a = %a.to_string(),
            b = %b.to_string(),
            ordering,
            name = %user_cmp.name(),
            "Validated user comparator compare() and name()"
        );

        assert!(ordering < 0, "Bytewise comparator must order 'a' < 'b'");

        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn dbimpl_user_comparator_delegates_to_raw_options_comparator_interface() {
        let dbname = build_temp_db_path_for_dbimpl_create_suite();
        let options = build_options_with_explicit_bytewise_comparator_for_dbimpl_create_suite();

        let _ = std::fs::create_dir_all(&dbname);

        let db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        let raw_name = options.comparator().as_ref().name().to_string();
        let user_cmp = db.user_comparator();
        let user_name = user_cmp.name().to_string();

        tracing::info!(
            raw_name = %raw_name,
            user_name = %user_name,
            "Comparing raw options comparator name vs db.user_comparator() name"
        );

        assert_eq!(
            user_name, raw_name,
            "DBImpl::user_comparator must expose the same comparator identity as options.comparator"
        );

        // Compare semantics: ensure multiple orderings hold.
        let cases = [
            ("", "", 0),
            ("a", "a", 0),
            ("a", "b", -1),
            ("b", "a", 1),
            ("abc", "abd", -1),
            ("abd", "abc", 1),
            ("abc", "abc\0", -1),
        ];

        for (lhs, rhs, sign) in cases.iter().copied() {
            let l = Slice::from_str(lhs);
            let r = Slice::from_str(rhs);
            let got = user_cmp.compare(&l, &r);

            tracing::trace!(
                lhs = %lhs,
                rhs = %rhs,
                got,
                expected_sign = sign,
                "Comparator case"
            );

            match sign {
                0 => assert_eq!(got, 0, "Expected equality for lhs={lhs:?} rhs={rhs:?}"),
                -1 => assert!(got < 0, "Expected lhs < rhs for lhs={lhs:?} rhs={rhs:?}"),
                1 => assert!(got > 0, "Expected lhs > rhs for lhs={lhs:?} rhs={rhs:?}"),
                _ => {
                    tracing::error!(sign, "Invalid expected sign in test vector");
                    panic!();
                }
            }
        }

        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn dbimpl_new_panics_if_no_env_is_provided_in_any_options_path() {
        let dbname = build_temp_db_path_for_dbimpl_create_suite();

        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);
        options.set_env(None);

        tracing::info!(
            dbname = %dbname,
            "Invoking DBImpl::new with env=None; expecting panic"
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _db = DBImpl::new(&options, &dbname);
        }));

        assert!(
            result.is_err(),
            "DBImpl::new must not proceed without an Env"
        );

        let _ = std::fs::remove_dir_all(&dbname);
    }
}
