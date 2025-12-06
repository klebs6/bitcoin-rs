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

            let rep = &mut *rep_ptr;

            if rep.options().filter_policy().is_null() {
                trace!(
                    "Table::read_meta: no filter_policy configured; skipping metaindex"
                );
                return;
            }

            // TODO(sanjay): Skip this if footer.metaindex_handle() size indicates
            // it is an empty block.
            let mut opt = ReadOptions::default();

            if *rep.options().paranoid_checks() {
                *opt.verify_checksums_mut() = true;
            }

            let mut contents = BlockContents {
                data:           Slice::default(),
                cachable:       false,
                heap_allocated: false,
            };

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

            let iter_ptr = (*meta_block_ptr).new_iterator(base_cmp);
            trace!(
                "Table::read_meta: metaindex iterator created @ {:?}",
                iter_ptr
            );

            let mut key = String::from("filter.");
            let policy = &*rep.options().filter_policy();
            let policy_name = policy.name();
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

            drop(Box::from_raw(iter_ptr));
            drop(Box::from_raw(meta_block_ptr));
        }
    }
}
