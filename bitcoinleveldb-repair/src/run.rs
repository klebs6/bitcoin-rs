// ---------------- [ File: bitcoinleveldb-repair/src/run.rs ]
crate::ix!();

impl Repairer {
    pub fn run(&mut self) -> crate::Status {
        trace!(dbname = %self.dbname, "Repairer::run: start");

        let mut status: crate::Status = self.find_files();

        if status.is_ok() {
            self.convert_log_files_to_tables();
            self.extract_meta_data();
            status = self.write_descriptor();
        }

        if status.is_ok() {
            let mut bytes: u64 = 0;
            for i in 0..self.tables.len() {
                bytes = bytes.wrapping_add(*self.tables[i].meta.file_size());
            }

            info!(
                dbname = %self.dbname,
                recovered_files = self.tables.len(),
                recovered_bytes = bytes,
                "**** Repaired leveldb; Some data may have been lost. ****"
            );
        } else {
            warn!(
                dbname = %self.dbname,
                status = %status.to_string(),
                "Repairer::run: failed"
            );
        }

        trace!(dbname = %self.dbname, "Repairer::run: done");
        status
    }
}

#[cfg(test)]
mod repairer_run_pipeline_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn run_returns_error_for_non_directory_dbname() {
        let db = EphemeralDbDir::new("run-non-dir");
        let dbname = format!("{}/NOT_A_DIR", db.path_string());
        touch_file(&dbname);
        assert!(!is_directory(&dbname));

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling Repairer::run");
        let st = repairer.run();

        info!(ok = st.is_ok(), status = %st.to_string(), "run returned");
        assert!(!st.is_ok(), "expected run failure");
    }

    #[traced_test]
    fn run_succeeds_and_installs_new_manifest_on_minimal_nonempty_directory() {
        let db = EphemeralDbDir::new("run-minimal-nonempty");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling Repairer::run");
        let st = repairer.run();

        info!(ok = st.is_ok(), status = %st.to_string(), "run returned");
        assert!(st.is_ok(), "expected ok run: {}", st.to_string());

        let manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&manifest), "expected new manifest installed");

        let current_guess = read_current_file_guess(&dbname).unwrap_or_default();
        debug!(current = %current_guess, "CURRENT contents (best-effort)");
        assert!(
            current_guess.contains("MANIFEST-000001") || current_guess.contains("MANIFEST-1"),
            "expected CURRENT to mention manifest 000001; got: {:?}",
            current_guess
        );
    }

    #[traced_test]
    fn run_archives_existing_manifest_files_before_installing_new_one() {
        let db = EphemeralDbDir::new("run-archives-old-manifest");
        let dbname: String = db.path_string();

        let old_manifest = descriptor_file_name(&dbname, 2);
        touch_file(&old_manifest);

        let current = format!("{}/CURRENT", dbname);
        write_text_file(&current, "MANIFEST-000002\n");

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, old_manifest = %old_manifest, "calling run");
        let st = repairer.run();

        info!(ok = st.is_ok(), status = %st.to_string(), "run returned");
        assert!(st.is_ok(), "expected ok run: {}", st.to_string());

        let _ = assert_archived(&old_manifest);

        let new_manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&new_manifest), "expected new manifest installed");
    }
}
