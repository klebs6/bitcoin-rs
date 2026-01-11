// ---------------- [ File: bitcoinleveldb-dbimpl/src/create.rs ]
crate::ix!();

impl DBImpl {
    
    /// Extra methods (for testing) that are
    /// not in the public DB interface
    /// 
    pub fn user_comparator(&self) -> Box<dyn SliceComparator> {
        self.internal_comparator_.user_comparator()
    }
    
    pub fn new(raw_options: &Options, dbname: &String) -> Self {
        let env_ = raw_options.env().clone();

        let internal_comparator_ = InternalKeyComparator::new(raw_options.comparator());
        let internal_filter_policy_ = InternalFilterPolicy::new(raw_options.filter_policy());

        let options_ = sanitize_options(
            dbname,
            &internal_comparator_,
            &internal_filter_policy_,
            raw_options,
        );

        let owns_info_log_ = options_.info_log != raw_options.info_log;
        let owns_cache_ = options_.block_cache != raw_options.block_cache;

        let dbname_ = dbname.clone();

        let table_cache_ = Box::into_raw(Box::new(TableCache::new(
            &dbname_,
            &options_,
            table_cache_size(&options_),
        )));

        let mut mutex_: Mutex = Default::default();
        let background_work_finished_signal_ = ConditionVariable::new(&mut mutex_);

        let versions_ = Box::into_raw(Box::new(VersionSet::new(
            &dbname_,
            &options_,
            table_cache_,
            &internal_comparator_,
        )));

        Self {
            env_,
            internal_comparator_,
            internal_filter_policy_,
            options_,
            owns_info_log_,
            owns_cache_,
            dbname_,
            table_cache_,

            db_lock_: core::ptr::null_mut(),
            mutex_,
            shutting_down_: core::sync::atomic::AtomicBool::new(false),

            background_work_finished_signal_,

            mem_: core::ptr::null_mut(),
            imm_: core::ptr::null_mut(),
            has_imm_: core::sync::atomic::AtomicBool::new(false),

            logfile_: core::ptr::null_mut(),
            logfile_number_: 0,
            log_: core::ptr::null_mut(),

            seed_: 0,
            tmp_batch_: Box::into_raw(Box::new(WriteBatch::default())),

            background_compaction_scheduled_: false,
            manual_compaction_: core::ptr::null_mut(),

            versions_,

            writers_: Default::default(),
            snapshots_: Default::default(),
            pending_outputs_: Default::default(),
            bg_error_: Status::ok(),
            stats_: Default::default(),
        }
    }
}

#[cfg(test)]
#[disable]
mod create_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn user_comparator_is_exposed_and_new_constructs_safely() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("user_comparator_is_exposed_and_new_constructs_safely");
        remove_db_dir_best_effort(&dbname);

        let db: DBImpl = DBImpl::new(&opts, &dbname);
        let cmp = db.user_comparator();
        let name = cmp.name();
        tracing::info!(comparator = %name, "user comparator name");

        assert!(!name.is_empty(), "comparator name should be non-empty");

        remove_db_dir_best_effort(&dbname);
    }
}
