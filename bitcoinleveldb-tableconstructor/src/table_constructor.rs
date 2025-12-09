// ---------------- [ File: bitcoinleveldb-tableconstructor/src/table_constructor.rs ]
crate::ix!();

#[derive(Getters,Setters,MutGetters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct TableConstructor {
    base:   Constructor,
    // Non‑owning pointer into the in‑memory table data source.
    // Lifetime is tied to the RandomAccessFile stored inside the Table.
    source: *mut StringSource,
    // Owning pointer to the opened Table (allocated in Table::open).
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
}

#[cfg(test)]
mod table_constructor_lifecycle_behavior {
    use super::*;

    #[traced_test]
    fn new_initializes_null_pointers() {
        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
        let ctor = TableConstructor::new(cmp);

        assert!(
            ctor.table().is_null(),
            "table pointer must start null"
        );
        assert!(
            ctor.source().is_null(),
            "source pointer must start null"
        );
    }

    #[traced_test]
    fn reset_clears_table_and_source_pointers() {
        let mut ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );

        unsafe {
            // Install a dummy Table so we exercise the owned‑pointer branch.
            let tbl_box: Box<Table> =
                Box::new(Table::new(core::ptr::null_mut()));
            let raw_tbl: *mut Table = Box::into_raw(tbl_box);
            ctor.set_table(raw_tbl);

            // Source is non‑owning; we can use any non‑null address for testing.
            ctor.set_source(0x1 as *mut StringSource);
        }

        assert!(!ctor.table().is_null());
        assert!(!ctor.source().is_null());

        ctor.reset();

        assert!(ctor.table().is_null());
        assert!(ctor.source().is_null());
    }

    #[test]
    #[should_panic(expected = "TableConstructor::new_iterator: table pointer is null")]
    fn new_iterator_panics_when_table_is_null() {
        let ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );
        let _ = ctor.new_iterator();
    }

    #[test]
    #[should_panic(expected = "TableConstructor::approximate_offset_of: table pointer is null")]
    fn approximate_offset_of_panics_when_table_is_null() {
        let ctor = TableConstructor::new(
            Box::new(BytewiseComparatorImpl::default()),
        );
        let key = Slice::from(b"some-key".as_ref());
        let _ = ctor.approximate_offset_of(&key);
    }
}
