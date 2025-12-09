// ---------------- [ File: bitcoinleveldb-dbimplinner/src/inner.rs ]
crate::ix!();

pub struct DBImplInner {

    background_work_finished_signal: Condvar,

    /**
      | Memtable being compacted
      |
      */
    imm: *mut MemTable,

    logfile_number: u64,

    /**
      | For sampling.
      |
      */
    seed:  u32,

    /**
      | Queue of writers.
      |
      */
    writers:                         VecDeque<*mut DBImplWriter>,
    tmp_batch:                       *mut WriteBatch,
    snapshots:                       SnapshotList,

    /**
      | Set of table files to protect from deletion
      | because they are part of ongoing compactions.
      |
      */
    pending_outputs:                 HashSet<u64>,

    /**
      | Has a background compaction been scheduled
      | or is running?
      |
      */
    background_compaction_scheduled: bool,
    manual_compaction:               *mut ManualCompaction,
    versions:                        *const VersionSet,

    /**
      | Have we encountered a background error
      | in paranoid mode?
      |
      */
    bg_error:                        Status,
    stats:                           [CompactionStats; NUM_LEVELS],
}
