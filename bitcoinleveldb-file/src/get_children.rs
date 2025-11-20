// ---------------- [ File: bitcoinleveldb-file/src/get_children.rs ]
crate::ix!();

pub trait GetChildren {

    /**
      | Store in *result the names of the children of
      | the specified directory.
      |
      | The names are relative to "dir".
      |
      | Original contents of *results are dropped.
      */
    fn get_children(&mut self, 
            dir:    &String,
            result: *mut Vec<String>) -> crate::Status;
}
