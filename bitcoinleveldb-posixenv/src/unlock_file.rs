// ---------------- [ File: bitcoinleveldb-posixenv/src/unlock_file.rs ]
crate::ix!();

impl UnlockFile for PosixEnv {
    
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
            PosixFileLock* posix_file_lock = static_cast<PosixFileLock*>(lock);
        if (LockOrUnlock(posix_file_lock->fd(), false) == -1) {
          return posix_error("unlock " + posix_file_lock->filename(), errno);
        }
        locks_.Remove(posix_file_lock->filename());
        ::close(posix_file_lock->fd());
        delete posix_file_lock;
        return crate::Status::OK();
        */
    }
}
