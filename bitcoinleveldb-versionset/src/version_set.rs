// ---------------- [ File: bitcoinleveldb-versionset/src/version_set.rs ]
crate::ix!();

///------------------
pub struct VersionSet {

    env:                  Box<dyn Env>,
    dbname:               String,
    options:              *const Options,
    table_cache:          *const TableCache,
    icmp:                 InternalKeyComparator,
    next_file_number:     u64,
    manifest_file_number: u64,
    last_sequence:        u64,
    log_number:           u64,

    /**
      | 0 or backing store for memtable being
      | compacted
      |
      */
    prev_log_number:      u64,

    /**
      | Opened lazily
      |
      */
    descriptor_file:      *mut dyn WritableFile,

    descriptor_log:       *mut LogWriter,

    /**
      | Head of circular doubly-linked list
      | of versions.
      |
      */
    dummy_versions:       Version,

    /**
      | == dummy_versions_.prev_
      |
      */
    current:              *mut Version,

    /**
      | Per-level key at which the next compaction
      | at that level should start.
      | 
      | Either an empty string, or a valid
      | 
      | InternalKey.
      |
      */
    compact_pointer:      [String; NUM_LEVELS],
}

impl GetInternalKeyComparator for VersionSet {
    fn icmp(&self) -> &InternalKeyComparator {
        &self.icmp
    }
}

impl VersionSetInterface for VersionSet {}
impl VersionSetVersionInterface for VersionSet {}
impl CompactionInterface for VersionSet {}

impl Into<Version> for *mut VersionSet {

    fn into(self) -> Version {
    
        todo!();
        /*
        : vset(vset),
        : next(this),
        : prev(this),
        : refs(0),
        : file_to_compact(nullptr),
        : file_to_compact_level(-1),
        : compaction_score(-1),
        : compaction_level(-1),

        
        */
    }
}

impl CurrentVersion for VersionSet {

    /**
      | Return the current version.
      |
      */
    fn current(&self) -> *mut Version {
        
        todo!();
        /*
            return current_;
        */
    }
}

impl ManifestFileNumber for VersionSet {

    /**
      | Return the current manifest file number
      |
      */
    fn manifest_file_number(&self) -> u64 {
        
        todo!();
        /*
            return manifest_file_number_;
        */
    }
}

impl NewFileNumber for VersionSet {

    /**
      | Allocate and return a new file number
      |
      */
    fn new_file_number(&mut self) -> u64 {
        
        todo!();
        /*
            return next_file_number_++;
        */
    }
}

impl ReuseFileNumber for VersionSet {

    /**
      | Arrange to reuse "file_number" unless a newer
      | file number has already been allocated.
      |
      | REQUIRES: "file_number" was returned by
      | a call to NewFileNumber().
      */
    fn reuse_file_number(&mut self, file_number: u64)  {
        
        todo!();
        /*
            if (next_file_number_ == file_number + 1) {
          next_file_number_ = file_number;
        }
        */
    }
}

impl LastSequenceNumber for VersionSet {

    /**
      | Return the last sequence number.
      |
      */
    fn last_sequence(&self) -> u64 {
        
        todo!();
        /*
            return last_sequence_;
        */
    }
}

impl SetLastSequenceNumber for VersionSet {

    /**
      | Set the last sequence number to s.
      |
      */
    fn set_last_sequence(&mut self, s: u64)  {
        
        todo!();
        /*
            assert(s >= last_sequence_);
        last_sequence_ = s;
        */
    }
}

impl GetCurrentLogFileNumber for VersionSet {

    /**
      | Return the current log file number.
      |
      */
    fn log_number(&self) -> u64 {
        
        todo!();
        /*
            return log_number_;
        */
    }
}

impl GetPrevLogFileNumber for VersionSet {

    /**
      | Return the log file number for the log
      | file that is currently being compacted,
      | or zero if there is no such log file.
      |
      */
    fn prev_log_number(&self) -> u64 {
        
        todo!();
        /*
            return prev_log_number_;
        */
    }
}

impl NeedsCompaction for VersionSet {

    /**
      | Returns true iff some level needs a compaction.
      |
      */
    fn needs_compaction(&self) -> bool {
        
        todo!();
        /*
            Version* v = current_;
        return (v->compaction_score_ >= 1) || (v->file_to_compact_ != nullptr);
        */
    }
}
    
impl MarkFileNumberUsed for VersionSet {

    /**
      | Mark the specified file number as used.
      |
      */
    fn mark_file_number_used(&mut self, number: u64)  {
        
        todo!();
        /*
            if (next_file_number_ <= number) {
        next_file_number_ = number + 1;
      }
        */
    }
}
