// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.cc]
pub struct DBImpl {

    /**
      | Constant after construction
      |
      */
    env:                    Box<dyn Env>,

    internal_comparator:    InternalKeyComparator,
    internal_filter_policy: InternalFilterPolicy,

    /**
      | options_.comparator == &internal_comparator_
      |
      */
    options:                Options,

    owns_info_log:          bool,
    owns_cache:             bool,
    dbname:                 String,

    /**
      | table_cache_ provides its own synchronization
      |
      */
    table_cache:            *const TableCache,

    /**
      | Lock over the persistent DB state.
      | 
      | Non-null iff successfully acquired.
      |
      */
    db_lock:                Rc<RefCell<dyn FileLock>>,

    /// State below is protected by mutex_
    mutex:                  RawMutex,

    //--------------------------------------------[mutex-guarded-fields]
    background_work_finished_signal: Condvar,

    /// Memtable being compacted
    /// 
    imm: *mut MemTable,

    logfile_number: u64,

    /// For sampling.
    /// 
    seed: u32,

    /// Queue of writers.
    /// 
    writers: VecDeque<*mut DBImplWriter>,
    tmp_batch: *mut WriteBatch,
    snapshots: SnapshotList,

    /// Set of table files to protect from deletion
    /// because they are part of ongoing compactions.
    /// 
    pending_outputs: HashSet<u64>,

    /// Has a background compaction been scheduled
    /// or is running?
    /// 
    background_compaction_scheduled: bool,
    manual_compaction: *mut ManualCompaction,
    versions: *const VersionSet,

    /// Have we encountered a background error
    /// in paranoid mode?
    /// 
    bg_error: Status,
    stats: [CompactionStats; NUM_LEVELS],

    //--------------------------------------------[marks-end-of-mutex-guarded-fields]

    shutting_down:          AtomicBool,

    mem:                    *mut MemTable,

    /**
      | So bg thread can detect non-null imm_
      |
      */
    has_imm:                AtomicBool,

    logfile:                Rc<RefCell<dyn WritableFile>>,
    log:                    *mut LogWriter,
}

impl DB for DBImpl { }

#[cfg(test)]
#[disable]
mod dbimpl_struct_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn dbimpl_new_initializes_expected_defaults_and_owns_tmp_batch() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("dbimpl_new_initializes_expected_defaults_and_owns_tmp_batch");
        remove_db_dir_best_effort(&dbname);

        let db: DBImpl = DBImpl::new(&opts, &dbname);

        tracing::info!(dbname = %dbname, "constructed DBImpl");

        assert!(!db.background_compaction_scheduled_, "should not start scheduled");
        assert!(db.manual_compaction_.is_null(), "manual compaction should be null");
        assert!(db.mem_.is_null(), "memtable should be null before open path initializes it");
        assert!(db.imm_.is_null(), "imm should start null");
        assert!(db.bg_error.is_ok(), "bg_error should start OK");
        assert!(!db.tmp_batch_.is_null(), "tmp_batch should be allocated");
        assert_eq!(
            unsafe { WriteBatchInternal::count(db.tmp_batch_) },
            0,
            "tmp batch should start empty"
        );

        remove_db_dir_best_effort(&dbname);
    }
}
