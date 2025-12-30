// ---------------- [ File: bitcoinleveldb-dbinterface/src/write.rs ]
crate::ix!();

pub trait DBInterfaceWrite {

    /**
      | Apply the specified updates to the database.
      |
      | Returns OK on success, non-OK on failure.
      |
      | Note: consider setting options.sync = true.
      */
    fn write(&mut self, 
            options: &WriteOptions,
            updates: *mut WriteBatch) -> crate::Status;
}
