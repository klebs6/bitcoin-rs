// ---------------- [ File: bitcoin-sync/src/threadinterrupt.rs ]
crate::ix!();

use parking_lot::{Condvar, Mutex};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration as StdDuration, Instant},
};

//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.h]
//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.cpp]

/// Interruptible sleeper modelled after Bitcoin Core’s `CThreadInterrupt`.
///
/// * Call [`invoke`](Self::invoke) to wake – and permanently
///   interrupt – all sleepers.
/// * Call [`reset`](Self::reset) to clear the interrupt flag.
#[derive(Default)]
pub struct ThreadInterrupt {
    cond: Condvar,
    gate: Mutex<()>,
    flag: AtomicBool,
}

impl ThreadInterrupt {
    /// `true` once an interrupt has been requested.
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }

    /// Create a new, *non‑interrupted* instance.
    pub fn new() -> Self {
        trace!("ThreadInterrupt::new");
        Self {
            cond: Condvar::new(),
            gate: Mutex::new(()),
            flag: AtomicBool::new(false),
        }
    }

    /// Clear any pending interrupt.
    pub fn reset(&self) {
        trace!("ThreadInterrupt::reset");
        self.flag.store(false, Ordering::Release);
    }

    /// Set the interrupt flag and wake all sleepers.
    pub fn invoke(&self) {
        trace!("ThreadInterrupt::invoke");
        {
            let _guard = self.gate.lock();
            self.flag.store(true, Ordering::Release);
        }
        self.cond.notify_all();
    }

    /// Sleep for `rel_time`.
    ///
    /// *Returns*  
    /// `true`  → the **full** timeout elapsed  
    /// `false` → interrupted first
    pub fn sleep_for(&self, rel_time: StdDuration) -> bool {
        debug!(
            "ThreadInterrupt::sleep_for — requested {:?}",
            rel_time
        );

        if self.as_bool() {
            debug!("ThreadInterrupt::sleep_for — already interrupted");
            return false;
        }

        let deadline = Instant::now() + rel_time;
        let mut guard = self.gate.lock();

        loop {
            if self.as_bool() {
                return false; // interrupted
            }

            let now = Instant::now();
            if now >= deadline {
                return true; // timed out
            }

            let remaining = deadline - now;
            // `parking_lot::Condvar::wait_for` returns `WaitTimeoutResult`
            // → `.timed_out()` yields the required `bool`.
            if self
                .cond
                .wait_for(&mut guard, remaining)
                .timed_out()
            {
                return true; // waited the entire remaining period
            }
            // Otherwise we were notified: loop back to re‑check.
        }
    }
}

/// ---------------- tests
#[cfg(test)]
mod thread_interrupt_timing_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration as StdDuration, Instant};

    #[traced_test]
    fn uninterrupted_sleep_times_out() {
        let ti = ThreadInterrupt::new();
        let start = Instant::now();

        let timed_out = ti.sleep_for(StdDuration::from_millis(150));
        let elapsed   = start.elapsed();

        assert!(timed_out, "sleep_for must return true on timeout");
        assert!(
            elapsed >= StdDuration::from_millis(140),
            "elapsed {:?} unexpectedly short",
            elapsed
        );
    }

    #[traced_test]
    fn interrupt_wakes_early() {
        let ti      = Arc::new(ThreadInterrupt::new());
        let sleeper = Arc::clone(&ti);

        let handle = thread::spawn(move || sleeper.sleep_for(StdDuration::from_secs(10)));

        thread::sleep(StdDuration::from_millis(50));
        ti.invoke();

        let timed_out = handle.join().expect("sleeping thread panicked");
        assert!(!timed_out, "sleep_for should return false after interrupt");
    }
}
