// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_drop.rs ]
crate::ix!();

impl Drop for TableBuilder {

    /// REQUIRES: Either Finish() or Abandon() has been called.
    fn drop(&mut self) {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            if rep_ptr.is_null() {
                trace!("TableBuilder::drop: rep pointer is null; nothing to free");
                return;
            }

            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            debug_assert!(
                *rep.closed(),
                "TableBuilder::drop: Rep must be closed (Finish() or Abandon() not called)"
            );

            trace!(
                "TableBuilder::drop: freeing TableBuilderRep @ {:?}, filter_block={:?}",
                rep_ptr,
                rep.filter_block()
            );

            let filter_block_ptr: *mut FilterBlockBuilder = rep.filter_block();
            if !filter_block_ptr.is_null() {
                trace!(
                    "TableBuilder::drop: deleting FilterBlockBuilder @ {:?}",
                    filter_block_ptr
                );
                let _fb: Box<FilterBlockBuilder> = Box::from_raw(filter_block_ptr);
                rep.set_filter_block(core::ptr::null_mut());
            }

            let _rep_box: Box<TableBuilderRep> = Box::from_raw(rep_ptr);
            self.set_rep_ptr(core::ptr::null_mut());
        }
    }
}

#[cfg(test)]
mod table_builder_drop_semantics_tests {
    use super::*;

    #[traced_test]
    fn drop_with_null_rep_pointer_is_noop() {
        trace!("drop_with_null_rep_pointer_is_noop: constructing dummy TableBuilder with null rep");

        let mut builder = TableBuilder::invalid();

        trace!("dropping TableBuilder with null rep pointer; should be a no-op");
        drop(builder);
    }

    #[traced_test]
    fn drop_after_abandon_respects_closed_invariant() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("drop_after_abandon_respects_closed_invariant");

        trace!("drop_after_abandon_respects_closed_invariant: calling abandon to mark closed");
        builder.abandon();

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;
            assert!(
                *rep.closed(),
                "builder must be marked closed before Drop to satisfy invariant"
            );
        }

        trace!("dropping TableBuilder after abandon");
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
