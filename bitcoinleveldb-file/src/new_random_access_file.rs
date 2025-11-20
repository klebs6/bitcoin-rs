// ---------------- [ File: bitcoinleveldb-file/src/new_random_access_file.rs ]
crate::ix!();

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
