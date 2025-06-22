// ---------------- [ File: bitcoin-sync/src/threadinterrupt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.h]
//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.cpp]

/// A helper class for interruptible sleeps.
///
/// Calling [`invoke`](Self::invoke) will interrupt any current sleep,
/// and after that point [`as_bool`](Self::as_bool) will return `true`
/// until [`reset`](Self::reset) is called.
#[derive(Default)]
pub struct ThreadInterrupt {
    cond: Condvar,
    /// Internal mutex used in conjunction with `cond`.
    mut_: Mutex<()>,
    flag: AtomicBool,
}

impl ThreadInterrupt {
    /// Query whether an interrupt has been requested.
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.flag.load(atomic::Ordering::Acquire)
    }

    /// Construct a new, *non‑interrupted* [`ThreadInterrupt`].
    pub fn new() -> Self {
        trace!("ThreadInterrupt::new");
        Self {
            cond: Condvar::new(),
            mut_: Mutex::new(()),
            flag: AtomicBool::new(false),
        }
    }

    /// Clear any pending interrupt.
    pub fn reset(&self) {
        trace!("ThreadInterrupt::reset");
        self.flag.store(false, atomic::Ordering::Release);
    }

    /// Request an interrupt and wake every sleeper.
    pub fn invoke(&self) {
        trace!("ThreadInterrupt::invoke");
        {
            // Lock scope ensures the mutex guard is dropped before `notify_all`.
            let _guard = self
                .mut_
                .lock()
                .expect("ThreadInterrupt mutex poisoned during invoke");
            self.flag.store(true, atomic::Ordering::Release);
        }
        self.cond.notify_all();
    }

    /// Sleep for `rel_time`, returning `true` **only** if the full
    /// period elapsed **without** an interrupt.
    ///
    /// This mirrors the original C++ semantics:
    /// * `true`  → timeout reached  
    /// * `false` → interrupted before timeout
    pub fn sleep_for(&self, rel_time: Duration) -> bool {
        debug!(
            "ThreadInterrupt::sleep_for — {:?} requested",
            rel_time
        );

        // Fast‑path: abort early if already interrupted.
        if self.as_bool() {
            debug!("ThreadInterrupt::sleep_for — immediate interrupt");
            return false;
        }

        let guard = self
            .mut_
            .lock()
            .expect("ThreadInterrupt mutex poisoned during sleep_for");

        // `wait_timeout_while` returns `(guard, wait_result)`
        // where `wait_result.timed_out()` is `true` when the
        // timeout elapsed **without** the predicate triggering.
        let (_guard, wait_result) = self
            .cond
            .wait_timeout_while(guard, rel_time, |_| {
                // Keep waiting **while** *not* interrupted.
                !self.as_bool()
            })
            .expect("ThreadInterrupt condvar poisoned during sleep_for");

        let timed_out = wait_result.timed_out();

        debug!(
            "ThreadInterrupt::sleep_for — finished; timed_out = {}, interrupted = {}",
            timed_out,
            self.as_bool()
        );

        timed_out
    }
}

/// ---------------- Tests for `ThreadInterrupt`
#[cfg(test)]
mod thread_interrupt_timing_tests {
    use super::*;

    /// Verify that `sleep_for` returns `true` when no interrupt occurs.
    #[traced_test]
    fn sleep_times_out_without_interrupt() {
        let ti = ThreadInterrupt::new();
        let start = Instant::now();
        let slept_full = ti.sleep_for(Duration::from_millis(150));
        let elapsed = start.elapsed();

        assert!(slept_full, "sleep_for should time out without interrupt");
        // Allow for scheduler variance.
        assert!(
            elapsed >= Duration::from_millis(140),
            "elapsed = {:?} too short",
            elapsed
        );
    }

    /// Verify that `sleep_for` returns `false` when interrupted.
    #[traced_test]
    fn sleep_returns_false_when_interrupted() {
        let ti = Arc::new(ThreadInterrupt::new());
        let sleeper = Arc::clone(&ti);

        let handle = thread::spawn(move || sleeper.sleep_for(Duration::from_secs(10)));

        // Give the thread a moment to block.
        thread::sleep(Duration::from_millis(50));
        ti.invoke();

        let slept_full = handle
            .join()
            .expect("thread panicked while sleeping");

        assert!(
            !slept_full,
            "`sleep_for` should have returned false after interrupt"
        );
    }
}
