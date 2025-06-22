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
    gate: Mutex<()>,          // protects `flag`
    flag: AtomicBool,
}

impl ThreadInterrupt {
    /// `true` once an interrupt has been requested.
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }

    /// New, *non‑interrupted* instance.
    pub fn new() -> Self {
        trace!("ThreadInterrupt::new");
        Self {
            cond: Condvar::new(),
            gate: Mutex::new(()),
            flag: AtomicBool::new(false),
        }
    }

    /// Clear the interrupt flag.
    pub fn reset(&self) {
        trace!("ThreadInterrupt::reset");
        self.flag.store(false, Ordering::Release);
    }

    /// Set the interrupt flag and wake every waiter.
    pub fn invoke(&self) {
        trace!("ThreadInterrupt::invoke");
        {
            let _guard = self.gate.lock();
            self.flag.store(true, Ordering::Release);
        }
        self.cond.notify_all();
    }

    /// Sleep for `rel_time`, returning `false` if interrupted first.
    ///
    /// The API mirrors the C++ original:
    /// * `true`  → full timeout elapsed  
    /// * `false` → interrupted
    pub fn sleep_for(&self, rel_time: StdDuration) -> bool {
        debug!("ThreadInterrupt::sleep_for — {:?}", rel_time);

        // Quick exit if already interrupted.
        if self.as_bool() {
            debug!("ThreadInterrupt::sleep_for — immediate interrupt");
            return false;
        }

        let deadline = Instant::now() + rel_time;
        let mut guard = self.gate.lock();

        loop {
            // Interrupt check with the mutex held.
            if self.as_bool() {
                return false;
            }

            // Timeout check *without* waiting.
            let now = Instant::now();
            if now >= deadline {
                return true; // waited the entire period
            }

            // Wait for the remaining period or until notified.
            let remaining = deadline - now;
            guard = self.cond.wait_for(guard, remaining);
            // On spurious wake‑up or notify, loop back and re‑test.
        }
    }
}

/// ---------------- Tests
#[cfg(test)]
mod thread_interrupt_timing_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration as StdDuration, Instant};

    /// Verify that an uninterrupted sleep reaches its timeout.
    #[traced_test]
    fn uninterrupted_sleep_times_out() {
        let ti = ThreadInterrupt::new();
        let start = Instant::now();

        let timed_out = ti.sleep_for(StdDuration::from_millis(150));
        let elapsed   = start.elapsed();

        assert!(timed_out, "must return true on timeout");
        assert!(
            elapsed >= StdDuration::from_millis(140),
            "elapsed {:?} unexpectedly short",
            elapsed
        );
    }

    /// Verify that an interrupt wakes sleepers early.
    #[traced_test]
    fn interrupt_wakes_early() {
        let ti      = Arc::new(ThreadInterrupt::new());
        let sleeper = Arc::clone(&ti);

        let handle = thread::spawn(move || sleeper.sleep_for(StdDuration::from_secs(10)));

        // Give the spawned thread time to block.
        std::thread::sleep(StdDuration::from_millis(50));
        ti.invoke();

        let timed_out = handle.join().expect("sleeping thread panicked");
        assert!(
            !timed_out,
            "`sleep_for` should return false after interrupt"
        );
    }
}
