// ---------------- [ File: bitcoinleveldb-version/src/version.rs ]
crate::ix!();


/**
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
#[derive(Builder,Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
#[builder(pattern="owned")]
pub struct Version {

    /**
      | VersionSet to which this Version belongs
      |
      */
    #[getset(skip)]
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

    pub fn vset(&self) -> *mut dyn VersionSetInterface {
        self.vset
    }

    pub fn num_files(&self, level: i32) -> i32 {
        trace!("Version::num_files: level={}", level);
        assert!(
            level >= 0 && (level as usize) < NUM_LEVELS,
            "Version::num_files: level {} out of range 0..{}",
            level,
            NUM_LEVELS - 1
        );

        let count = self.files[level as usize].len() as i32;
        debug!(
            "Version::num_files: level={} has {} files",
            level, count
        );
        count
    }
}

#[cfg(test)]
mod version_core_behavior_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[traced_test]
    fn num_files_zero_for_all_levels_when_empty() {
        let version = helpers::build_empty_version();

        for level in 0..NUM_LEVELS {
            let count = version.num_files(level as i32);
            assert_eq!(
                count, 0,
                "Empty Version must have zero files at level {}",
                level
            );
        }
    }

    #[traced_test]
    fn num_files_reflects_actual_counts_per_level() {
        let mut version = helpers::build_empty_version();

        {
            let files = version.files_mut();
            files[0].push(helpers::build_file_meta_boxed(1, 10, "a", "b"));
            files[0].push(helpers::build_file_meta_boxed(2, 10, "c", "d"));
            files[1].push(helpers::build_file_meta_boxed(3, 10, "e", "f"));
        }

        assert_eq!(
            version.num_files(0),
            2,
            "Level 0 should report two files after insertion"
        );
        assert_eq!(
            version.num_files(1),
            1,
            "Level 1 should report one file after insertion"
        );
        assert_eq!(
            version.num_files((NUM_LEVELS - 1) as i32),
            0,
            "Highest level with no files must report zero"
        );
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn num_files_panics_on_negative_level() {
        let version = helpers::build_empty_version();
        let _ = version.num_files(-1);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn num_files_panics_on_level_at_or_above_num_levels() {
        let version = helpers::build_empty_version();
        let _ = version.num_files(NUM_LEVELS as i32);
    }
}
