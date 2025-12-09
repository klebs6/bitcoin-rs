// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_db.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn newdb(&mut self) -> crate::Status {
        
        todo!();
        /*
            VersionEdit new_db;
      new_db.SetComparatorName(user_comparator()->Name());
      new_db.SetLogNumber(0);
      new_db.SetNextFile(2);
      new_db.SetLastSequence(0);

      const std::string manifest = DescriptorFileName(dbname_, 1);
      WritableFile* file;
      Status s = env_->NewWritableFile(manifest, &file);
      if (!s.ok()) {
        return s;
      }
      {
        LogWriter log(file);
        std::string record;
        new_db.EncodeTo(&record);
        s = log.AddRecord(record);
        if (s.ok()) {
          s = file->Close();
        }
      }
      delete file;
      if (s.ok()) {
        // Make "CURRENT" file that points to the new manifest file.
        s = SetCurrentFile(env_, dbname_, 1);
      } else {
        env_->DeleteFile(manifest);
      }
      return s;
        */
    }
}
