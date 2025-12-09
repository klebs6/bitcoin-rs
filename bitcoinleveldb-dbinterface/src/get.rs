// ---------------- [ File: bitcoinleveldb-dbinterface/src/get.rs ]
crate::ix!();

pub trait Get {

    /**
      | If the database contains an entry for "key"
      | store the corresponding value in *value and
      | return OK.
      |
      | If there is no entry for "key" leave *value
      | unchanged and return a status for which
      | Status::IsNotFound() returns true.
      |
      | May return some other Status on an error.
      */
    fn get(&mut self, 
            options: &ReadOptions,
            key_:     &Slice,
            value:   *mut String) -> crate::Status;
}
