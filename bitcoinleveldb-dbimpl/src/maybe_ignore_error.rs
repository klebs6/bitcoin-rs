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
        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, false);
        let dbname: String = "bitcoinleveldb_dbimpl_maybe_ignore_error_ok".to_string();
        let db = DBImpl::new(&options, &dbname);

        let mut s: Status = Status::ok();
        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(status = %s.to_string(), "Status after maybe_ignore_error(ok)");
        assert!(s.is_ok());
    }

    #[traced_test]
    fn maybe_ignore_error_ignores_non_ok_when_paranoid_checks_false() {
        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, false);
        let dbname: String = "bitcoinleveldb_dbimpl_maybe_ignore_error_nonparanoid".to_string();
        let db = DBImpl::new(&options, &dbname);

        let msg = Slice::from_str("io");
        let mut s: Status = Status::io_error(&msg, None);
        assert!(!s.is_ok());

        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(status = %s.to_string(), "Status after maybe_ignore_error(non-ok, paranoid=false)");
        assert!(s.is_ok(), "Non-paranoid mode must ignore (overwrite) non-OK status");
    }

    #[traced_test]
    fn maybe_ignore_error_preserves_non_ok_when_paranoid_checks_true() {
        let env = PosixEnv::shared();
        let options = build_options_with_paranoid_checks(env, true);
        let dbname: String = "bitcoinleveldb_dbimpl_maybe_ignore_error_paranoid".to_string();
        let db = DBImpl::new(&options, &dbname);

        let msg = Slice::from_str("io");
        let mut s: Status = Status::io_error(&msg, None);
        assert!(!s.is_ok());

        db.maybe_ignore_error(&mut s as *mut Status);

        tracing::debug!(status = %s.to_string(), "Status after maybe_ignore_error(non-ok, paranoid=true)");
        assert!(!s.is_ok(), "Paranoid mode must preserve non-OK status");
    }
}
