// ---------------- [ File: bitcoinleveldb-table/src/table_builder_drop.rs ]
crate::ix!();

impl Drop for TableBuilder {

    /**
      | REQUIRES: Either Finish() or Abandon()
      | has been called.
      |
      */
    fn drop(&mut self) {
        unsafe {
            if self.rep.is_null() {
                trace!("TableBuilder::drop: rep pointer is null; nothing to free");
                return;
            }

            let rep = &mut *self.rep;

            debug_assert!(
                rep.closed,
                "TableBuilder::drop: Rep must be closed (Finish() or Abandon() not called)"
            );

            trace!(
                "TableBuilder::drop: freeing TableBuilderRep @ {:?}, filter_block={:?}",
                self.rep,
                rep.filter_block
            );

            if !rep.filter_block.is_null() {
                trace!(
                    "TableBuilder::drop: deleting FilterBlockBuilder @ {:?}",
                    rep.filter_block
                );
                let _fb: Box<FilterBlockBuilder> =
                    Box::from_raw(rep.filter_block as *mut FilterBlockBuilder);
                rep.filter_block = core::ptr::null_mut();
            }

            let _rep_box: Box<TableBuilderRep> = Box::from_raw(self.rep);
            self.rep = core::ptr::null_mut();
        }
    }
}
