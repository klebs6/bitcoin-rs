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

    /**
      | State below is protected by mutex_
      |
      */
    mutex:                  Mutex<DBImplInner>,

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
