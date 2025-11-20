// ---------------- [ File: bitcoinleveldb-file/src/new_writable_file.rs ]
crate::ix!();

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
