// ---------------- [ File: bitcoinleveldb-env/src/interface.rs ]
/*!
  | An Env is an interface used by the leveldb
  | implementation to access operating system
  | functionality like the filesystem etc.  Callers
  | may wish to provide a custom Env object when
  | opening a database to get fine gain control;
  | e.g., to rate limit file system operations.
  |
  | All Env implementations are safe for concurrent
  | access from multiple threads without any
  | external synchronization.
  */
crate::ix!();

/**
  | in the c++, we had the following annotated
  | empty Default impl:
  |
  | Return a default environment suitable for the
  | current operating system.  Sophisticated
  | users may wish to provide their own Env
  | implementation instead of relying on this
  | default environment.
  |
  | The result of Default() belongs to leveldb
  | and must never be deleted.
  */
pub trait Env:
NewSequentialFile
+ NewRandomAccessFile
+ NewWritableFile
+ NewAppendableFile
+ FileExists
+ GetChildren
+ DeleteFile
+ CreateDir
+ DeleteDir
+ GetFileSize
+ RenameFile
+ LockFile
+ UnlockFile
+ Schedule
+ StartThread
+ GetTestDirectory
+ NewLogger
+ NowMicros
+ SleepForMicroseconds {

    fn new_appendable_file(&mut self, 
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            return crate::Status::NotSupported("NewAppendableFile", fname);
        */
    }

    fn posix_default(&mut self) -> Rc<RefCell<dyn Env>> {
        
        todo!();
        /*
            static PosixDefaultEnv env_container;
      return env_container.env();
        */
    }
}
