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
        level: i32,
        file: u64,
        file_size: u64,
        smallest: &InternalKey,
        largest: &InternalKey,
    ) {
        trace!(
            "VersionEdit::add_file: level={}, file={}, file_size={}",
            level,
            file,
            file_size
        );

        let mut meta = FileMetaData::default();
        // FileMetaData::default() is expected to initialize refs/allowed_seeks.
        meta.number    = file;
        meta.file_size = file_size;
        meta.smallest  = smallest.clone();
        meta.largest   = largest.clone();

        self.new_files.push((level, meta));

        debug!(
            "VersionEdit::add_file: added file {} at level {} (size={})",
            file, level, file_size
        );
    }
}
