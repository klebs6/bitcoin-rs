// ---------------- [ File: bitcoinleveldb-repair/src/public_surface_suite.rs ]
crate::ix!();

#[cfg(test)]
mod crate_public_api_surface_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn repairdb_creates_manifest_and_current_for_minimal_directory() {
        let db = EphemeralDbDir::new("public-api-repairdb-minimal");
        let dbname: String = db.path_string();

        // Ensure the directory is not empty in a way that won't collide with DB filenames.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);

        trace!(dbname = %dbname, "calling repairdb");
        let status = repairdb(&dbname, &options);

        info!(
            dbname = %dbname,
            ok = status.is_ok(),
            status = %status.to_string(),
            "repairdb returned"
        );

        assert!(status.is_ok(), "repairdb expected ok: {}", status.to_string());

        let manifest = descriptor_file_name(&dbname, 1);
        debug!(manifest = %manifest, "checking new manifest existence");
        assert!(
            path_exists(&manifest),
            "expected new manifest to exist: {}",
            manifest
        );

        let current_guess = read_current_file_guess(&dbname).unwrap_or_default();
        info!(current = %current_guess, "CURRENT contents (best-effort)");
        assert!(
            current_guess.contains("MANIFEST-000001") || current_guess.contains("MANIFEST-1"),
            "expected CURRENT to mention manifest 000001; got: {:?}",
            current_guess
        );
    }

    #[traced_test]
    fn repairer_run_succeeds_with_at_least_one_non_db_file_present() {
        let db = EphemeralDbDir::new("public-api-repairer-run-minimal");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling Repairer::run");
        let status = repairer.run();

        info!(
            dbname = %dbname,
            ok = status.is_ok(),
            status = %status.to_string(),
            "Repairer::run returned"
        );

        assert!(status.is_ok(), "expected ok run: {}", status.to_string());
    }
}
