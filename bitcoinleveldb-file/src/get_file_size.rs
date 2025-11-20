// ---------------- [ File: bitcoinleveldb-file/src/get_file_size.rs ]
crate::ix!();

pub trait GetFileSize {

    /**
      | Store the size of fname in *file_size.
      |
      */
    fn get_file_size(&mut self, 
            fname:     &String,
            file_size: *mut u64) -> crate::Status;
}
