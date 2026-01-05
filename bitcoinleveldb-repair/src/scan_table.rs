// ---------------- [ File: bitcoinleveldb-repair/src/scan_table.rs ]
crate::ix!();

impl Repairer {

    pub fn scan_table(&mut self, number: u64) {
        use std::ptr;

        trace!(table_no = number, "Repairer::scan_table: start");

        let mut t = RepairerTableInfo::default();
        t.meta_mut().set_number(number);

        let mut fname = table_file_name(self.dbname(), number);

        let mut status = self
            .env_mut()
            .get_file_size(&fname, t.meta_mut().file_size_mut() as *mut u64);

        if !status.is_ok() {
            // Try alternate file name.
            fname = sst_table_file_name(self.dbname(), number);

            let s2 = self
                .env_mut()
                .get_file_size(&fname, t.meta_mut().file_size_mut() as *mut u64);

            if s2.is_ok() {
                status = crate::Status::ok();
            }
        }

        if !status.is_ok() {
            self.archive_file(&table_file_name(self.dbname(), number));
            self.archive_file(&sst_table_file_name(self.dbname(), number));

            warn!(
                table_no = number,
                status = %status.to_string(),
                "Repairer::scan_table: dropped"
            );
            return;
        }

        // Extract metadata by scanning through table.
        let mut counter: i32 = 0;

        let iter = self.new_table_iterator(t.meta());

        let mut empty = true;
        let mut parsed = ParsedInternalKey::default();
        *t.max_sequence_mut() = 0;

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
                    let _ = t.meta_mut().smallest_mut().decode_from(&key);
                }

                let _ = t.meta_mut().largest_mut().decode_from(&key);

                if *parsed.sequence() > *t.max_sequence() {
                    *t.max_sequence_mut() = *parsed.sequence();
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
            self.tables_mut().push(t);
        } else {
            // RepairTable archives input file.
            self.repair_table(&fname, t);
        }

        trace!(table_no = number, "Repairer::scan_table: done");
    }
}
#[cfg(test)]
mod scan_table_behavior_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn scan_table_archives_both_candidate_names_when_table_is_missing() {
        let db = EphemeralDbDir::new("scan-table-missing");
        let dbname: String = db.path_string();

        // Ensure non-empty so Repairer::new doesn't depend on any env-created artifacts.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        let table_no: u64 = 777;
        let ldb = table_file_name(&dbname, table_no);
        let sst = sst_table_file_name(&dbname, table_no);

        assert!(!path_exists(&ldb));
        assert!(!path_exists(&sst));

        trace!(table_no, ldb = %ldb, sst = %sst, "calling scan_table for missing table");
        repairer.scan_table(table_no);

        // The archive attempts won't move anything (files absent), but the lost directory
        // should be created by archive_file's directory creation attempt.
        let lost_dir = format!("{}/lost", dbname);
        info!(lost_dir = %lost_dir, "verifying lost dir creation best-effort");
        assert!(is_directory(&lost_dir), "expected lost dir to exist");
    }

    #[traced_test]
    fn scan_table_archives_invalid_existing_table_file() {
        let db = EphemeralDbDir::new("scan-table-invalid");
        let dbname: String = db.path_string();

        let table_no: u64 = 5;
        let ldb = table_file_name(&dbname, table_no);
        touch_file(&ldb);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(table_no, ldb = %ldb, "calling scan_table for invalid table");
        repairer.scan_table(table_no);

        let _ = assert_archived(&ldb);
    }
}
