// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_debug_string.rs ]
crate::ix!();

impl VersionEdit {

    pub fn debug_string(&self) -> String {
        trace!("VersionEdit::debug_string: generating human-readable summary");

        let mut r = String::from("VersionEdit {");

        if *self.has_comparator() {
            r.push_str("\n  Comparator: ");
            r.push_str(self.comparator());
        }
        if *self.has_log_number() {
            r.push_str("\n  LogNumber: ");
            r.push_str(&self.log_number().to_string());
        }
        if *self.has_prev_log_number() {
            r.push_str("\n  PrevLogNumber: ");
            r.push_str(&self.prev_log_number().to_string());
        }
        if *self.has_next_file_number() {
            r.push_str("\n  NextFile: ");
            r.push_str(&self.next_file_number().to_string());
        }
        if *self.has_last_sequence() {
            r.push_str("\n  LastSeq: ");
            r.push_str(&self.last_sequence().to_string());
        }

        for (level, key) in self.compact_pointers() {
            r.push_str("\n  CompactPointer: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&key.debug_string());
        }

        for (level, number) in self.deleted_files() {
            r.push_str("\n  DeleteFile: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&number.to_string());
        }

        for (level, f) in self.new_files() {
            r.push_str("\n  AddFile: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&f.number().to_string());
            r.push(' ');
            r.push_str(&f.file_size().to_string());
            r.push(' ');
            r.push_str(&f.smallest().debug_string());
            r.push_str(" .. ");
            r.push_str(&f.largest().debug_string());
        }

        r.push_str("\n}\n");

        r
    }
}

#[cfg(test)]
mod version_edit_debug_string_tests {
    use super::*;

    #[traced_test]
    fn debug_string_includes_all_set_fields_and_file_changes() {
        trace!("debug_string_includes_all_set_fields_and_file_changes: start");

        const K_BIG: u64 = 1u64 << 40;

        let mut edit = VersionEdit::default();

        let cmp_name  = String::from("cmp");
        let cmp_slice = Slice::from(&cmp_name);
        edit.set_comparator_name(&cmp_slice);
        edit.set_log_number(K_BIG + 1);
        edit.set_prev_log_number(K_BIG + 2);
        edit.set_next_file(K_BIG + 3);
        edit.set_last_sequence((K_BIG + 4) as SequenceNumber);

        let user_smallest = Slice::from("a".as_bytes());
        let user_largest  = Slice::from("z".as_bytes());

        let smallest = InternalKey::new(
            &user_smallest,
            (K_BIG + 5) as SequenceNumber,
            ValueType::TypeValue,
        );
        let largest = InternalKey::new(
            &user_largest,
            (K_BIG + 6) as SequenceNumber,
            ValueType::TypeValue,
        );

        edit.add_file(1, K_BIG + 7, K_BIG + 8, &smallest, &largest);
        edit.delete_file(2, K_BIG + 9);

        let compact_key = InternalKey::new(
            &Slice::from("x".as_bytes()),
            (K_BIG + 10) as SequenceNumber,
            ValueType::TypeValue,
        );
        edit.set_compact_pointer(3, &compact_key);

        let dbg = edit.debug_string();
        debug!(debug_string = %dbg, "version_edit debug_string output");

        assert!(
            dbg.contains("Comparator: cmp"),
            "debug_string should contain comparator name"
        );
        assert!(
            dbg.contains("LogNumber:"),
            "debug_string should contain LogNumber"
        );
        assert!(
            dbg.contains("PrevLogNumber:"),
            "debug_string should contain PrevLogNumber"
        );
        assert!(
            dbg.contains("NextFile:"),
            "debug_string should contain NextFile"
        );
        assert!(
            dbg.contains("LastSeq:"),
            "debug_string should contain LastSeq"
        );
        assert!(
            dbg.contains("CompactPointer:"),
            "debug_string should contain CompactPointer entries"
        );
        assert!(
            dbg.contains("DeleteFile:"),
            "debug_string should contain DeleteFile entries"
        );
        assert!(
            dbg.contains("AddFile:"),
            "debug_string should contain AddFile entries"
        );
    }
}
