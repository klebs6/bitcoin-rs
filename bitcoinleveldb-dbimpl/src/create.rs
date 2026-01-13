// ---------------- [ File: bitcoinleveldb-dbimpl/src/create.rs ]
crate::ix!();

impl DBImpl {
    
    /// Extra methods (for testing) that are
    /// not in the public DB interface
    /// 
    pub fn user_comparator(&self) -> Box<dyn SliceComparator> {
        self.internal_comparator.user_comparator()
    }
    
    pub fn new(raw_options: &Options, dbname: &String) -> Self {
        let env = raw_options.env().clone();

        let internal_comparator = InternalKeyComparator::new(raw_options.comparator());
        let internal_filter_policy = InternalFilterPolicy::new(raw_options.filter_policy());

        let options = sanitize_options(
            dbname,
            &internal_comparator,
            &internal_filter_policy,
            raw_options,
        );

        let owns_info_log = options.info_log() != raw_options.info_log();
        let owns_cache = options.block_cache() != raw_options.block_cache();

        let dbname = dbname.clone();

        let table_cache = Box::into_raw(Box::new(TableCache::new(
            &dbname,
            &options,
            table_cache_size(&options),
        )));

        let mut mutex: RawMutex = Default::default();
        let background_work_finished_signal = Condvar::new(&mut mutex);

        let versions = Box::into_raw(Box::new(VersionSet::new(
            &dbname,
            &options,
            table_cache,
            &internal_comparator,
        )));

        Self {
            env,
            internal_comparator,
            internal_filter_policy,
            options,
            owns_info_log,
            owns_cache,
            dbname,
            table_cache,

            db_lock: core::ptr::null_mut(),
            mutex,
            shutting_down: core::sync::atomic::AtomicBool::new(false),

            background_work_finished_signal,

            mem: core::ptr::null_mut(),
            imm: core::ptr::null_mut(),
            has_imm: core::sync::atomic::AtomicBool::new(false),

            logfile: core::ptr::null_mut(),
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
