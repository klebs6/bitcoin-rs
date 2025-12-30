// ---------------- [ File: bitcoinleveldb-repair/src/convert_log_to_table.rs ]
crate::ix!();

impl Repairer {

    pub fn convert_log_to_table(&mut self, log: u64) -> crate::Status {
        use std::ptr;

        trace!(lognum = log, "Repairer::convert_log_to_table: start");

        // Open the log file
        let logname = log_file_name(&self.dbname, log);

        let mut lfile_ptr: *mut Box<dyn SequentialFile> = ptr::null_mut();
        let mut status = self.env.new_sequential_file(&logname, &mut lfile_ptr);

        if !status.is_ok() {
            debug!(
                lognum = log,
                file = %logname,
                status = %status.to_string(),
                "Repairer::convert_log_to_table: NewSequentialFile failed"
            );
            return status;
        }

        {
            let mut lfile_holder: Box<Box<dyn SequentialFile>> = unsafe {
                assert!(
                    !lfile_ptr.is_null(),
                    "Repairer::convert_log_to_table: env returned null SequentialFile"
                );
                Box::from_raw(lfile_ptr)
            };

            // Create the log reader.
            let mut reporter = RepairLogReporter {
                info_log: *self.options.info_log(),
                lognum:   log,
            };

            // We intentionally make LogReader do checksumming so that
            // corruptions cause entire commits to be skipped instead of
            // propagating bad information (like overly large sequence
            // numbers).
            //
            // NOTE: We preserve the original argument/comment pairing from the
            // upstream source: `false /*do not checksum*/`.
            let mut reader = LogReader::new(
                lfile_holder.as_mut(),
                &mut reporter,
                false, // do not checksum
                0,     // initial_offset
            );

            // Read all the records and add to a memtable
            let mut scratch = String::new();
            let mut record = Slice::default();

            let mut batch = WriteBatch::default();

            let mut mem: *mut MemTable = Box::into_raw(Box::new(MemTable::new(&self.icmp)));
            unsafe {
                (*mem).ref_();
            }

            let mut counter: i32 = 0;

            while reader.read_record(&mut record, &mut scratch) {
                if *record.size() < 12 {
                    let msg = Slice::from(&b"log record too small"[..]);
                    let msg2 = Slice::from(logname.as_bytes());
                    let s = crate::Status::corruption(&msg, Some(&msg2));
                    reporter.corruption(*record.size(), &s);
                    continue;
                }

                write_batch_internal::set_contents(&mut batch, &record);
                status = write_batch_internal::insert_into(&batch, mem);

                if status.is_ok() {
                    counter += write_batch_internal::count(&batch);
                } else {
                    warn!(
                        lognum = log,
                        status = %status.to_string(),
                        "Repairer::convert_log_to_table: ignoring writebatch insert error"
                    );
                    status = crate::Status::ok(); // Keep going with rest of file
                }
            }

            // `lfile_holder` drops here (mirrors `delete lfile;`).
        }

        // Do not record a version edit for this conversion to a Table
        // since ExtractMetaData() will also generate edits.
        let mut meta = FileMetaData::default();

        let file_no = self.next_file_number;
        self.next_file_number = self.next_file_number.wrapping_add(1);
        meta.set_number(file_no);

        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };

        status = build_table(
            &self.dbname,
            &mut *self.env,
            &self.options,
            self.table_cache,
            iter,
            &mut meta,
        );

        unsafe {
            drop(Box::from_raw(iter));
        }

        unsafe {
            (*mem).unref();
        }
        mem = ptr::null_mut();

        if status.is_ok() {
            if *meta.file_size() > 0 {
                self.table_numbers.push(*meta.number());
            }
        }

        info!(
            lognum = log,
            ops_saved = counter,
            table_no = *meta.number(),
            status = %status.to_string(),
            "Repairer::convert_log_to_table: ops saved to Table"
        );

        trace!(lognum = log, "Repairer::convert_log_to_table: done");
        status
    }
}

#[cfg(test)]
mod convert_log_to_table_error_and_smoke_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn convert_log_to_table_returns_error_when_log_file_is_missing() {
        let db = EphemeralDbDir::new("convert-log-missing");
        let dbname: String = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let lognum: u64 = 12345;
        let log_path = log_file_name(&dbname, lognum);
        assert!(!path_exists(&log_path), "precondition: log should be absent");

        trace!(lognum, log_path = %log_path, "calling convert_log_to_table");
        let st = repairer.convert_log_to_table(lognum);

        info!(
            lognum,
            ok = st.is_ok(),
            status = %st.to_string(),
            "convert_log_to_table returned"
        );
        assert!(
            !st.is_ok(),
            "expected conversion to fail for missing log file; got: {}",
            st.to_string()
        );
    }

    #[traced_test]
    fn convert_log_to_table_does_not_panic_for_empty_existing_log_file() {
        let db = EphemeralDbDir::new("convert-log-empty-file");
        let dbname: String = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let lognum: u64 = 1;
        let log_path = log_file_name(&dbname, lognum);
        touch_file(&log_path);
        assert!(path_exists(&log_path));

        trace!(lognum, log_path = %log_path, "calling convert_log_to_table");
        let st = repairer.convert_log_to_table(lognum);

        info!(
            lognum,
            ok = st.is_ok(),
            status = %st.to_string(),
            "convert_log_to_table returned"
        );

        // Empty logs are valid in the sense that the conversion should be resilient.
        // Accept either Ok or a non-fatal error depending on underlying log/table code,
        // but require that it returns a Status and does not panic.
        let _ = st;
    }
}
