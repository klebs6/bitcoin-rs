// ---------------- [ File: bitcoinleveldb-table/src/table_read_filter.rs ]
crate::ix!();

impl Table {

    pub fn read_filter(&mut self, filter_handle_value: &Slice) {
        unsafe {
            if self.rep.is_null() {
                debug!(
                    "Table::read_filter: rep pointer is null; skipping filter load"
                );
                return;
            }

            let rep = &mut *(self.rep as *mut TableRep);

            if rep.options.filter_policy.is_null() {
                trace!(
                    "Table::read_filter: options.filter_policy is null; no filter will be loaded"
                );
                return;
            }

            let mut v = *filter_handle_value;
            let mut handle = BlockHandle::default();
            let decode_status = handle.decode_from(&mut v as *mut Slice);

            if !decode_status.is_ok() {
                debug!(
                    "Table::read_filter: failed to decode filter BlockHandle; status is not OK"
                );
                return;
            }

            // We might want to unify with ReadBlock() if we start
            // requiring checksum verification in Table::Open.
            let mut opt = ReadOptions::default();

            if rep.options.paranoid_checks {
                opt.verify_checksums = true;
            }

            let mut block = BlockContents {
                data:           Slice::default(),
                cachable:       false,
                heap_allocated: false,
            };

            trace!(
                "Table::read_filter: reading filter block at offset={}, size={}",
                handle.offset(),
                handle.size()
            );

            let s = read_block(
                rep.file.clone(),
                &opt,
                &handle,
                &mut block as *mut BlockContents,
            );

            if !s.is_ok() {
                warn!(
                    "Table::read_filter: ReadBlock(filter) returned non-OK; filter will be disabled"
                );
                return;
            }

            if block.heap_allocated {
                // Will need to delete later
                rep.filter_data = block.data.data() as *mut u8;
                rep.filter_data_len = block.data.size();
                trace!(
                    "Table::read_filter: filter data heap_allocated; ptr={:?}, len={}",
                    rep.filter_data,
                    rep.filter_data_len
                );
            } else {
                rep.filter_data = core::ptr::null_mut();
                rep.filter_data_len = 0;
                trace!(
                    "Table::read_filter: filter data not heap_allocated; assuming external lifetime"
                );
            }

            let policy = &*rep.options.filter_policy;
            let filter_reader = FilterBlockReader::new(policy, &block.data);
            rep.filter = Box::into_raw(Box::new(filter_reader));

            trace!(
                "Table::read_filter: FilterBlockReader created @ {:?}",
                rep.filter
            );
        }
    }
}
