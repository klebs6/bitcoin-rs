// ---------------- [ File: bitcoinleveldb-table/src/table_new_iterator.rs ]
crate::ix!();

impl Table {

    /// Returns a new iterator over the table contents.
    /// 
    /// The result of new_iterator() is initially invalid (caller must call one
    /// of the Seek methods on the iterator before using it).
    ///
    pub fn new_iterator(&self, options: &ReadOptions) -> *mut LevelDBIterator {
        unsafe {
            let rep_ptr = (*(self as *const Table as *mut Table)).rep_mut_ptr();
            assert!(
                !rep_ptr.is_null(),
                "Table::new_iterator: rep pointer is null"
            );

            let rep: &mut TableRep = &mut *rep_ptr;

            let index_block_ptr_ref = rep.index_block();
            let index_block_ptr: *mut Block = *index_block_ptr_ref;

            assert!(
                !index_block_ptr.is_null(),
                "Table::new_iterator: index_block pointer is null"
            );

            let index_block_ref: &mut Block = &mut *index_block_ptr;

            let cmp_arc: &Arc<dyn SliceComparator> = rep.options().comparator();
            let cmp_ptr: *const dyn SliceComparator = &**cmp_arc;

            assert!(
                !cmp_ptr.is_null(),
                "Table::new_iterator: comparator pointer is null"
            );

            let index_iter_raw: *mut LevelDBIterator =
                index_block_ref.new_iterator(cmp_ptr);

            trace!(
                "Table::new_iterator: index iterator created @ {:?}",
                index_iter_raw
            );

            // Take ownership of the raw LevelDBIterator as a Box<LevelDBIterator>,
            // then upcast to the abstract iterator interface.
            let index_iter_box: Box<LevelDBIterator> = Box::from_raw(index_iter_raw);
            let index_iter_iface: Box<dyn LevelDBIteratorInterface> = index_iter_box;

            let table_ptr = self as *const Table as *mut c_void;

            // This mirrors the original LevelDB call:
            // NewTwoLevelIterator(index_iter, &Table::BlockReader, this, options)
            let two_level_iface: Box<dyn LevelDBIteratorInterface> =
                bitcoinleveldb_duplex::new_two_level_iterator(
                    index_iter_iface,
                    Table::block_reader,
                    table_ptr,
                    options,
                );

            trace!(
                "Table::new_iterator: two-level iterator created"
            );

            // Wrap the resulting TwoLevelIterator interface in a LevelDBIterator
            // wrapper, as everywhere else in this port.
            let wrapper = LevelDBIterator::new(Some(two_level_iface));
            let boxed_wrapper = Box::new(wrapper);
            let raw_wrapper: *mut LevelDBIterator = Box::into_raw(boxed_wrapper);

            trace!(
                "Table::new_iterator: returning LevelDBIterator wrapper @ {:?}",
                raw_wrapper
            );

            raw_wrapper
        }
    }
}

#[cfg(test)]
mod table_new_iterator_invariants {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DummyRandomAccessFile(new_iterator)".to_string())
        }
    }

    impl RandomAccessFileRead for DummyRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "DummyRandomAccessFile::read(new_iterator): offset={}, n={}, scratch={:?}",
                offset,
                n,
                scratch
            );
            unsafe {
                *result = Slice::default();
            }
            Status::ok()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[test]
    #[should_panic(expected = "Table::new_iterator: rep pointer is null")]
    fn new_iterator_panics_when_rep_pointer_is_null() {
        let table = Table::new(std::ptr::null_mut());
        let options = ReadOptions::default();

        trace!(
            "new_iterator_panics_when_rep_pointer_is_null: calling new_iterator on Table with null rep"
        );

        let _ = table.new_iterator(&options);
    }

    #[test]
    #[should_panic(expected = "Table::new_iterator: index_block pointer is null")]
    fn new_iterator_panics_when_index_block_pointer_is_null() {
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile));

        let options_for_rep = Options::default();
        let cache_id = 0_u64;
        let metaindex_handle = BlockHandle::default();
        let index_block_ptr: *mut Block = core::ptr::null_mut();

        let rep = TableRep::new(
            options_for_rep,
            file.clone(),
            cache_id,
            metaindex_handle,
            index_block_ptr,
        );
        let rep_box = Box::new(rep);
        let rep_ptr: *mut TableRep = Box::into_raw(rep_box);

        let table = Table::new(rep_ptr);
        let read_opts = ReadOptions::default();

        trace!(
            "new_iterator_panics_when_index_block_pointer_is_null: calling new_iterator on Table with null index_block"
        );

        let _ = table.new_iterator(&read_opts);
    }
}
