// ---------------- [ File: bitcoinleveldb-repair/src/extract_meta_data.rs ]
crate::ix!();

impl Repairer {

    pub fn extract_meta_data(&mut self) {
        trace!(
            table_count = self.table_numbers.len(),
            "Repairer::extract_meta_data: start"
        );

        for i in 0..self.table_numbers.len() {
            self.scan_table(self.table_numbers[i]);
        }

        trace!(
            tables_scanned = self.table_numbers.len(),
            tables_kept = self.tables.len(),
            "Repairer::extract_meta_data: done"
        );
    }
}

#[cfg(test)]
mod extract_meta_data_flow_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn extract_meta_data_archives_invalid_table_files_discovered_by_find_files() {
        let db = EphemeralDbDir::new("extract-meta-invalid-table");
        let dbname: String = db.path_string();

        let table_no: u64 = 7;
        let table_path = table_file_name(&dbname, table_no);
        touch_file(&table_path);

        info!(table_no, table_path = %table_path, "created empty table file");

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let st = repairer.find_files();
        assert!(st.is_ok(), "find_files expected ok: {}", st.to_string());

        trace!(table_no, "calling extract_meta_data");
        repairer.extract_meta_data();

        // Empty/invalid table should be archived during ScanTable/RepairTable flow.
        let _dst = assert_archived(&table_path);

        let lost_dir = format!("{}/lost", dbname);
        assert!(is_directory(&lost_dir), "expected lost dir to exist");
    }

    #[traced_test]
    fn extract_meta_data_is_noop_when_no_table_files_are_present() {
        let db = EphemeralDbDir::new("extract-meta-no-tables");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let st = repairer.find_files();
        assert!(st.is_ok(), "find_files expected ok: {}", st.to_string());

        trace!("calling extract_meta_data (no tables expected)");
        repairer.extract_meta_data();

        // Sentinel should remain; no guarantee about lost dir creation, so only assert
        // stability of the directory and sentinel file.
        assert!(path_exists(&sentinel));
    }
}
