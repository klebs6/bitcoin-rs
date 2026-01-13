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
        let internal_filter_policy = InternalFilterPolicy::new(raw_options.filter_policy().as_ref());

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

        let owns_info_log = options.info_log() != raw_options.info_log();
        let owns_cache = options.block_cache() != raw_options.block_cache();

        let dbname = dbname.clone();

        let table_cache = Box::into_raw(Box::new(TableCache::new(
            &dbname,
            &options,
            table_cache_size(&options),
        )));

        let mutex: RawMutex = RawMutex::INIT;
        let background_work_finished_signal = Condvar::new();

        let versions = Box::into_raw(Box::new(VersionSet::new(
            &dbname,
            &options,
            table_cache,
            &internal_comparator,
        )));

        let db_lock: Rc<RefCell<dyn FileLock>> = Rc::new(RefCell::new(DbImplNullFileLock));

        // Placeholder handle; real logfile is established during Open()/recovery paths.
        let logfile: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(StdoutPrinter {}));

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
