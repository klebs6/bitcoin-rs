// ---------------- [ File: bitcoinleveldb-versionset/src/next_file_number.rs ]
crate::ix!();

impl VersionSet {
    pub(crate) fn traced_next_file_number_value(&self) -> u64 {
        let n: u64 = self.next_file_number();
        tracing::trace!(next_file_number = n, "read next_file_number");
        n
    }

    pub(crate) fn traced_set_next_file_number_value(&mut self, n: u64) {
        let old: u64 = self.next_file_number();
        tracing::info!(
            old_next_file_number = old,
            new_next_file_number = n,
            "update next_file_number"
        );
        self.set_next_file_number(n);
    }
}

#[cfg(test)]
mod next_file_number_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn traced_next_file_number_helpers_read_and_write_consistently() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let before = vs.traced_next_file_number_value();
        debug!(before, "traced_next_file_number_value");
        assert_eq!(before, vs.next_file_number(), "traced read must match direct read");

        vs.traced_set_next_file_number_value(99);
        let after = vs.traced_next_file_number_value();
        debug!(after, "after traced_set_next_file_number_value");
        assert_eq!(after, 99, "traced set must update the value");
        assert_eq!(after, vs.next_file_number(), "traced set must match direct read");
    }
}
