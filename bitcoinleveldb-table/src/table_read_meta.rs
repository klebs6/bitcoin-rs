// ---------------- [ File: bitcoinleveldb-table/src/table_read_meta.rs ]
crate::ix!();

impl Table {

    pub fn read_meta(&mut self, footer: &Footer) {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if rep_ptr.is_null() {
                // Do not need any metadata
                debug!(
                    "Table::read_meta: rep pointer is null; skipping metaindex load"
                );
                return;
            }

            let rep: &mut TableRep = &mut *rep_ptr;

            // Presence of a FilterPolicy implies we may have a filter block.
            let _policy_box: &Arc<dyn FilterPolicy> =
                rep.options().filter_policy();

            trace!(
                "Table::read_meta: filter_policy present; proceeding to read metaindex"
            );

            let mut opt = ReadOptions::default();

            if *rep.options().paranoid_checks() {
                *opt.verify_checksums_mut() = true;
            }

            // Use public default instead of private-field literal.
            let mut contents = BlockContents::default();

            trace!(
                "Table::read_meta: reading metaindex block at offset={}, size={}",
                footer.metaindex_handle().offset(),
                footer.metaindex_handle().size()
            );

            // Do not propagate errors since meta info is not needed for operation
            let s = read_block(
                rep.file().clone(),
                &opt,
                footer.metaindex_handle(),
                &mut contents as *mut BlockContents,
            );

            if !s.is_ok() {
                warn!(
                    "Table::read_meta: ReadBlock(metaindex) returned non-OK; metadata filters disabled"
                );
                return;
            }

            let meta_block = Box::new(Block::new(&contents));

            let meta_block_ptr: *mut Block = Box::into_raw(meta_block);

            let base_cmp = &*bitcoinleveldb_comparator::bytewise_comparator();

            let iter_ptr: *mut LevelDBIterator =
                (*meta_block_ptr).new_iterator(base_cmp);
            trace!(
                "Table::read_meta: metaindex iterator created @ {:?}",
                iter_ptr
            );

            let mut key = String::from("filter.");
            let policy_name = {
                let policy_box = rep.options().filter_policy();
                policy_box.name()
            };
            key.push_str(policy_name.as_ref());

            let key_slice = Slice::from(key.as_bytes());

            (*iter_ptr).seek(&key_slice);

            if (*iter_ptr).valid() {
                let current_key = (*iter_ptr).key();
                if current_key == key_slice {
                    let value = (*iter_ptr).value();
                    trace!(
                        "Table::read_meta: located filter block entry with key='{}'",
                        key
                    );
                    self.read_filter(&value);
                } else {
                    trace!(
                        "Table::read_meta: iterator valid but key mismatch; expected='{}'",
                        key
                    );
                }
            } else {
                trace!(
                    "Table::read_meta: metaindex iterator not valid after seek; no filter entry"
                );
            }

            let it_status = (*iter_ptr).status();
            if !it_status.is_ok() {
                warn!(
                    "Table::read_meta: metaindex iterator reported non-OK status; ignoring"
                );
            }

            trace!(
                "Table::read_meta: deleting metaindex iterator @ {:?}",
                iter_ptr
            );
            drop(Box::from_raw(iter_ptr));

            trace!(
                "Table::read_meta: deleting metaindex block @ {:?}",
                meta_block_ptr
            );
            drop(Box::from_raw(meta_block_ptr));
        }
    }
}

#[cfg(test)]
mod table_read_meta_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct DummyRandomAccessFile {
        name: &'static str,
        status_to_return: Status,
    }

    impl DummyRandomAccessFile {
        fn new_error(status: Status) -> Self {
            DummyRandomAccessFile {
                name: "DummyRandomAccessFile(read_meta)",
                status_to_return: status,
            }
        }
    }

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(self.name)
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
                "DummyRandomAccessFile::read(read_meta): offset={}, n={}, scratch={:?}",
                offset,
                n,
                scratch
            );
            unsafe {
                *result = Slice::default();
            }
            self.status_to_return.clone()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[traced_test]
    fn read_meta_is_noop_when_table_has_null_rep_pointer() {
        let mut table = Table::new(std::ptr::null_mut());
        let footer = Footer::default();

        trace!(
            "read_meta_is_noop_when_table_has_null_rep_pointer: calling read_meta on Table with null rep"
        );

        table.read_meta(&footer);

        // As with read_filter, the essential guarantee here is that nothing
        // panics when the Table has no backing TableRep.
    }

    #[traced_test]
    fn read_meta_gracefully_handles_metaindex_read_error() {
        let msg = b"forced metaindex read error";
        let msg_slice = Slice::from(&msg[..]);
        let error_status = Status::corruption(&msg_slice, None);

        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile::new_error(
                error_status.clone(),
            )));

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

        let mut table = Table::new(rep_ptr);
        let footer = Footer::default();

        trace!(
            "read_meta_gracefully_handles_metaindex_read_error: calling read_meta with forced read error"
        );

        table.read_meta(&footer);

        unsafe {
            let rep_after: &TableRep = &*table.rep_mut_ptr();
            trace!(
                "read_meta_gracefully_handles_metaindex_read_error: filter_data_ptr={:?}, filter_data_len={}, filter_present={}",
                rep_after.filter_data(),
                rep_after.filter_data_len(),
                rep_after.filter().is_some()
            );
            assert!(
                rep_after.filter().is_none(),
                "filter must remain None when metaindex block cannot be read"
            );
            assert_eq!(
                rep_after.filter_data_len(),
                &0usize,
                "filter_data_len must remain zero when metaindex read fails"
            );
        }

        drop(table);
    }
}
