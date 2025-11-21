// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_clear.rs ]
crate::ix!();

impl VersionEdit {

    pub fn clear(&mut self) {
        trace!(
            "VersionEdit::clear: resetting mutable state \
             (preserving compact_pointers)"
        );
        self.reset_core_state();
    }
}

#[cfg(test)]
mod version_edit_clear_tests {
    use super::*;

    #[traced_test]
    fn clear_resets_state_but_preserves_compact_pointers() {
        trace!("clear_resets_state_but_preserves_compact_pointers: start");

        let mut edit = VersionEdit::default();

        // Populate scalar fields and file collections.
        let cmp_name  = String::from("cmp");
        let cmp_slice = Slice::from(&cmp_name);
        edit.set_comparator_name(&cmp_slice);
        edit.set_log_number(1);
        edit.set_prev_log_number(2);
        edit.set_next_file(3);
        edit.set_last_sequence(4 as SequenceNumber);

        edit.deleted_files_mut().insert((0, 100));
        edit.new_files_mut().push((1, FileMetaData::default()));

        // Add compact pointer to verify it survives clear().
        let user = Slice::from("key".as_bytes());
        let ikey = InternalKey::new(&user, 5 as SequenceNumber, ValueType::TypeValue);
        edit.set_compact_pointer(2, &ikey);

        edit.clear();

        assert!(!*edit.has_comparator());
        assert!(!*edit.has_log_number());
        assert!(!*edit.has_prev_log_number());
        assert!(!*edit.has_next_file_number());
        assert!(!*edit.has_last_sequence());

        assert_eq!(*edit.log_number(), 0);
        assert_eq!(*edit.prev_log_number(), 0);
        assert_eq!(*edit.next_file_number(), 0);
        assert_eq!(*edit.last_sequence(), 0 as SequenceNumber);

        assert!(edit.deleted_files().is_empty(), "deleted_files should be cleared");
        assert!(edit.new_files().is_empty(), "new_files should be cleared");

        assert_eq!(
            edit.compact_pointers().len(),
            1,
            "clear should preserve compact_pointers contents"
        );
    }
}
