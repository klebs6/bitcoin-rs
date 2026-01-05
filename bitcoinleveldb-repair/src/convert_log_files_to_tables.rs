// ---------------- [ File: bitcoinleveldb-repair/src/convert_log_files_to_tables.rs ]
crate::ix!();

impl Repairer {

    pub fn convert_log_files_to_tables(&mut self) {
        trace!(
            log_count = self.logs().len(),
            "Repairer::convert_log_files_to_tables: start"
        );

        let log_len = self.logs().len();
        for i in 0..log_len {
            let lognum = self.logs()[i];
            let logname = log_file_name(self.dbname(), lognum);

            trace!(
                lognum,
                file = %logname,
                "Repairer::convert_log_files_to_tables: converting log"
            );

            let status = self.convert_log_to_table(lognum);

            if !status.is_ok() {
                warn!(
                    lognum,
                    status = %status.to_string(),
                    "Repairer::convert_log_files_to_tables: ignoring conversion error"
                );
            }

            self.archive_file(&logname);
        }

        trace!("Repairer::convert_log_files_to_tables: done");
    }
}

#[cfg(test)]
mod convert_log_files_to_tables_batch_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn convert_log_files_to_tables_archives_each_discovered_log_file() {
        let db = EphemeralDbDir::new("convert-log-files-archives");
        let dbname: String = db.path_string();

        let log3 = log_file_name(&dbname, 3);
        let log5 = log_file_name(&dbname, 5);

        touch_file(&log3);
        touch_file(&log5);

        info!(log3 = %log3, log5 = %log5, "created log files to be discovered");

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        let st = repairer.find_files();
        info!(ok = st.is_ok(), status = %st.to_string(), "find_files completed");
        assert!(st.is_ok(), "find_files expected ok: {}", st.to_string());

        trace!("calling convert_log_files_to_tables");
        repairer.convert_log_files_to_tables();

        let _dst3 = assert_archived(&log3);
        let _dst5 = assert_archived(&log5);

        let lost_dir = format!("{}/lost", dbname);
        debug!(lost_dir = %lost_dir, "verifying lost dir created");
        assert!(is_directory(&lost_dir), "expected lost dir to exist");
    }

    #[traced_test]
    fn convert_log_files_to_tables_is_noop_when_no_logs_are_present() {
        let db = EphemeralDbDir::new("convert-log-files-noop");
        let dbname: String = db.path_string();

        // Ensure directory is non-empty so find_files doesn't depend on env-created logs.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        let st = repairer.find_files();
        info!(ok = st.is_ok(), status = %st.to_string(), "find_files completed");
        assert!(st.is_ok(), "find_files expected ok: {}", st.to_string());

        trace!("calling convert_log_files_to_tables with no logs");
        repairer.convert_log_files_to_tables();

        // No archiving should have happened for logs; if lost exists, it must be due to
        // unrelated behavior, so we only assert that sentinel is still present.
        assert!(path_exists(&sentinel));
    }
}
