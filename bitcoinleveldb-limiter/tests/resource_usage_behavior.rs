// ---------------- [ File: bitcoinleveldb-limiter/tests/resource_usage_behavior.rs ]
use bitcoinleveldb_limiter::*;
use bitcoin_imports::*;

use std::sync::{Arc, Mutex};
use std::thread;

// -------------------------------------------------------------
//  Basic capacity semantics
// -------------------------------------------------------------
#[traced_test]
fn acquire_succeeds_up_to_capacity_then_fails() {

    let max = 3;
    let limiter = Limiter::new(max);

    for _ in 0..max {
        assert!(
            limiter.acquire(),
            "acquire should succeed while capacity is available"
        );
    }

    assert!(
        !limiter.acquire(),
        "acquire should fail once capacity is exhausted"
    );
}

// -------------------------------------------------------------
//  Zero-capacity limiter never grants resources
// -------------------------------------------------------------
#[traced_test]
fn zero_capacity_never_allows_acquire() {

    let limiter = Limiter::new(0);

    assert!(
        !limiter.acquire(),
        "zero-capacity limiter must not grant resources"
    );
    assert!(
        !limiter.acquire(),
        "subsequent acquires on zero-capacity limiter must still fail"
    );
}

// -------------------------------------------------------------
//  Release returns capacity back to the limiter
// -------------------------------------------------------------
#[traced_test]
fn release_restores_capacity() {

    let max = 2;
    let limiter = Limiter::new(max);

    assert!(limiter.acquire(), "first acquire should succeed");
    assert!(limiter.acquire(), "second acquire should succeed");
    assert!(
        !limiter.acquire(),
        "third acquire should fail with exhausted capacity"
    );

    limiter.release();

    assert!(
        limiter.acquire(),
        "acquire should succeed again after a release"
    );
    assert!(
        !limiter.acquire(),
        "capacity should again be exhausted after re-acquire"
    );
}

// -------------------------------------------------------------
//  Concurrent acquire/release never exceeds capacity
// -------------------------------------------------------------
#[traced_test]
fn concurrent_acquire_does_not_exceed_capacity() {

    const MAX: i32 = 4;
    const THREADS: usize = 16;
    const ITERATIONS_PER_THREAD: usize = 1_000;

    let limiter = Arc::new(Limiter::new(MAX));

    // Tracks how many threads are inside the critical section
    let active = Arc::new(Mutex::new(0_i32));
    // Counts any observed over-capacity violations
    let violations = Arc::new(Mutex::new(0_u32));
    // Ensure we actually exercised the limiter
    let total_acquired = Arc::new(Mutex::new(0_i32));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let limiter = Arc::clone(&limiter);
        let active = Arc::clone(&active);
        let violations = Arc::clone(&violations);
        let total_acquired = Arc::clone(&total_acquired);

        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS_PER_THREAD {
                if limiter.acquire() {
                    {
                        let mut a = active.lock().unwrap();
                        *a += 1;
                        if *a > MAX {
                            let mut v = violations.lock().unwrap();
                            *v += 1;
                        }
                    }

                    // Encourage interleaving between threads
                    thread::yield_now();

                    {
                        let mut a = active.lock().unwrap();
                        *a -= 1;
                    }

                    limiter.release();

                    let mut t = total_acquired.lock().unwrap();
                    *t += 1;
                } else {
                    // Back off briefly when no capacity is available
                    thread::yield_now();
                }
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread join");
    }

    assert_eq!(
        *violations.lock().unwrap(),
        0,
        "no more than MAX concurrent acquisitions should be observed"
    );

    assert!(
        *total_acquired.lock().unwrap() > 0,
        "expected at least some successful acquires under contention"
    );
}
