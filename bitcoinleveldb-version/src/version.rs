// ---------------- [ File: bitcoinleveldb-version/src/version.rs ]
/*!
  | The representation of a DBImpl consists of
  | a set of Versions. The newest version is
  | called "current". Older versions may be kept
  | around to provide a consistent view to live
  | iterators.
  |
  | Each Version keeps track of a set of Table
  | files per level. The entire set of versions is
  | maintained in a VersionSet.
  |
  | Version,VersionSet are thread-compatible, but
  | require external synchronization on all
  | accesses.
  */

crate::ix!();

pub struct Version {

    /**
      | VersionSet to which this Version belongs
      |
      */
    vset:                  *mut dyn VersionSetInterface,

    /**
      | Next version in linked list
      |
      */
    next:                  *mut Version,

    /**
      | Previous version in linked list
      |
      */
    prev:                  *mut Version,

    /**
      | Number of live refs to this version
      |
      */
    refs:                  i32,

    /**
      | List of files per level
      |
      */
    files:                 [Vec<*mut FileMetaData>; NUM_LEVELS],

    /**
      | Next file to compact based on seek stats.
      |
      */
    file_to_compact:       *mut FileMetaData,

    file_to_compact_level: i32,

    /**
      | Level that should be compacted next
      | and its compaction score. Score < 1 means
      | compaction is not strictly needed.
      | These fields are initialized by Finalize().
      |
      */
    compaction_score:      f64,
    compaction_level:      i32,
}

impl Version {

    pub fn num_files(&self, level: i32) -> i32 {
        
        todo!();
        /*
            return files_[level].size();
        */
    }
}
