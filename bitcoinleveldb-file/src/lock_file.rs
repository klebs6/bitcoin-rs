// ---------------- [ File: bitcoinleveldb-file/src/lock_file.rs ]
crate::ix!();

/**
  | Identifies a locked file.
  |
  */
pub trait FileLock { }

pub trait LockFile {

    /**
      | Lock the specified file.  Used to prevent
      | concurrent access to the same db by multiple
      | processes.  On failure, stores nullptr in
      | *lock and returns non-OK.
      |
      | On success, stores a pointer to the object
      | that represents the acquired lock in *lock
      | and returns OK.  The caller should call
      | UnlockFile(*lock) to release the lock.  If
      | the process exits, the lock will be
      | automatically released.
      |
      | If somebody else already holds the lock,
      | finishes immediately with a failure.  I.e.,
      | this call does not wait for existing locks to
      | go away.
      |
      | May create the named file if it does not
      | already exist.
      */
    fn lock_file(&mut self, 
            fname: &String,
            lock:  *mut *mut Box<dyn FileLock>) -> crate::Status;
}

pub trait UnlockFile {

    /**
      | Release the lock acquired by a previous
      | successful call to LockFile.
      |
      | REQUIRES: lock was returned by a successful
      | LockFile() call
      |
      | REQUIRES: lock has not already been unlocked.
      */
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status;
}
