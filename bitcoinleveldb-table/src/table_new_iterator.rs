// ---------------- [ File: bitcoinleveldb-table/src/table_new_iterator.rs ]
crate::ix!();

impl Table {
    
    /**
      | Returns a new iterator over the table
      | contents.
      |
      | The result of NewIterator() is initially
      | invalid (caller must call one of the Seek
      | methods on the iterator before using it).
      */
    pub fn new_iterator(&self, options: &ReadOptions) -> *mut LevelDBIterator {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            assert!(
                !rep_ptr.is_null(),
                "Table::new_iterator: rep pointer is null"
            );

            let rep = &mut *rep_ptr;

            assert!(
                !rep.index_block().is_null(),
                "Table::new_iterator: index_block pointer is null"
            );

            let index_block = rep.index_block_mut();

            let cmp_ptr = rep.options().comparator();
            assert!(
                !cmp_ptr.is_null(),
                "Table::new_iterator: comparator pointer is null"
            );

            let index_iter = index_block.new_iterator(&*cmp_ptr);

            trace!(
                "Table::new_iterator: index iterator created @ {:?}",
                index_iter
            );

            let table_ptr = self as *const Table as *mut c_void;

            bitcoinleveldb_duplex::new_two_level_iterator(
                index_iter,
                Table::block_reader as fn(*mut c_void, &ReadOptions, &Slice) -> *mut LevelDBIterator,
                table_ptr,
                *options,
            )
        }
    }
}
