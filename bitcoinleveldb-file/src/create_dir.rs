// ---------------- [ File: bitcoinleveldb-file/src/create_dir.rs ]
crate::ix!();

pub trait CreateDir {

    /**
      | Create the specified directory.
      |
      */
    fn create_dir(&mut self, dirname: &String) -> crate::Status;
}
