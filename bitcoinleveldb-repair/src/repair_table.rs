// ---------------- [ File: bitcoinleveldb-repair/src/repair_table.rs ]
crate::ix!();

impl Repairer {

    pub fn repair_table(&mut self, src: &str, t: RepairerTableInfo) {
        use std::ptr;

        trace!(
            src = %src,
            table_no = *t.meta.number(),
            "Repairer::repair_table: start"
        );

        // We will copy src contents to a new table and then rename the
        // new table over the source.

        let mut t = t;

        // Create builder.
        let copy = table_file_name(&self.dbname, self.next_file_number);
        self.next_file_number = self.next_file_number.wrapping_add(1);

        let mut file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();
        let mut s = self.env.new_writable_file(&copy, &mut file_ptr);

        if !s.is_ok() {
            debug!(
                file = %copy,
                status = %s.to_string(),
                "Repairer::repair_table: NewWritableFile failed"
            );
            return;
        }

        {
            let mut file_holder: Box<Box<dyn WritableFile>> = unsafe {
                assert!(
                    !file_ptr.is_null(),
                    "Repairer::repair_table: env returned null WritableFile"
                );
                Box::from_raw(file_ptr)
            };

            let mut builder: *mut TableBuilder =
                Box::into_raw(Box::new(TableBuilder::new(&self.options, file_holder.as_mut())));

            // Copy data.
            let iter = self.new_table_iterator(&t.meta);
            let mut counter: i32 = 0;

            unsafe {
                (*iter).seek_to_first();
                while (*iter).valid() {
                    (*builder).add(&(*iter).key(), &(*iter).value());
                    counter += 1;
                    (*iter).next();
                }
                drop(Box::from_raw(iter));
            }

            self.archive_file(src);

            if counter == 0 {
                // Nothing to save
                unsafe { (*builder).abandon() };
            } else {
                s = unsafe { (*builder).finish() };
                if s.is_ok() {
                    let sz = unsafe { (*builder).file_size() };
                    t.meta.set_file_size(sz);
                }
            }

            unsafe {
                drop(Box::from_raw(builder));
            }
            builder = ptr::null_mut();

            if s.is_ok() {
                s = file_holder.as_mut().close();
            }

            // `file_holder` drops here, mirroring `delete file;`.
        }

        if counter > 0 && s.is_ok() {
            let orig = table_file_name(&self.dbname, *t.meta.number());
            s = self.env.rename_file(&copy, &orig);
            if s.is_ok() {
                info!(
                    table_no = *t.meta.number(),
                    entries = counter,
                    "Repairer::repair_table: entries repaired"
                );
                self.tables.push(t);
            }
        }

        if !s.is_ok() {
            let s_del = self.env.delete_file(&copy);
            debug!(
                file = %copy,
                ok = s_del.is_ok(),
                status = %s_del.to_string(),
                "Repairer::repair_table: cleanup delete copy"
            );
        }

        trace!(
            src = %src,
            table_no = *t.meta.number(),
            status = %s.to_string(),
            "Repairer::repair_table: done"
        );
    }
}

#[cfg(test)]
mod repair_table_recovery_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn scan_table_triggers_repair_table_and_archives_source_for_invalid_table() {
        let db = EphemeralDbDir::new("repair-table-via-scan");
        let dbname: String = db.path_string();

        let table_no: u64 = 10;
        let table_path = table_file_name(&dbname, table_no);
        touch_file(&table_path);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(table_no, table_path = %table_path, "calling scan_table to trigger repair_table path");
        repairer.scan_table(table_no);

        // Regardless of how repair proceeds, scan_table->repair_table archives input file on failure.
        let _dst = assert_archived(&table_path);
    }
}
