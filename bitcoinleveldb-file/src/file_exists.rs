// ---------------- [ File: bitcoinleveldb-file/src/file_exists.rs ]
crate::ix!();

pub trait FileExists {

    /**
      | Returns true iff the named file exists.
      |
      */
    fn file_exists(&mut self, fname: &String) -> bool;
}
