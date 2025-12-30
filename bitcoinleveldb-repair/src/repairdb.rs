// ---------------- [ File: bitcoinleveldb-repair/src/repairdb.rs ]
crate::ix!();

pub fn repairdb(dbname: &String, options: &Options) -> crate::Status {
    trace!(dbname = %dbname, "repairdb: start");

    let mut repairer: Repairer = Repairer::new(dbname, options);
    let status = repairer.run();

    debug!(
        dbname = %dbname,
        ok = status.is_ok(),
        status = %status.to_string(),
        "repairdb: done"
    );

    status
}

#[cfg(test)]
mod repairdb_api_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn repairdb_returns_error_for_non_directory_dbname() {
        let db = EphemeralDbDir::new("repairdb-non-dir");
        let dbname = format!("{}/NOT_A_DIR", db.path_string());
        touch_file(&dbname);
        assert!(!is_directory(&dbname));

        let options = Options::default();

        trace!(dbname = %dbname, "calling repairdb for non-directory path");
        let st = repairdb(&dbname, &options);

        info!(ok = st.is_ok(), status = %st.to_string(), "repairdb returned");
        assert!(!st.is_ok(), "expected failure status");
    }

    #[traced_test]
    fn repairdb_succeeds_and_writes_manifest_for_minimal_nonempty_directory() {
        let db = EphemeralDbDir::new("repairdb-minimal-success");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();

        trace!(dbname = %dbname, "calling repairdb");
        let st = repairdb(&dbname, &options);

        info!(ok = st.is_ok(), status = %st.to_string(), "repairdb returned");
        assert!(st.is_ok(), "expected ok: {}", st.to_string());

        let manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&manifest), "expected manifest file to exist");

        let current_guess = read_current_file_guess(&dbname).unwrap_or_default();
        debug!(current = %current_guess, "CURRENT contents (best-effort)");
        assert!(
            current_guess.contains("MANIFEST-000001") || current_guess.contains("MANIFEST-1"),
            "expected CURRENT to mention manifest 000001; got: {:?}",
            current_guess
        );
    }
}
