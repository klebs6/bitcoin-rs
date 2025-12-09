// ---------------- [ File: bitcoinleveldb-db/src/destroy_db.rs ]
crate::ix!();

pub fn destroydb(
        dbname:  &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
            Env* env = options.env;
      std::vector<std::string> filenames;
      Status result = env->GetChildren(dbname, &filenames);
      if (!result.ok()) {
        // Ignore error in case directory does not exist
        return Status::OK();
      }

      FileLock* lock;
      const std::string lockname = LockFileName(dbname);
      result = env->LockFile(lockname, &lock);
      if (result.ok()) {
        uint64_t number;
        FileType type;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) &&
              type != kDBLockFile) {  // Lock file will be deleted at end
            Status del = env->DeleteFile(dbname + "/" + filenames[i]);
            if (result.ok() && !del.ok()) {
              result = del;
            }
          }
        }
        env->UnlockFile(lock);  // Ignore error since state is already gone
        env->DeleteFile(lockname);
        env->DeleteDir(dbname);  // Ignore error in case dir contains other files
      }
      return result;
        */
}
