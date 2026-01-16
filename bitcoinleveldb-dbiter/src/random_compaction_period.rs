// ---------------- [ File: bitcoinleveldb-dbiter/src/random_compaction_period.rs ]
crate::ix!();

impl DBIter {
    /// Picks the number of bytes that can be
    /// read until a compaction is scheduled.
    /// 
    pub fn random_compaction_period(&mut self) -> usize {
        // C++: return rnd_.Uniform(2 * config::kReadBytesPeriod);
        let max: i32 = (2u64 * (READ_BYTES_PERIOD as u64)) as i32;
        let r: u32 = self.rnd_mut().uniform(max);

        r as usize
    }
}

#[cfg(test)]
mod dbiter_random_compaction_period_suite {
    use super::*;

    #[traced_test]
    fn random_compaction_period_is_within_expected_bounds() {
        info!("random_compaction_period returns a value in [0, 2*kReadBytesPeriod)");

        let (mut dbiter, _calls, _last_len) = build_dbiter_direct(10, 123, vec![]);

        for _ in 0..256 {
            let p = dbiter.random_compaction_period();
            assert!(p < (2usize * READ_BYTES_PERIOD));
        }
    }

    #[traced_test]
    fn random_compaction_period_is_deterministic_for_same_seed_and_state() {
        info!("random_compaction_period is deterministic for identical initial RNG state");

        let (mut dbiter1, _c1, _l1) = build_dbiter_direct(10, 4242, vec![]);
        let (mut dbiter2, _c2, _l2) = build_dbiter_direct(10, 4242, vec![]);

        let p1 = dbiter1.random_compaction_period();
        let p2 = dbiter2.random_compaction_period();

        assert_eq!(p1, p2);
    }
}
