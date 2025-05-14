// ---------------- [ File: bitcoinleveldb-file/src/file.rs ]
crate::ix!();

pub trait NewSequentialFile {

    /**
      | Create an object that sequentially reads the
      | file with the specified name.
      |
      | On success, stores a pointer to the new file
      | in *result and returns OK.
      |
      | On failure stores nullptr in *result and
      | returns non-OK.  If the file does
      |
      | not exist, returns a non-OK status.
      | Implementations should return a NotFound
      | status when the file does not exist.
      |
      | The returned file will only be accessed by
      | one thread at a time.
      */
    fn new_sequential_file(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn SequentialFile>) -> crate::Status;
}

pub trait NewRandomAccessFile {

    /**
      | Create an object supporting random-access
      | reads from the file with the specified name.
      | On success, stores a pointer to the new file
      | in *result and returns OK.  On failure stores
      | nullptr in *result and returns non-OK.  If
      | the file does not exist, returns a non-OK
      | status.  Implementations should return
      | a NotFound status when the file does not
      | exist.
      |
      | The returned file may be concurrently
      | accessed by multiple threads.
      */
    fn new_random_access_file(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn RandomAccessFile>) -> crate::Status;
}

pub trait NewWritableFile {

    /**
      | Create an object that writes to a new file
      | with the specified name.  Deletes any
      | existing file with the same name and creates
      | a new file.  On success, stores a pointer to
      | the new file in *result and returns OK.  On
      | failure stores nullptr in *result and returns
      | non-OK.
      |
      | The returned file will only be accessed by
      | one thread at a time.
      */
    fn new_writable_file(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn WritableFile>) -> crate::Status;
}

pub trait NewAppendableFile {

    /**
      | Create an object that either appends to an
      | existing file, or writes to a new file (if
      | the file does not exist to begin with).  On
      | success, stores a pointer to the new file in
      | *result and returns OK.  On failure stores
      | nullptr in *result and returns non-OK.
      |
      | The returned file will only be accessed by
      | one thread at a time.
      |
      | May return an IsNotSupportedError error if
      | this Env does not allow appending to an
      | existing file.  Users of Env (including the
      | leveldb implementation) must be prepared to
      | deal with an Env that does not support
      | appending.
      */
    fn new_appendable_file(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn WritableFile>) -> crate::Status;

}

pub trait FileExists {

    /**
      | Returns true iff the named file exists.
      |
      */
    fn file_exists(&mut self, fname: &String) -> bool;
}

pub trait GetChildren {

    /**
      | Store in *result the names of the children of
      | the specified directory.
      |
      | The names are relative to "dir".
      |
      | Original contents of *results are dropped.
      */
    fn get_children(&mut self, 
            dir:    &String,
            result: *mut Vec<String>) -> crate::Status;
}

pub trait DeleteFile {

    /**
      | Delete the named file.
      |
      */
    fn delete_file(&mut self, fname: &String) -> crate::Status;
}

pub trait CreateDir {

    /**
      | Create the specified directory.
      |
      */
    fn create_dir(&mut self, dirname: &String) -> crate::Status;
}

pub trait DeleteDir {

    /**
      | Delete the specified directory.
      |
      */
    fn delete_dir(&mut self, dirname: &String) -> crate::Status;
}

pub trait GetFileSize {

    /**
      | Store the size of fname in *file_size.
      |
      */
    fn get_file_size(&mut self, 
            fname:     &String,
            file_size: *mut u64) -> crate::Status;
}

pub trait RenameFile {

    /**
      | Rename file src to target.
      |
      */
    fn rename_file(&mut self, 
            src:    &String,
            target: &String) -> crate::Status;
}

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

/**
  | A file abstraction for reading sequentially
  | through a file
  |
  */
pub trait SequentialFile: 
SequentialFileRead 
+ SequentialFileSkip 
+ GetName { }

pub trait SequentialFileRead {

    /**
      | Read up to "n" bytes from the file. "scratch[0..n-1]"
      | may be written by this routine. Sets
      | "*result" to the data that was read (including
      | if fewer than "n" bytes were successfully
      | read).
      | 
      | May set "*result" to point at data in
      | "scratch[0..n-1]", so "scratch[0..n-1]"
      | must be live when "*result" is used.
      | 
      | If an error was encountered, returns
      | a non-OK status.
      | 
      | REQUIRES: External synchronization
      |
      */
    fn read(&mut self, 
            n:       usize,
            result:  *mut Slice,
            scratch: *mut u8) -> crate::Status;
}

pub trait SequentialFileSkip {

    /**
      | Skip "n" bytes from the file. This is
      | guaranteed to be no slower that reading the
      | same data, but may be faster.
      |
      | If end of file is reached, skipping will stop
      | at the end of the file, and Skip will return
      | OK.
      |
      | REQUIRES: External synchronization
      */
    fn skip(&mut self, n: u64) -> crate::Status;
}

/**
  | A file abstraction for randomly reading
  | the contents of a file.
  |
  */
pub trait RandomAccessFile: 
RandomAccessFileRead 
+ GetName {}

pub trait RandomAccessFileRead {

    /**
      | Read up to "n" bytes from the file starting
      | at "offset".  "scratch[0..n-1]" may be
      | written by this routine.  Sets "*result" to
      | the data that was read (including if fewer
      | than "n" bytes were successfully read).  May
      | set "*result" to point at data in
      | "scratch[0..n-1]", so "scratch[0..n-1]" must
      | be live when "*result" is used.  If an error
      | was encountered, returns a non-OK status.
      |
      | Safe for concurrent use by multiple threads.
      */
    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status;
}

/**
  | A file abstraction for sequential writing.  The
  | implementation must provide buffering since
  | callers may append small fragments at a time to
  | the file.
  */
pub trait WritableFile: 
WritableFileAppend 
+ WritableFileClose 
+ WritableFileFlush 
+ WritableFileSync 

/*
   | Get a name for the file, only for error
   | reporting
   |
   */
+ GetName {}

pub trait WritableFileAppend {
    fn append(&mut self, data: &Slice) -> crate::Status;
}

pub trait WritableFileClose {
    fn close(&mut self) -> crate::Status;
}

pub trait WritableFileFlush {
    fn flush(&mut self) -> crate::Status;
}

pub trait WritableFileSync {
    fn sync(&mut self) -> crate::Status;
}

impl From<Rc<RefCell<dyn WritableFile>>> for Box<dyn WritableFile> {

    /**
      | Create a writer that will append data to
      | "*dest".
      |
      | "*dest" must be initially empty.
      |
      | "*dest" must remain live while this LogWriter is
      | in use.
      */
    fn from(dest: Rc<RefCell<dyn WritableFile>>) -> Self {
    
        todo!();
        /*
           : dest(dest),
           : block_offset(0),
           InitTypeCrc(type_crc_);
        */
    }
}

/**
  | Identifies a locked file.
  |
  */
pub trait FileLock { }
