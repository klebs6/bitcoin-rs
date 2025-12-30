// ---------------- [ File: bitcoinleveldb-versionset/src/mark_file_number_used.rs ]
crate::ix!();
   
impl MarkFileNumberUsed for VersionSet {

    /// Mark the specified file number as used.
    /// 
    fn mark_file_number_used(&mut self, number: u64) {
        let cur_next: u64 = self.next_file_number();

        trace!(
            number = number,
            next_file_number_before = cur_next,
            "VersionSet::mark_file_number_used called"
        );

        if cur_next <= number {
            let bumped: u64 = number.wrapping_add(1);
            self.set_next_file_number(bumped);
            trace!(
                next_file_number_after = bumped,
                "VersionSet::mark_file_number_used bumped next_file_number"
            );
        }
    }
}

#[cfg(test)]
mod mark_file_number_used_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace, warn};

    #[traced_test]
    fn mark_file_number_used_bumps_next_file_number_only_when_needed_and_wraps() {
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

        let initial_next = vs.next_file_number();
        debug!(initial_next, "initial next_file_number");
        assert_eq!(initial_next, 2, "expected default next_file_number=2");

        vs.mark_file_number_used(0);
        assert_eq!(vs.next_file_number(), 2, "mark_file_number_used must not bump when next>number");

        vs.mark_file_number_used(2);
        assert_eq!(vs.next_file_number(), 3, "mark_file_number_used must bump when next<=number");

        vs.mark_file_number_used(5);
        assert_eq!(vs.next_file_number(), 6, "mark_file_number_used must bump to number+1");

        vs.mark_file_number_used(5);
        assert_eq!(
            vs.next_file_number(),
            6,
            "mark_file_number_used must not bump again if next already past number"
        );

        // Edge wrap behavior is explicitly `wrapping_add`.
        vs.set_next_file_number(123);
        vs.mark_file_number_used(u64::MAX);
        debug!(next = vs.next_file_number(), "next after wrapping bump");
        assert_eq!(vs.next_file_number(), 0, "u64::MAX + 1 must wrap to 0");

        // Restore to a sane value to keep downstream invariants stable within this test.
        vs.set_next_file_number(2);
    }
}
