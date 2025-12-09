// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_write_snapshot.rs ]
crate::ix!();

impl WriteSnapshot for VersionSet {
    
    /**
      | Save current contents to *log
      |
      */
    fn write_snapshot(&mut self, log: *mut LogWriter) -> Status {
        
        todo!();
        /*
            // TODO: Break up into multiple records to reduce memory usage on recovery?

      // Save metadata
      VersionEdit edit;
      edit.SetComparatorName(icmp_.user_comparator()->Name());

      // Save compaction pointers
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        if (!compact_pointer_[level].empty()) {
          InternalKey key;
          key.DecodeFrom(compact_pointer_[level]);
          edit.SetCompactPointer(level, key);
        }
      }

      // Save files
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        const std::vector<FileMetaData*>& files = current_->files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          const FileMetaData* f = files[i];
          edit.AddFile(level, f->number, f->file_size, f->smallest, f->largest);
        }
      }

      std::string record;
      edit.EncodeTo(&record);
      return log->AddRecord(record);
        */
    }
}
