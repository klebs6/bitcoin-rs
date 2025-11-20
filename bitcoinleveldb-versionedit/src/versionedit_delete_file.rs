// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_delete_file.rs ]
crate::ix!();

impl VersionEdit {

    /**
      | Delete the specified "file" from the
      | specified "level".
      |
      */
    pub fn delete_file(&mut self, level: i32, file: u64) {
        trace!(
            "VersionEdit::delete_file: marking file {} for deletion at level {}",
            file,
            level
        );
        self.deleted_files.insert((level, file));
    }
}
