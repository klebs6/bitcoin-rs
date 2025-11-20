// ---------------- [ File: bitcoinleveldb-file/src/delete_dir.rs ]
crate::ix!();

pub trait DeleteDir {

    /**
      | Delete the specified directory.
      |
      */
    fn delete_dir(&mut self, dirname: &String) -> crate::Status;
}
