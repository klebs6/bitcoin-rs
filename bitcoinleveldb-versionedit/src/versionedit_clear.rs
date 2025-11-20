// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_clear.rs ]
crate::ix!();

impl VersionEdit {

    pub fn clear(&mut self) {
        trace!("VersionEdit::clear: resetting mutable state");
        self.comparator.clear();
        self.log_number           = 0;
        self.prev_log_number      = 0;
        self.last_sequence        = 0;
        self.next_file_number     = 0;
        self.has_comparator       = false;
        self.has_log_number       = false;
        self.has_prev_log_number  = false;
        self.has_next_file_number = false;
        self.has_last_sequence    = false;
        // NOTE: compact_pointers is intentionally NOT cleared here to match
        // the original LevelDB semantics.
        self.deleted_files.clear();
        self.new_files.clear();
        debug!(
            "VersionEdit::clear: state cleared (compact_pointers preserved, \
             deleted_files and new_files emptied)"
        );
    }
}
