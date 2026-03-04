// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl DBImpl {
    
    /// Return the maximum overlapping data (in bytes) at next level for any file at a level >= 1.
    pub fn test_max_next_level_overlapping_bytes(&mut self) -> i64 { 
        self.mutex.lock();
        let v = unsafe { (*self.versions).max_next_level_overlapping_bytes() };
        unsafe { self.mutex.unlock() };
        v
    }
}

#[cfg(test)]
mod max_next_level_overlapping_bytes_interface_suite {
    use super::*;

    fn build_temp_db_path_for_max_next_level_overlap_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_or_panic_for_max_next_level_overlap_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!(
                "Options::with_env(env) produced Options with env=None; cannot run max_next_level overlap suite"
            );
            panic!();
        }

        options
    }

    #[traced_test]
    fn test_max_next_level_overlapping_bytes_signature_is_stable() {
        tracing::info!(
            "Asserting DBImpl::test_max_next_level_overlapping_bytes signature is stable"
        );

        type Sig = fn(&mut DBImpl) -> i64;
        let _sig: Sig = DBImpl::test_max_next_level_overlapping_bytes;

        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn test_max_next_level_overlapping_bytes_method_item_is_addressable() {
        tracing::info!(
            "Asserting DBImpl::test_max_next_level_overlapping_bytes method item is addressable"
        );

        let _m = DBImpl::test_max_next_level_overlapping_bytes;
        let _ = _m;
    }

    #[traced_test]
    fn test_max_next_level_overlapping_bytes_can_be_called_on_fresh_dbimpl_without_deadlocking() {
        let dbname = build_temp_db_path_for_max_next_level_overlap_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_max_next_level_overlap_suite();
        let mut db = DBImpl::new(&options, &dbname);

        tracing::info!(
            dbname = %dbname,
            "Calling DBImpl::test_max_next_level_overlapping_bytes on a fresh DBImpl"
        );

        let v: i64 = db.test_max_next_level_overlapping_bytes();

        tracing::debug!(value = v, "Observed max_next_level_overlapping_bytes");
        assert!(v >= 0, "Overlapping bytes should never be negative");

        let reacquired = db.mutex.try_lock();
        tracing::debug!(reacquired, "RawMutex try_lock after max_next_level_overlapping_bytes");
        assert!(
            reacquired,
            "DB mutex must be unlocked after test_max_next_level_overlapping_bytes"
        );
        unsafe { db.mutex.unlock() };

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
