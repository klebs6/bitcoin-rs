// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_num_entries.rs ]
crate::ix!();

impl TableBuilder {
    
    /**
      | Number of calls to Add() so far.
      |
      */
    pub fn num_entries(&self) -> u64 {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::num_entries: rep pointer is null"
            );
            let r = &*self.rep;
            if r.num_entries < 0 {
                0
            } else {
                r.num_entries as u64
            }
        }
    }
}
