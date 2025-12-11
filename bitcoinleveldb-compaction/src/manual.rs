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

#[cfg(test)]
mod manual_compaction_struct_tests {
    use super::*;

    #[traced_test]
    fn manual_compaction_fields_can_be_initialized() {
        let begin_key = InternalKey::default();
        let end_key   = InternalKey::default();

        let manual = ManualCompaction {
            level: 1,
            done:  false,
            begin: &begin_key as *const InternalKey,
            end:   &end_key as *const InternalKey,
            tmp_storage: InternalKey::default(),
        };

        assert_eq!(manual.level, 1);
        assert!(!manual.done);
        assert_eq!(manual.begin, &begin_key as *const InternalKey);
        assert_eq!(manual.end,   &end_key as *const InternalKey);
    }

    #[traced_test]
    fn manual_compaction_allows_unbounded_key_range_via_null_pointers() {
        let manual = ManualCompaction {
            level: 0,
            done:  false,
            begin: core::ptr::null::<InternalKey>(),
            end:   core::ptr::null::<InternalKey>(),
            tmp_storage: InternalKey::default(),
        };

        assert_eq!(manual.level, 0);
        assert!(!manual.done);
        assert!(manual.begin.is_null());
        assert!(manual.end.is_null());
    }

    #[traced_test]
    fn manual_compaction_progress_can_be_marked_done() {
        let begin_key = InternalKey::default();

        let mut manual = ManualCompaction {
            level: 2,
            done:  false,
            begin: &begin_key as *const InternalKey,
            end:   core::ptr::null::<InternalKey>(),
            tmp_storage: InternalKey::default(),
        };

        assert!(!manual.done);
        manual.done = true;
        assert!(manual.done);
    }
}
