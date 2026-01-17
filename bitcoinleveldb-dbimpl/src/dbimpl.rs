// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.cc]
pub struct DBImpl {

    /// Constant after construction
    /// 
    pub env:                    Box<dyn Env>,

    pub internal_comparator:    InternalKeyComparator,
    pub internal_filter_policy: InternalFilterPolicy,

    /// options.comparator == &internal_comparator
    /// 
    pub options:                Options,

    pub owns_info_log:          bool,
    pub owns_cache:             bool,
    pub dbname:                 String,

    /// table_cache provides its own synchronization
    /// 
    pub table_cache:            *const TableCache,

    /// Lock over the persistent DB state.
    /// 
    /// Non-null iff successfully acquired.
    /// 
    pub db_lock:                *mut Box<dyn FileLock>,

    /// State below is protected by mutex
    pub mutex:                  RawMutex,

    //--------------------------------------------[mutex-guarded-fields]

    /// Dedicated mutex used only to provide a `MutexGuard` for `Condvar::wait()`,
    /// since the DB state lock is a `RawMutex`.
    ///
    /// Lock-ordering rule:
    /// - always acquire `mutex` before acquiring this mutex
    /// - never acquire `mutex` while holding this mutex
    pub background_work_finished_mutex: Mutex<()>,

    pub background_work_finished_signal: Condvar,

    /// Memtable being compacted
    /// 
    pub imm: *mut MemTable,

    pub logfile_number: u64,

    /// For sampling.
    /// 
    pub seed: u32,

    /// Queue of writers.
    /// 
    pub writers: VecDeque<*mut DBImplWriter>,
    pub tmp_batch: *mut WriteBatch,
    pub snapshots: SnapshotList,

    /// Set of table files to protect from deletion
    /// because they are part of ongoing compactions.
    /// 
    pub pending_outputs: HashSet<u64>,

    /// Has a background compaction been scheduled
    /// or is running?
    /// 
    pub background_compaction_scheduled: bool,
    pub manual_compaction: *mut ManualCompaction,
    pub versions: *mut VersionSet,

    /// Have we encountered a background error
    /// in paranoid mode?
    /// 
    pub bg_error: Status,
    pub stats: [CompactionStats; NUM_LEVELS],

    //--------------------------------------------[marks-end-of-mutex-guarded-fields]

    pub shutting_down:          AtomicBool,

    pub mem:                    *mut MemTable,

    /// So bg thread can detect non-null imm
    /// 
    pub has_imm:                AtomicBool,

    pub logfile:                Rc<RefCell<dyn WritableFile>>,
    pub log:                    *mut LogWriter,

}

impl DB for DBImpl { }
