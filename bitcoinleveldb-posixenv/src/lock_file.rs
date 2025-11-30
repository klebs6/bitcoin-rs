// ---------------- [ File: bitcoinleveldb-posixenv/src/lock_file.rs ]
crate::ix!();

impl LockFile for PosixEnv {

    fn lock_file(&mut self, 
        filename: &String,
        lock:     *mut *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
        *lock = nullptr;

        int fd = ::open(filename.c_str(), O_RDWR | O_CREAT | kOpenBaseFlags, 0644);
        if (fd < 0) {
          return posix_error(filename, errno);
        }

        if (!locks_.Insert(filename)) {
          ::close(fd);
          return crate::Status::IOError("lock " + filename, "already held by process");
        }

        if (LockOrUnlock(fd, true) == -1) {
          int lock_errno = errno;
          ::close(fd);
          locks_.Remove(filename);
          return posix_error("lock " + filename, lock_errno);
        }

        *lock = new PosixFileLock(fd, filename);
        return crate::Status::OK();
        */
    }
}
