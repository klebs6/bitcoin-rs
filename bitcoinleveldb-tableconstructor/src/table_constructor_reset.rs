// ---------------- [ File: bitcoinleveldb-tableconstructor/src/table_constructor_reset.rs ]
crate::ix!();

impl TableConstructor {

    /// Helper: clear the owned Table pointer if present.
    fn reset_table_ptr(&mut self) {
        unsafe {
            if !self.table().is_null() {
                trace!(
                    "TableConstructor::reset_table_ptr: deleting Table @ {:?}",
                    self.table()
                );
                let _tbl_box: Box<Table> = Box::from_raw(*self.table_mut());
                self.set_table(core::ptr::null_mut());
            } else {
                trace!(
                    "TableConstructor::reset_table_ptr: table pointer is null; nothing to delete"
                );
            }
        }
    }

    /// Helper: clear the non‑owning StringSource pointer.
    ///
    /// The underlying StringSource is owned via the Rc<RefCell<...>>
    /// kept inside the TableRep, so we must **not** free it here.
    fn reset_source_ptr(&mut self) {
        unsafe {
            if !self.source().is_null() {
                trace!(
                    "TableConstructor::reset_source_ptr: clearing non‑owning StringSource pointer @ {:?}",
                    self.source()
                );
                self.set_source(core::ptr::null_mut());
            } else {
                trace!(
                    "TableConstructor::reset_source_ptr: source pointer is null; nothing to clear"
                );
            }
        }
    }

    pub fn reset(&mut self) {
        trace!("TableConstructor::reset: begin");
        self.reset_table_ptr();
        self.reset_source_ptr();
    }
}
