// ---------------- [ File: bitcoin-sync/src/semaphore.rs ]
crate::ix!();

/// A counting semaphore implemented with a mutex and condition variable.
#[derive(Default)]
pub struct Semaphore {
    condition: Condvar,
    /// Protected counter; value ≥ 0
    mutex: Mutex<i32>,
}

impl Semaphore {
    /// Create a semaphore with an initial count of `init`.
    pub fn new(init: i32) -> Self {
        assert!(
            init >= 0,
            "Semaphore::new — initial value must be non‑negative"
        );
        trace!("Semaphore::new — init = {}", init);

        Self {
            condition: Condvar::new(),
            mutex: Mutex::new(init),
        }
    }

    /// Block until a permit is available, then consume one.
    pub fn wait(&self) {
        trace!("Semaphore::wait — waiting for permit");

        let mut count = self
            .mutex
            .lock()
            .expect("Semaphore mutex poisoned in wait");

        while *count < 1 {
            count = self
                .condition
                .wait(count)
                .expect("Semaphore condvar poisoned in wait");
        }
        *count -= 1;

        debug!("Semaphore::wait — acquired permit; remaining = {}", *count);
    }

    /// Try to consume a permit without blocking.
    ///
    /// Returns `true` if successful.
    pub fn try_wait(&self) -> bool {
        trace!("Semaphore::try_wait — attempting non‑blocking acquire");

        let mut count = self
            .mutex
            .lock()
            .expect("Semaphore mutex poisoned in try_wait");

        if *count < 1 {
            debug!("Semaphore::try_wait — no permit available");
            return false;
        }
        *count -= 1;
        debug!(
            "Semaphore::try_wait — succeeded; remaining = {}",
            *count
        );
        true
    }

    /// Add a permit and wake one waiting thread (if any).
    pub fn post(&self) {
        trace!("Semaphore::post — releasing permit");

        let mut count = self
            .mutex
            .lock()
            .expect("Semaphore mutex poisoned in post");

        *count += 1;
        debug!(
            "Semaphore::post — new count = {}; notifying one waiter",
            *count
        );
        self.condition.notify_one();
    }
}

/// ---------------- Tests for `Semaphore`
#[cfg(test)]
mod semaphore_functionality_tests {
    use super::*;

    /// Confirm that `wait` blocks until `post` is called.
    #[traced_test]
    fn waiter_blocks_until_post() {
        let sem = Arc::new(Semaphore::new(0));
        let waiter = Arc::clone(&sem);

        let start = Instant::now();
        let handle = thread::spawn(move || {
            waiter.wait();
            Instant::now()
        });

        // Ensure the waiter is blocked.
        thread::sleep(Duration::from_millis(100));
        sem.post(); // Unblock.

        let wake_time = handle.join().expect("waiter thread panicked");
        let blocked_for = wake_time.duration_since(start);

        // The waiter should have blocked for at least ~100 ms.
        assert!(
            blocked_for >= Duration::from_millis(90),
            "waiter unblocked too early: {:?}",
            blocked_for
        );
    }

    /// Verify the behaviour of `try_wait`.
    #[traced_test]
    fn try_wait_behaviour() {
        let sem = Semaphore::new(1);

        assert!(sem.try_wait(), "first try_wait should succeed");
        assert!(
            !sem.try_wait(),
            "second try_wait should fail with count 0"
        );

        sem.post();
        assert!(sem.try_wait(), "try_wait should succeed after post");
    }
}
