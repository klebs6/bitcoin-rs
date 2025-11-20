// ---------------- [ File: bitcoinleveldb-file/src/new_appendable_file.rs ]
crate::ix!();

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
