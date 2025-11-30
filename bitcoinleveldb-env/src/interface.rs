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

    fn new_appendable_file(
        &mut self,
        fname:  &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(
            file = %fname,
            "Env::new_appendable_file default: returning Status::not_supported"
        );

        let op_name  = "NewAppendableFile".to_string();
        let msg1     = Slice::from(&op_name);
        let msg2     = Slice::from(fname);

        Status::not_supported(&msg1, Some(&msg2))
    }

}
