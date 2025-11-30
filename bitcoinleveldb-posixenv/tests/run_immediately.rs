// ---------------- [ File: bitcoinleveldb-posixenv/tests/run_immediately.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_run_immediately() {
    use std::sync::{Condvar, Mutex};

    trace!("env_test_run_immediately: start");

    struct RunState {
        mu:   Mutex<bool>,
        cvar: Condvar,
    }

    fn run_state_run(arg: *mut c_void) -> c_void {
        trace!(?arg, "RunState::Run invoked");

        assert!(
            !arg.is_null(),
            "RunState::Run: received null argument pointer"
        );

        unsafe {
            let state = &*(arg as *mut RunState);
            let mut called = state
                .mu
                .lock()
                .expect("RunState::Run: state mutex poisoned");

            assert_eq!(
                *called,
                false,
                "RunState::Run should be called exactly once"
            );

            *called = true;

            debug!(
                called = *called,
                "RunState::Run: state updated, notifying waiters"
            );

            state.cvar.notify_all();
        }

        // Model the `c_void` return type by returning an opaque,
        // never-inspected value.
        unsafe { std::mem::zeroed() }
    }

    let test_env = EnvTest::default();
    let env_rc = test_env.env().clone();

    let state = RunState {
        mu:   Mutex::new(false),
        cvar: Condvar::new(),
    };

    // Schedule callback.
    {
        let mut env = env_rc.borrow_mut();
        env.schedule(run_state_run, &state as *const _ as *mut c_void);
    }

    // Wait for callback to run.
    let mut called_guard = state
        .mu
        .lock()
        .expect("env_test_run_immediately: state mutex poisoned");

    while !*called_guard {
        trace!("env_test_run_immediately: waiting for callback to fire");
        called_guard = state
            .cvar
            .wait(called_guard)
            .expect("env_test_run_immediately: Condvar wait poisoned");
    }

    info!("env_test_run_immediately: completed");
}
