// ---------------- [ File: bitcoinleveldb-compaction/src/manual.rs ]
crate::ix!();

/**
  | Information for a manual compaction
  |
  */
pub struct ManualCompaction {
    level:       i32,
    done:        bool,

    /**
      | null means beginning of key range
      |
      */
    begin:       *const InternalKey,

    /**
      | null means end of key range
      |
      */
    end:         *const InternalKey,

    /**
      | Used to keep track of compaction progress
      |
      */
    tmp_storage: InternalKey,
}
