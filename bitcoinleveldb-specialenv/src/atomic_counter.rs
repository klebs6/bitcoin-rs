// ---------------- [ File: bitcoinleveldb-specialenv/src/atomic_counter.rs ]
crate::ix!();

pub struct AtomicCounter {
    mu:    Mutex<AtomicCounterInner>,

}

pub struct AtomicCounterInner {
    count: i32,
}

impl Default for AtomicCounter {
    fn default() -> Self {
        trace!("AtomicCounter::default");
        Self {
            mu: Mutex::new(AtomicCounterInner { count: 0 }),
        }
    }
}

impl AtomicCounter {
    pub fn increment(&self) {
        trace!("AtomicCounter::increment");
        self.increment_by(1);
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn increment_by(&self, count: i32) {
        let mut guard = self.mu.lock();
        let before = guard.count;
        let after = before.wrapping_add(count);
        guard.count = after;

        debug!(
            delta = count,
            before = before,
            after = after,
            "AtomicCounter::increment_by"
        );
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn read(&self) -> i32 {
        let guard = self.mu.lock();
        let v = guard.count;
        trace!(value = v, "AtomicCounter::read");
        v
    }

    pub fn reset(&self) {
        let mut guard = self.mu.lock();
        let before = guard.count;
        guard.count = 0;
        debug!(before = before, after = 0, "AtomicCounter::reset");
    }
}

#[cfg(test)]
mod atomic_counter_contract_suite {
    crate::ix!();

    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[traced_test]
    fn atomic_counter_defaults_to_zero_and_is_readable() {
        trace!("test: atomic_counter_defaults_to_zero_and_is_readable");
        let counter = AtomicCounter::default();
        let v = counter.read();
        debug!(value = v, "observed default counter value");
        assert_eq!(v, 0);
    }

    #[traced_test]
    fn atomic_counter_increment_increases_by_one() {
        trace!("test: atomic_counter_increment_increases_by_one");
        let counter = AtomicCounter::default();

        let before = counter.read();
        counter.increment();
        let after = counter.read();

        debug!(before, after, "increment changed counter value");
        assert_eq!(before, 0);
        assert_eq!(after, 1);
    }

    #[traced_test]
    fn atomic_counter_increment_by_supports_positive_negative_and_zero_deltas() {
        trace!("test: atomic_counter_increment_by_supports_positive_negative_and_zero_deltas");
        let counter = AtomicCounter::default();

        counter.increment_by(0);
        assert_eq!(counter.read(), 0);

        counter.increment_by(7);
        assert_eq!(counter.read(), 7);

        counter.increment_by(-3);
        assert_eq!(counter.read(), 4);

        counter.increment_by(-4);
        assert_eq!(counter.read(), 0);
    }

    #[traced_test]
    fn atomic_counter_reset_sets_value_back_to_zero() {
        trace!("test: atomic_counter_reset_sets_value_back_to_zero");
        let counter = AtomicCounter::default();

        counter.increment_by(123);
        assert_eq!(counter.read(), 123);

        counter.reset();
        assert_eq!(counter.read(), 0);

        counter.increment();
        assert_eq!(counter.read(), 1);

        counter.reset();
        assert_eq!(counter.read(), 0);
    }

    #[traced_test]
    fn atomic_counter_increment_by_wraps_on_overflow_and_underflow() {
        trace!("test: atomic_counter_increment_by_wraps_on_overflow_and_underflow");
        let counter = AtomicCounter::default();

        counter.increment_by(i32::MAX);
        assert_eq!(counter.read(), i32::MAX);

        counter.increment();
        assert_eq!(counter.read(), i32::MIN);

        counter.reset();
        assert_eq!(counter.read(), 0);

        counter.increment_by(i32::MIN);
        assert_eq!(counter.read(), i32::MIN);

        counter.increment_by(-1);
        assert_eq!(counter.read(), i32::MAX);
    }

    #[traced_test]
    fn atomic_counter_handles_concurrent_increments_without_lost_updates() {
        trace!("test: atomic_counter_handles_concurrent_increments_without_lost_updates");

        let counter = Arc::new(AtomicCounter::default());
        let threads: usize = 8;
        let iters_per_thread: usize = 10_000;

        debug!(threads, iters_per_thread, "spawning worker threads");

        let mut handles = Vec::with_capacity(threads);
        for tid in 0..threads {
            let c = counter.clone();
            handles.push(thread::spawn(move || {
                trace!(thread_id = tid, "worker start");
                for _ in 0..iters_per_thread {
                    c.increment();
                }
                trace!(thread_id = tid, "worker done");
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let expected: i32 = (threads * iters_per_thread) as i32;
        let observed = counter.read();

        info!(expected, observed, "final concurrent counter value");
        assert_eq!(observed, expected);
    }
}
