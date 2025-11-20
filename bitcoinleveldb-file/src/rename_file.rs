// ---------------- [ File: bitcoinleveldb-file/src/rename_file.rs ]
crate::ix!();

pub trait RenameFile {

    /**
      | Rename file src to target.
      |
      */
    fn rename_file(
        &mut self, 
        src:    &String,
        target: &String) -> crate::Status;
}
