// ---------------- [ File: bitcoinleveldb-table/src/table_constructor.rs ]
crate::ix!();

pub struct TableConstructor {
    base:   Constructor,
    source: *mut StringSource,
    table:  *mut Table,
}

impl Drop for TableConstructor {
    fn drop(&mut self) {
        trace!(
            "TableConstructor::drop: dropping with table={:?}, source={:?}",
            self.table,
            self.source
        );
        self.reset();
    }
}

impl TableConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
        trace!(
            "TableConstructor::new: creating constructor with custom comparator"
        );
        TableConstructor {
            base:   Constructor::new(cmp),
            source: core::ptr::null_mut(),
            table:  core::ptr::null_mut(),
        }
    }
   
    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        unsafe {
            assert!(
                !self.table.is_null(),
                "TableConstructor::new_iterator: table pointer is null"
            );

            let table_ref = &mut *self.table;
            let read_options = ReadOptions::default();

            trace!(
                "TableConstructor::new_iterator: creating iterator for table @ {:?}",
                self.table
            );

            table_ref.new_iterator(&read_options)
        }
    }

    pub fn approximate_offset_of(&self, key_: &Slice) -> u64 {
        unsafe {
            assert!(
                !self.table.is_null(),
                "TableConstructor::approximate_offset_of: table pointer is null"
            );

            let table_ref = &*self.table;
            let off = table_ref.approximate_offset_of(key_);

            trace!(
                "TableConstructor::approximate_offset_of: key_offset={}",
                off
            );

            off
        }
    }
   
    pub fn reset(&mut self) {
        unsafe {
            if !self.table.is_null() {
                trace!(
                    "TableConstructor::reset: deleting Table @ {:?}",
                    self.table
                );
                let _tbl_box: Box<Table> =
                    Box::from_raw(self.table);
                self.table = core::ptr::null_mut();
            } else {
                trace!(
                    "TableConstructor::reset: table pointer is null; nothing to delete"
                );
            }

            if !self.source.is_null() {
                trace!(
                    "TableConstructor::reset: deleting StringSource @ {:?}",
                    self.source
                );
                let _src_box: Box<StringSource> =
                    Box::from_raw(self.source);
                self.source = core::ptr::null_mut();
            } else {
                trace!(
                    "TableConstructor::reset: source pointer is null; nothing to delete"
                );
            }
        }
    }
}
