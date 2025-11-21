// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_delete_file.rs ]
crate::ix!();

impl VersionEdit {

    /// Delete the specified "file" from the
    /// specified "level".
    /// 
    pub fn delete_file(&mut self, level: i32, file: u64) {
        trace!(
            "VersionEdit::delete_file: marking file {} for deletion at level {}",
            file,
            level
        );
        self.deleted_files_mut().insert((level, file));
    }
}

#[cfg(test)]
mod version_edit_delete_file_tests {
    use super::*;

    #[traced_test]
    fn delete_file_marks_file_in_deleted_set() {
        trace!("delete_file_marks_file_in_deleted_set: start");

        let mut edit = VersionEdit::default();

        let level = 2;
        let file  = 12345_u64;

        edit.delete_file(level, file);

        assert!(
            edit.deleted_files().contains(&(level, file)),
            "delete_file should insert the (level, file) pair into deleted_files"
        );
    }
}
