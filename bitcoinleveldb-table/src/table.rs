// ---------------- [ File: bitcoinleveldb-table/src/table.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table.cc]

/**
  | A Table is a sorted map from strings to
  | strings.  Tables are immutable and persistent.
  | A Table may be safely accessed from multiple
  | threads without external synchronization.
  */
pub struct Table {
    rep: *const TableRep,
}

impl Table {

    #[inline]
    pub fn rep_ptr(&self) -> *const TableRep {
        trace!(
            "Table::rep_ptr: returning rep pointer {:?}",
            self.rep
        );
        self.rep
    }

    #[inline]
    pub fn rep_mut_ptr(&mut self) -> *mut TableRep {
        trace!(
            "Table::rep_mut_ptr: returning mutable rep pointer {:?}",
            self.rep as *mut TableRep
        );
        self.rep as *mut TableRep
    }

    pub fn new(rep: *mut TableRep) -> Self {
        unsafe {
            if rep.is_null() {
                trace!(
                    "Table::new: constructed with null TableRep pointer; table is empty shell"
                );
            } else {
                trace!(
                    "Table::new: constructed with TableRep @ {:?}",
                    rep
                );
            }
        }

        Table {
            rep: rep as *const TableRep,
        }
    }
}

impl Drop for Table {

    fn drop(&mut self) {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if !rep_ptr.is_null() {
                trace!(
                    "Table::drop: deleting TableRep @ {:?}",
                    rep_ptr
                );
                let _rep_box: Box<TableRep> = Box::from_raw(rep_ptr);
            } else {
                trace!("Table::drop: rep pointer is null; nothing to free");
            }
        }
    }
}

#[cfg(test)]
mod table_lifecycle_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DummyRandomAccessFile(table_lifecycle)".to_string())
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
                "DummyRandomAccessFile::read(table_lifecycle): offset={}, n={}, scratch={:?}",
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

    #[traced_test]
    fn table_created_with_null_rep_exposes_null_pointers_and_is_safe_to_drop() {
        trace!(
            "table_created_with_null_rep_exposes_null_pointers_and_is_safe_to_drop: start"
        );

        let mut table = Table::new(std::ptr::null_mut());

        let rep_ptr = table.rep_ptr();
        let rep_mut_ptr = table.rep_mut_ptr();

        trace!(
            "table_created_with_null_rep_exposes_null_pointers_and_is_safe_to_drop: rep_ptr={:?}, rep_mut_ptr={:?}",
            rep_ptr,
            rep_mut_ptr
        );

        assert!(
            rep_ptr.is_null(),
            "rep_ptr should be null when constructed with a null TableRep pointer"
        );
        assert!(
            rep_mut_ptr.is_null(),
            "rep_mut_ptr should be null when constructed with a null TableRep pointer"
        );

        drop(table);
    }

    #[traced_test]
    fn table_created_with_valid_rep_exposes_same_pointer_through_accessors() {
        trace!(
            "table_created_with_valid_rep_exposes_same_pointer_through_accessors: start"
        );

        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile));

        let options = Options::default();
        let cache_id = 0_u64;
        let metaindex_handle = BlockHandle::default();

        let contents = BlockContents::default();
        let index_block_box = Box::new(Block::new(&contents));
        let index_block_ptr: *mut Block = Box::into_raw(index_block_box);

        let rep = TableRep::new(
            options,
            file.clone(),
            cache_id,
            metaindex_handle,
            index_block_ptr,
        );

        let rep_box = Box::new(rep);
        let rep_ptr: *mut TableRep = Box::into_raw(rep_box);

        let mut table = Table::new(rep_ptr);

        let rep_seen_const = table.rep_ptr();
        let rep_seen_mut = table.rep_mut_ptr();

        trace!(
            "table_created_with_valid_rep_exposes_same_pointer_through_accessors: rep_ptr={:?}, rep_seen_const={:?}, rep_seen_mut={:?}",
            rep_ptr,
            rep_seen_const,
            rep_seen_mut
        );

        assert_eq!(
            rep_seen_const,
            rep_ptr as *const TableRep,
            "rep_ptr accessor must return the same pointer used at construction"
        );
        assert_eq!(
            rep_seen_mut,
            rep_ptr,
            "rep_mut_ptr accessor must return the same pointer used at construction"
        );

        drop(table);
    }
}
