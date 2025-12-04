// ---------------- [ File: bitcoinleveldb-table/src/table_builder_status.rs ]
crate::ix!();

impl TableBuilder {
    
    /**
      | Return non-ok iff some error has been
      | detected.
      |
      */
    pub fn status(&self) -> Status {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::status: rep pointer is null"
            );
            let r = &*self.rep;
            r.status.clone()
        }
    }
}
