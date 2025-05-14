// ---------------- [ File: bitcoinleveldb-cfg/src/cfg.rs ]
crate::ix!();

/**
  | Grouping of constants. We may want to
  | make some of these parameters set via
  | options.
  |
  */
pub const NUM_LEVELS: usize = 7;

/**
  | Level-0 compaction is started when
  | we hit this many files.
  |
  */
pub const L0_COMPACTION_TRIGGER: usize = 4;

/**
  | Soft limit on number of level-0 files.
  | We slow down writes at this point.
  |
  */
pub const L0_SLOWDOWN_WRITES_TRIGGER: usize = 8;

/**
  | Maximum number of level-0 files. We
  | stop writes at this point.
  |
  */
pub const L0_STOP_WRITES_TRIGGER: usize = 12;

/**
  | Maximum level to which a new compacted memtable
  | is pushed if it does not create overlap.  We
  | try to push to level 2 to avoid the relatively
  | expensive level 0=>1 compactions and to avoid
  | some expensive manifest file operations.  We do
  | not push all the way to the largest level since
  | that can generate a lot of wasted disk space if
  | the same key space is being repeatedly
  | overwritten.
  */
pub const MAX_MEM_COMPACT_LEVEL: usize = 2;

/**
  | Approximate gap in bytes between samples
  | of data read during iteration.
  |
  */
pub const READ_BYTES_PERIOD: usize = 1048576;
