// ---------------- [ File: bitcoinleveldb-versionset/src/get_current_log_file_number.rs ]
crate::ix!();

impl GetCurrentLogFileNumber for VersionSet {

    /// Return the current log file number.
    fn log_number(&self) -> u64 {
        let n: u64 = VersionSet::log_number(self);

        trace!(
            log_number = n,
            "VersionSet::log_number (GetCurrentLogFileNumber)"
        );

        n
    }
}

impl VersionSet {
    pub fn get_current_log_file_number(&self) -> u64 {
        let log_number: u64 = <VersionSet as GetCurrentLogFileNumber>::log_number(self);

        trace!(
            log_number = log_number,
            "VersionSet::get_current_log_file_number"
        );

        log_number
    }
}

#[cfg(test)]
mod get_current_log_file_number_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_unique_temp_db_dir(prefix: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        let mut p = std::env::temp_dir();
        p.push(format!("{prefix}_{pid}_{nanos}"));
        p
    }

    fn remove_dir_all_best_effort(dir: &Path) {
        match std::fs::remove_dir_all(dir) {
            Ok(()) => trace!(dir = %dir.display(), "removed temp db dir"),
            Err(e) => warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)"),
        }
    }

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    #[traced_test]
    fn log_number_accessors_match_and_reflect_updates() {
        let dir = make_unique_temp_db_dir("versionset_log_number_accessors");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 8));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let initial = vs.get_current_log_file_number();
        debug!(initial, "initial log number");
        assert_eq!(initial, vs.log_number(), "wrapper must match internal field");
        assert_eq!(
            initial,
            <VersionSet as GetCurrentLogFileNumber>::log_number(&vs),
            "trait method must match wrapper"
        );

        let new_val: u64 = 123456;
        vs.set_log_number(new_val);

        let after = vs.get_current_log_file_number();
        debug!(after, "updated log number");
        assert_eq!(after, new_val, "log number wrapper must reflect setter updates");

        remove_dir_all_best_effort(&dir);
    }
}
