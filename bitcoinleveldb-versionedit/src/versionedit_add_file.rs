// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_add_file.rs ]
crate::ix!();

impl VersionEdit {

    /**
      | Add the specified file at the specified
      | number.
      |
      | REQUIRES: This version has not been saved
      | (see VersionSet::SaveTo)
      |
      | REQUIRES: "smallest" and "largest" are
      | smallest and largest keys in file
      */
    pub fn add_file(
        &mut self,
        level:     i32,
        file:      u64,
        file_size: u64,
        smallest:  &InternalKey,
        largest:   &InternalKey,
    ) {
        trace!(
            "VersionEdit::add_file: level={}, file={}, file_size={}",
            level,
            file,
            file_size
        );

        let mut meta = FileMetaData::default();

        // FileMetaData::default() is expected to initialize refs/allowed_seeks.
        meta.set_number(file);
        meta.set_file_size(file_size);
        meta.set_smallest(smallest.clone());
        meta.set_largest(largest.clone());

        self.new_files_mut().push((level, meta));

        debug!(
            "VersionEdit::add_file: added file {} at level {} (size={})",
            file,
            level,
            file_size
        );
    }
}

#[cfg(test)]
mod version_edit_add_file_tests {
    use super::*;

    #[traced_test]
    fn add_file_populates_file_metadata_and_appends_entry() {
        trace!("add_file_populates_file_metadata_and_appends_entry: start");

        let mut edit = VersionEdit::default();

        let level     = 3;
        let file_num  = 123_u64;
        let file_size = 4096_u64;

        let smallest_user = Slice::from("foo".as_bytes());
        let largest_user  = Slice::from("zoo".as_bytes());

        let smallest = InternalKey::new(
            &smallest_user,
            100 as SequenceNumber,
            ValueType::TypeValue,
        );
        let largest = InternalKey::new(
            &largest_user,
            200 as SequenceNumber,
            ValueType::TypeValue,
        );

        edit.add_file(level, file_num, file_size, &smallest, &largest);

        assert_eq!(
            edit.new_files().len(),
            1,
            "add_file should append exactly one new file entry"
        );

        let (stored_level, meta) = &edit.new_files()[0];

        assert_eq!(
            *stored_level, level,
            "stored file level should match the level passed to add_file"
        );
        assert_eq!(*meta.number(), file_num);
        assert_eq!(*meta.file_size(), file_size);
        assert_eq!(
            meta.smallest().debug_string(),
            smallest.debug_string(),
            "smallest internal key should match"
        );
        assert_eq!(
            meta.largest().debug_string(),
            largest.debug_string(),
            "largest internal key should match"
        );
    }
}
