// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_ignore_error.rs ]
crate::ix!();

impl DBImpl {
    pub fn maybe_ignore_error(&self, s: *mut Status) {
        unsafe {
            if (*s).is_ok() || self.options_.paranoid_checks {
                // No change needed
            } else {
                tracing::warn!(status = %(*s).to_string(), "Ignoring error");
                *s = Status::ok();
            }
        }
    }
}

#[cfg(test)]
#[disable]
mod maybe_ignore_error_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn maybe_ignore_error_preserves_errors_when_paranoid_and_clears_when_not() {
        // Non-paranoid -> clears.
        {
            let mut opts: Options = default_test_options();
            opts.paranoid_checks = false;

            let dbname: String = unique_dbname("maybe_ignore_error_nonparanoid");
            remove_db_dir_best_effort(&dbname);

            let db: DBImpl = DBImpl::new(&opts, &dbname);

            let mut s: Status = Status::io_error("io", "simulated");
            db.maybe_ignore_error((&mut s) as *mut Status);

            tracing::info!(status = %s.to_string(), "after maybe_ignore_error non-paranoid");
            assert!(s.is_ok(), "non-paranoid should clear errors");

            remove_db_dir_best_effort(&dbname);
        }

        // Paranoid -> preserves.
        {
            let mut opts: Options = default_test_options();
            opts.paranoid_checks = true;

            let dbname: String = unique_dbname("maybe_ignore_error_paranoid");
            remove_db_dir_best_effort(&dbname);

            let db: DBImpl = DBImpl::new(&opts, &dbname);

            let mut s: Status = Status::io_error("io", "simulated");
            db.maybe_ignore_error((&mut s) as *mut Status);

            tracing::info!(status = %s.to_string(), "after maybe_ignore_error paranoid");
            assert!(!s.is_ok(), "paranoid should not clear errors");

            remove_db_dir_best_effort(&dbname);
        }
    }
}
