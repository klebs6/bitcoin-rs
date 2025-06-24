// ---------------- [ File: bitcoin-sync/src/thread.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/thread.h]
//-------------------------------------------[.cpp/bitcoin/src/util/thread.cpp]

/// Wrapper that logs a thread’s start/stop and
/// propagates panics exactly like Bitcoin Core’s
/// `TraceThread`.
pub fn trace_thread<F>(thread_name: &str, thread_func: F)
where
    F: FnOnce() + Send + 'static,
{
    // note: pass the variable as a field, not as the span name
    let span = span!(tracing::Level::INFO, "thread", name = %thread_name);
    let _enter = span.enter();

    info!("{} thread start", thread_name);
    if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(thread_func)) {
        error!("{} thread panic: {:?}", thread_name, e);
        std::panic::resume_unwind(e);
    }
    info!("{} thread exit", thread_name);
}

#[macro_export]
macro_rules! launch_traced_thread {
    ($name:expr, $closure:expr) => {{
        std::thread::Builder::new()
            .name($name.into())
            .spawn(|| $crate::thread::trace_thread($name, $closure))
            .expect("failed to spawn traced thread")
    }};
}

pub struct WaitTimedOut(pub bool);

impl WaitTimedOut {
    pub fn timed_out(&self) -> bool { self.0 }
}

/// Blocking wait‑until helper (wrapper around parking_lot).
pub fn wait_until<T: ?Sized, P>(
    cv: &parking_lot::Condvar,
    guard: &mut parking_lot::MutexGuard<'_, T>,
    deadline: std::time::Instant,
    mut predicate: P,
) -> bool
where
    P: FnMut() -> bool,
{
    while !predicate() {
        if cv.wait_until(guard, deadline).timed_out() {
            return predicate();
        }
    }
    true
}

#[cfg(test)]
mod thread_tests {
    use super::*;

    use parking_lot::{Condvar, Mutex};
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::{Duration, Instant},
    };

    #[traced_test]
    fn wait_until_expires() {
        // both primitives come from parking_lot ─ matches the signature
        let pair   = Arc::new((parking_lot::Mutex::new(()), parking_lot::Condvar::new()));
        let (m, cv) = &*pair;
        let mut guard = m.lock();

        // predicate never becomes true → must time‑out
        let ok = wait_until(
            cv,
            &mut guard,
            Instant::now() + Duration::from_millis(100),
            || false,
        );
        assert!(!ok, "predicate never became true, should time‑out");
    }

    #[traced_test]
    fn trace_thread_executes_closure() {
        let flag  = Arc::new(AtomicBool::new(false));
        let flag2 = Arc::clone(&flag);

        std::thread::spawn(move || {
            trace_thread("demo", move || flag2.store(true, Ordering::Release));
        })
        .join()
        .unwrap();

        assert!(flag.load(Ordering::Acquire));
    }
}
