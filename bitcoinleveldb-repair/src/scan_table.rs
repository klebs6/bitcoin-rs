// ---------------- [ File: bitcoinleveldb-repair/src/scan_table.rs ]
crate::ix!();

impl Repairer {

    pub fn scan_table(&mut self, number: u64) {
        use std::ptr;

        trace!(table_no = number, "Repairer::scan_table: start");

        let mut t = RepairerTableInfo {
            meta: FileMetaData::default(),
            max_sequence: 0,
        };
        t.meta.set_number(number);

        let mut fname = table_file_name(&self.dbname, number);

        let mut status = self
            .env
            .get_file_size(&fname, t.meta.file_size_mut() as *mut u64);

        if !status.is_ok() {
            // Try alternate file name.
            fname = sst_table_file_name(&self.dbname, number);

            let s2 = self
                .env
                .get_file_size(&fname, t.meta.file_size_mut() as *mut u64);

            if s2.is_ok() {
                status = crate::Status::ok();
            }
        }

        if !status.is_ok() {
            self.archive_file(&table_file_name(&self.dbname, number));
            self.archive_file(&sst_table_file_name(&self.dbname, number));

            warn!(
                table_no = number,
                status = %status.to_string(),
                "Repairer::scan_table: dropped"
            );
            return;
        }

        // Extract metadata by scanning through table.
        let mut counter: i32 = 0;

        let iter = self.new_table_iterator(&t.meta);

        let mut empty = true;
        let mut parsed = ParsedInternalKey::default();
        t.max_sequence = 0;

        unsafe {
            (*iter).seek_to_first();

            while (*iter).valid() {
                let key = (*iter).key();

                if !parse_internal_key(&key, &mut parsed as *mut ParsedInternalKey) {
                    let escaped = escape_for_debug(slice_as_bytes(&key));
                    warn!(
                        table_no = number,
                        key = %escaped,
                        "Repairer::scan_table: unparsable key"
                    );
                    (*iter).next();
                    continue;
                }

                counter += 1;

                if empty {
                    empty = false;
                    let _ = t.meta.smallest_mut().decode_from(&key);
                }

                let _ = t.meta.largest_mut().decode_from(&key);

                if *parsed.sequence() > t.max_sequence {
                    t.max_sequence = *parsed.sequence();
                }

                (*iter).next();
            }

            let it_status = (*iter).status();
            if !it_status.is_ok() {
                status = it_status;
            }

            drop(Box::from_raw(iter));
        }

        info!(
            table_no = number,
            entries = counter,
            status = %status.to_string(),
            "Repairer::scan_table: entries scanned"
        );

        if status.is_ok() {
            self.tables.push(t);
        } else {
            // RepairTable archives input file.
            self.repair_table(&fname, t);
        }

        trace!(table_no = number, "Repairer::scan_table: done");
    }
}
