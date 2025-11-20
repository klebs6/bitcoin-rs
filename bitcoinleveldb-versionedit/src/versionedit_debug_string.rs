// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_debug_string.rs ]
crate::ix!();

impl VersionEdit {

    pub fn debug_string(&self) -> String {
        trace!("VersionEdit::debug_string: generating human-readable summary");

        let mut r = String::from("VersionEdit {");

        if self.has_comparator {
            r.push_str("\n  Comparator: ");
            r.push_str(&self.comparator);
        }
        if self.has_log_number {
            r.push_str("\n  LogNumber: ");
            r.push_str(&self.log_number.to_string());
        }
        if self.has_prev_log_number {
            r.push_str("\n  PrevLogNumber: ");
            r.push_str(&self.prev_log_number.to_string());
        }
        if self.has_next_file_number {
            r.push_str("\n  NextFile: ");
            r.push_str(&self.next_file_number.to_string());
        }
        if self.has_last_sequence {
            r.push_str("\n  LastSeq: ");
            r.push_str(&self.last_sequence.to_string());
        }

        for (level, key) in &self.compact_pointers {
            r.push_str("\n  CompactPointer: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&key.debug_string());
        }

        for (level, number) in &self.deleted_files {
            r.push_str("\n  DeleteFile: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&number.to_string());
        }

        for (level, f) in &self.new_files {
            r.push_str("\n  AddFile: ");
            r.push_str(&level.to_string());
            r.push(' ');
            r.push_str(&f.number.to_string());
            r.push(' ');
            r.push_str(&f.file_size.to_string());
            r.push(' ');
            r.push_str(&f.smallest.debug_string());
            r.push_str(" .. ");
            r.push_str(&f.largest.debug_string());
        }

        r.push_str("\n}\n");

        r
    }
}
