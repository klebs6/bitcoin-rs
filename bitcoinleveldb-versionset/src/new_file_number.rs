// ---------------- [ File: bitcoinleveldb-versionset/src/new_file_number.rs ]
crate::ix!();

impl NewFileNumber for VersionSet {
    /// Allocate and return a new file number
    /// 
    fn new_file_number(&mut self) -> u64 {
        let n: u64 = self.next_file_number();
        let next: u64 = n.wrapping_add(1);

        trace!(
            next_file_number_before = n,
            next_file_number_after = next,
            "VersionSet::new_file_number allocated"
        );

        self.set_next_file_number(next);
        n
    }
}

#[cfg(test)]
mod new_file_number_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn new_file_number_allocates_monotonically_and_wraps_at_u64_max() {
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

        let n1 = vs.new_file_number();
        let n2 = vs.new_file_number();
        debug!(n1, n2, "allocated file numbers");
        assert!(n2 > n1, "file numbers must increase when not wrapping");
        assert_eq!(vs.next_file_number(), n2 + 1, "next_file_number must advance after allocation");

        vs.set_next_file_number(u64::MAX);
        let nmax = vs.new_file_number();
        debug!(nmax, next = vs.next_file_number(), "wrap allocation");
        assert_eq!(nmax, u64::MAX, "allocation at u64::MAX must return u64::MAX");
        assert_eq!(vs.next_file_number(), 0, "next_file_number must wrap to 0 after u64::MAX allocation");

        // Restore to avoid surprising subsequent uses in this test.
        vs.set_next_file_number(2);
    }
}
