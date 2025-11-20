// ---------------- [ File: bitcoinleveldb-file/src/delete_file.rs ]
crate::ix!();

pub trait DeleteFile {

    /**
      | Delete the named file.
      |
      */
    fn delete_file(&mut self, fname: &String) -> crate::Status;
}
