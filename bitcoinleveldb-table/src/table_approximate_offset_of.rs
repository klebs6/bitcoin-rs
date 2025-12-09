// ---------------- [ File: bitcoinleveldb-table/src/table_approximate_offset_of.rs ]
crate::ix!();

impl Table {

    /// Given a key, return an approximate byte
    /// offset in the file where the data for that
    /// key begins (or would begin if the key were
    /// present in the file).  The returned value is
    /// in terms of file bytes, and so includes
    /// effects like compression of the underlying
    /// data.
    /// 
    /// E.g., the approximate offset of the last key
    /// in the table will be close to the file
    /// length.
    pub fn approximate_offset_of(&self, key_: &Slice) -> u64 {
        unsafe {
            let rep_ptr = (*(self as *const Table as *mut Table)).rep_mut_ptr();
            if rep_ptr.is_null() {
                debug!(
                    "Table::approximate_offset_of: rep pointer is null; returning offset 0"
                );
                return 0;
            }

            let rep: &mut TableRep = &mut *rep_ptr;

            // index_block_: Block*
            let index_block_ptr_ref = rep.index_block();
            let index_block_ptr: *mut Block = *index_block_ptr_ref;

            assert!(
                !index_block_ptr.is_null(),
                "Table::approximate_offset_of: index_block pointer is null"
            );

            let index_block_ref: &mut Block = &mut *index_block_ptr;

            let cmp_arc: &Arc<dyn SliceComparator> = rep.options().comparator();
            let cmp_ptr: *const dyn SliceComparator = &**cmp_arc;

            assert!(
                !cmp_ptr.is_null(),
                "Table::approximate_offset_of: comparator pointer is null"
            );

            // Create iterator over index block
            let index_iter: *mut LevelDBIterator =
                index_block_ref.new_iterator(cmp_ptr);

            trace!(
                "Table::approximate_offset_of: index iterator created @ {:?}",
                index_iter
            );

            // Use the low‑level iterator API implemented for the raw
            // LevelDBIterator handle.  The traits are imported via
            // bitcoinleveldb_iterator::* in imports.rs.
            (*index_iter).seek(key_);

            let result = if (*index_iter).valid() {
                let mut handle = BlockHandle::default();
                let mut input = (*index_iter).value();

                let s = handle.decode_from(&mut input as *mut Slice);
                if s.is_ok() {
                    let off = handle.offset();
                    trace!(
                        "Table::approximate_offset_of: decoded handle; offset={}",
                        off
                    );
                    off
                } else {
                    let off = rep.metaindex_handle().offset();
                    trace!(
                        "Table::approximate_offset_of: failed to decode handle; using metaindex_handle offset={}",
                        off
                    );
                    off
                }
            } else {
                let off = rep.metaindex_handle().offset();
                trace!(
                    "Table::approximate_offset_of: key past last; using metaindex_handle offset={}",
                    off
                );
                off
            };

            // Iterator is heap‑allocated on the C++ side; free with Box::from_raw.
            trace!(
                "Table::approximate_offset_of: deleting index iterator @ {:?}",
                index_iter
            );
            drop(Box::from_raw(index_iter));

            result
        }
    }
}

#[cfg(test)]
mod table_approximate_offset_of_behavior {
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
            Cow::Owned("DummyRandomAccessFile(approximate_offset_of)".to_string())
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
                "DummyRandomAccessFile::read(approximate_offset_of): offset={}, n={}, scratch={:?}",
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
    fn approximate_offset_of_returns_zero_for_table_without_rep() {
        let table = Table::new(std::ptr::null_mut());

        let key_bytes = b"some-key".to_vec();
        let key = Slice::from(key_bytes.as_slice());

        trace!(
            "approximate_offset_of_returns_zero_for_table_without_rep: invoking approximate_offset_of for key_len={}",
            *key.size()
        );

        let offset = table.approximate_offset_of(&key);
        trace!(
            "approximate_offset_of_returns_zero_for_table_without_rep: observed_offset={}",
            offset
        );

        assert_eq!(
            offset, 0,
            "approximate_offset_of should return 0 when the Table has no TableRep"
        );
    }

    #[test]
    #[should_panic(expected = "Table::approximate_offset_of: index_block pointer is null")]
    fn approximate_offset_of_panics_when_index_block_pointer_is_null() {
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile));

        let options = Options::default();
        let cache_id = 0_u64;
        let metaindex_handle = BlockHandle::default();
        let index_block_ptr: *mut Block = core::ptr::null_mut();

        let rep = TableRep::new(
            options,
            file.clone(),
            cache_id,
            metaindex_handle,
            index_block_ptr,
        );

        let rep_box = Box::new(rep);
        let rep_ptr: *mut TableRep = Box::into_raw(rep_box);

        let table = Table::new(rep_ptr);

        let key_bytes = b"panic-key".to_vec();
        let key = Slice::from(key_bytes.as_slice());

        trace!(
            "approximate_offset_of_panics_when_index_block_pointer_is_null: calling approximate_offset_of"
        );

        let _ = table.approximate_offset_of(&key);
    }
}
