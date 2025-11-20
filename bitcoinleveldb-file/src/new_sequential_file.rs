// ---------------- [ File: bitcoinleveldb-file/src/new_sequential_file.rs ]
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
