// ---------------- [ File: bitcoinleveldb-versionset/src/reuse_file_number.rs ]
crate::ix!();

impl ReuseFileNumber for VersionSet {

    /// Arrange to reuse "file_number" unless a newer file number has already
    /// been allocated.
    /// 
    /// REQUIRES: "file_number" was returned by a call to NewFileNumber().
    fn reuse_file_number(&mut self, file_number: u64) {
        let cur_next: u64 = self.next_file_number();

        trace!(
            file_number = file_number,
            next_file_number_before = cur_next,
            "VersionSet::reuse_file_number called"
        );

        if file_number.wrapping_add(1) == cur_next {
            self.set_next_file_number(file_number);
            trace!(
                next_file_number_after = file_number,
                "VersionSet::reuse_file_number rolled back next_file_number"
            );
        } else {
            debug!(
                file_number = file_number,
                next_file_number = cur_next,
                "VersionSet::reuse_file_number did not roll back (not most-recent allocation)"
            );
        }
    }
}

#[cfg(test)]
mod reuse_file_number_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn reuse_file_number_rolls_back_only_for_most_recent_allocation_and_wraps() {
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

        let f1 = vs.new_file_number();
        let f2 = vs.new_file_number();
        debug!(f1, f2, next = vs.next_file_number(), "allocated numbers");
        assert_eq!(vs.next_file_number(), f2 + 1);

        vs.reuse_file_number(f1);
        assert_eq!(
            vs.next_file_number(),
            f2 + 1,
            "reuse_file_number must not roll back unless file_number was the most recent allocation"
        );

        vs.reuse_file_number(f2);
        assert_eq!(
            vs.next_file_number(),
            f2,
            "reuse_file_number must roll back when file_number was the most recent allocation"
        );

        // Explicit wrap edge: if next==0 and file_number==u64::MAX, the (+1) check matches.
        vs.set_next_file_number(0);
        vs.reuse_file_number(u64::MAX);
        assert_eq!(
            vs.next_file_number(),
            u64::MAX,
            "reuse_file_number must handle wrapping add semantics"
        );

        // Restore for cleanliness.
        vs.set_next_file_number(2);
    }
}
