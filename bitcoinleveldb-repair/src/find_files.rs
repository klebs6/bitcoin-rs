// ---------------- [ File: bitcoinleveldb-repair/src/find_files.rs ]
crate::ix!();

impl Repairer {
    
    pub fn find_files(&mut self) -> crate::Status {
        trace!(dbname = %self.dbname(), "Repairer::find_files: start");

        let mut filenames: Vec<String> = Vec::new();

        let dbname_ptr: *const String = self.dbname() as *const String;
        let status = self
            .env_mut()
            .get_children(unsafe { &*dbname_ptr }, &mut filenames as *mut Vec<String>);

        if !status.is_ok() {
            debug!(
                dbname = %self.dbname(),
                status = %status.to_string(),
                "Repairer::find_files: GetChildren failed"
            );
            return status;
        }

        if filenames.is_empty() {
            let msg1 = Slice::from(self.dbname().as_bytes());
            let msg2 = Slice::from(&b"repair found no files"[..]);
            error!(
                dbname = %self.dbname(),
                "Repairer::find_files: no files found"
            );
            return crate::Status::io_error(&msg1, Some(&msg2));
        }

        let mut number: u64 = 0;
        let mut ty: FileType = FileType::LogFile;

        for i in 0..filenames.len() {
            if parse_file_name(&filenames[i], &mut number as *mut u64, &mut ty as *mut FileType)
            {
                if matches!(ty, FileType::DescriptorFile) {
                    self.manifests_mut().push(filenames[i].clone());
                } else {
                    let next = number.wrapping_add(1);
                    if next > *self.next_file_number() {
                        *self.next_file_number_mut() = next;
                    }

                    match ty {
                        FileType::LogFile => {
                            self.logs_mut().push(number);
                        }
                        FileType::TableFile => {
                            self.table_numbers_mut().push(number);
                        }
                        _ => {
                            // Ignore other files
                        }
                    }
                }
            }
        }

        debug!(
            manifests = self.manifests().len(),
            logs = self.logs().len(),
            tables = self.table_numbers().len(),
            next_file_number = *self.next_file_number(),
            "Repairer::find_files: done"
        );

        status
    }
}

#[cfg(test)]
mod find_files_discovery_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn find_files_returns_error_for_non_directory_path() {
        let db = EphemeralDbDir::new("find-files-non-dir");
        let file_path = format!("{}/NOT_A_DIR", db.path_string());
        touch_file(&file_path);
        assert!(path_exists(&file_path));
        assert!(!is_directory(&file_path));

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&file_path, &options);

        trace!(path = %file_path, "calling find_files on non-directory");
        let st = repairer.find_files();

        info!(ok = st.is_ok(), status = %st.to_string(), "find_files returned");
        assert!(
            !st.is_ok(),
            "expected find_files to fail for non-directory dbname"
        );
    }

    #[traced_test]
    fn find_files_succeeds_when_directory_contains_any_file() {
        let db = EphemeralDbDir::new("find-files-nonempty");
        let dbname: String = db.path_string();

        // Any file presence makes filenames non-empty.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling find_files");
        let st = repairer.find_files();

        info!(ok = st.is_ok(), status = %st.to_string(), "find_files returned");
        assert!(st.is_ok(), "expected ok: {}", st.to_string());
    }

    #[traced_test]
    fn find_files_discovers_logs_tables_and_manifests_via_run_side_effects() {
        let db = EphemeralDbDir::new("find-files-discovers-by-run");
        let dbname: String = db.path_string();

        let log3 = log_file_name(&dbname, 3);
        let table4 = table_file_name(&dbname, 4);
        let manifest2 = descriptor_file_name(&dbname, 2);

        touch_file(&log3);
        touch_file(&table4);
        touch_file(&manifest2);

        // Also create CURRENT to resemble a DB directory; content isn't required here.
        let current = format!("{}/CURRENT", dbname);
        write_text_file(&current, "MANIFEST-000002\n");

        info!(
            log3 = %log3,
            table4 = %table4,
            manifest2 = %manifest2,
            "seeded db directory with representative files"
        );

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "running Repairer::run");
        let st = repairer.run();

        info!(
            ok = st.is_ok(),
            status = %st.to_string(),
            "Repairer::run returned"
        );
        assert!(st.is_ok(), "expected run ok: {}", st.to_string());

        // Logs are always archived during convert_log_files_to_tables.
        let _ = assert_archived(&log3);

        // Manifests are archived during write_descriptor.
        let _ = assert_archived(&manifest2);

        // Invalid/empty table files should be archived via scan_table->repair_table.
        let _ = assert_archived(&table4);

        let new_manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&new_manifest), "expected new manifest created");
    }
}
