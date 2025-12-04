// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_abandon.rs ]
crate::ix!();

impl TableBuilder {
    
    /**
      | Indicate that the contents of this builder
      | should be abandoned.  Stops using the file
      | passed to the constructor after this function
      | returns.
      |
      | If the caller is not going to call Finish(),
      | it must call Abandon() before destroying this
      | builder.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn abandon(&mut self) {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::abandon: rep pointer is null"
            );
            let r = &mut *self.rep;

            trace!(
                "TableBuilder::abandon: closing builder at offset={}, num_entries={}",
                r.offset,
                r.num_entries
            );

            assert!(
                !r.closed,
                "TableBuilder::abandon: builder already closed"
            );
            r.closed = true;
        }
    }
}
