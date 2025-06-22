// ---------------- [ File: bitcoin-sync/src/semaphore.rs ]
crate::ix!();

/// Simple counting semaphore built on `parking_lot`.
#[derive(Default)]
pub struct Semaphore {
    cv:    Condvar,
    count: Mutex<i32>,
}

impl Semaphore {
    /// Create with an initial permit count.
    pub fn new(init: i32) -> Self {
        assert!(init >= 0, "Semaphore initial count must be ≥ 0");
        trace!("Semaphore::new — init = {}", init);
        Self {
            cv: Condvar::new(),
            count: Mutex::new(init),
        }
    }

    /// Block until a permit is available, then consume it.
    pub fn wait(&self) {
        trace!("Semaphore::wait");
        let mut cnt = self.count.lock();
        while *cnt == 0 {
            self.cv.wait(&mut cnt);
        }
        *cnt -= 1;
        debug!("Semaphore::wait — remaining = {}", *cnt);
    }

    /// Attempt to consume a permit without blocking.
    pub fn try_wait(&self) -> bool {
        trace!("Semaphore::try_wait");
        let mut cnt = self.count.lock();
        if *cnt == 0 {
            debug!("Semaphore::try_wait — none available");
            return false;
        }
        *cnt -= 1;
        debug!("Semaphore::try_wait — remaining = {}", *cnt);
        true
    }

    /// Add a permit and wake one waiter.
    pub fn post(&self) {
        trace!("Semaphore::post");
        let mut cnt = self.count.lock();
        *cnt += 1;
        debug!("Semaphore::post — new count = {}", *cnt);
        self.cv.notify_one();
    }
}

#[cfg(test)]
mod semaphore_functionality_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration as StdDuration, Instant};

    #[traced_test]
    fn wait_blocks_until_post() {
        let sem   = Arc::new(Semaphore::new(0));
        let clone = Arc::clone(&sem);

        let start  = Instant::now();
        let handle = thread::spawn(move || {
            clone.wait();
            Instant::now()
        });

        thread::sleep(StdDuration::from_millis(100));
        sem.post();

        let woke_at     = handle.join().expect("waiter panicked");
        let blocked_for = woke_at - start;

        assert!(
            blocked_for >= StdDuration::from_millis(90),
            "waiter unblocked too early: {:?}",
            blocked_for
        );
    }

    #[traced_test]
    fn try_wait_semantics() {
        let sem = Semaphore::new(1);

        assert!(sem.try_wait(), "first permit should succeed");
        assert!(!sem.try_wait(), "no permits left, must fail");

        sem.post();
        assert!(sem.try_wait(), "permit after post should succeed");
    }
}
