// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_ignore_error.rs ]
crate::ix!();

impl DBImpl {

    pub fn maybe_ignore_error(&self, s: *mut Status) {
        unsafe {
            if (*s).is_ok() || *self.options.paranoid_checks() {
                // No change needed
            } else {
                tracing::warn!(status = %(*s).to_string(), "Ignoring error");
                *s = Status::ok();
            }
        }
    }
}

#[cfg(test)]
mod maybe_ignore_error_behavior_suite {
    use super::*;

    fn build_options_with_paranoid_checks(env: Rc<RefCell<dyn Env>>, paranoid: bool) -> Options {
        let mut options = Options::with_env(env);
        options.set_paranoid_checks(paranoid);
        options
    }

    #[traced_test]
    fn maybe_ignore_error_preserves_ok_status() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dbname: String = std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_maybe_ignore_error_ok_{}_{}",
                std::process::id(),
                nanos
            ))
            .to_string_lossy()
            .to_string();

        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, false);

        tracing::info!(dbname = %dbname, "Constructing DBImpl for maybe_ignore_error(ok) test");
        let db = DBImpl::new(&options, &dbname);

        let mut s: Status = Status::ok();
        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(status = %s.to_string(), "Status after maybe_ignore_error(ok)");
        assert!(s.is_ok());

        drop(db);

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(path = %dbname, "Removed maybe_ignore_error(ok) test directory"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(path = %dbname, "No maybe_ignore_error(ok) test directory to remove");
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove maybe_ignore_error(ok) test directory"
            ),
        }
    }

    #[traced_test]
    fn maybe_ignore_error_ignores_non_ok_when_paranoid_checks_false() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dbname: String = std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_maybe_ignore_error_nonparanoid_{}_{}",
                std::process::id(),
                nanos
            ))
            .to_string_lossy()
            .to_string();

        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, false);

        tracing::info!(
            dbname = %dbname,
            "Constructing DBImpl for maybe_ignore_error(non-ok, paranoid=false) test"
        );
        let db = DBImpl::new(&options, &dbname);

        let msg = Slice::from_str("io");
        let mut s: Status = Status::io_error(&msg, None);
        assert!(!s.is_ok());

        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(
            status = %s.to_string(),
            "Status after maybe_ignore_error(non-ok, paranoid=false)"
        );

        assert!(s.is_ok(), "Non-paranoid mode must ignore (overwrite) non-OK status");

        drop(db);

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(
                path = %dbname,
                "Removed maybe_ignore_error(nonparanoid) test directory"
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(
                    path = %dbname,
                    "No maybe_ignore_error(nonparanoid) test directory to remove"
                );
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove maybe_ignore_error(nonparanoid) test directory"
            ),
        }
    }

    #[traced_test]
    fn maybe_ignore_error_preserves_non_ok_when_paranoid_checks_true() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dbname: String = std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_maybe_ignore_error_paranoid_{}_{}",
                std::process::id(),
                nanos
            ))
            .to_string_lossy()
            .to_string();

        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, true);

        tracing::info!(
            dbname = %dbname,
            "Constructing DBImpl for maybe_ignore_error(non-ok, paranoid=true) test"
        );
        let db = DBImpl::new(&options, &dbname);

        let msg = Slice::from_str("io");
        let mut s: Status = Status::io_error(&msg, None);
        assert!(!s.is_ok());

        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(
            status = %s.to_string(),
            "Status after maybe_ignore_error(non-ok, paranoid=true)"
        );

        assert!(!s.is_ok(), "Paranoid mode must preserve non-OK status");

        drop(db);

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(
                path = %dbname,
                "Removed maybe_ignore_error(paranoid) test directory"
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(
                    path = %dbname,
                    "No maybe_ignore_error(paranoid) test directory to remove"
                );
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove maybe_ignore_error(paranoid) test directory"
            ),
        }
    }
}
