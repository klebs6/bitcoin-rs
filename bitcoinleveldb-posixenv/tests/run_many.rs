// ---------------- [ File: bitcoinleveldb-posixenv/tests/run_many.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_run_many() {
    use std::sync::{Condvar, Mutex};

    trace!("env_test_run_many: start");

    struct RunManyState {
        mu:   Mutex<i32>,
        cvar: Condvar,
    }

    struct Callback {
        state: *mut RunManyState,
        id:    i32,
    }

    fn callback_run(arg: *mut c_void) -> c_void {
        trace!(?arg, "callback_run: invoked");

        assert!(
            !arg.is_null(),
            "callback_run: received null argument pointer"
        );

        unsafe {
            let cb = &*(arg as *mut Callback);
            let state = &*cb.state;

            let mut last_id = state
                .mu
                .lock()
                .expect("callback_run: state mutex poisoned");

            trace!(
                current_last_id = *last_id,
                callback_id     = cb.id,
                "callback_run: acquired mutex"
            );

            assert_eq!(
                *last_id,
                cb.id - 1,
                "Callbacks should be executed in order"
            );

            *last_id = cb.id;
            debug!(
                new_last_id = *last_id,
                "callback_run: updated last_id, notifying waiters"
            );
            state.cvar.notify_all();
        }

        // Return an opaque `c_void` value.
        unsafe { std::mem::zeroed() }
    }

    let test_env = EnvTest::default();
    let env_rc = test_env.env().clone();

    let state = RunManyState {
        mu:   Mutex::new(0),
        cvar: Condvar::new(),
    };

    let mut callback1 = Callback {
        state: &state as *const _ as *mut RunManyState,
        id:    1,
    };
    let mut callback2 = Callback {
        state: &state as *const _ as *mut RunManyState,
        id:    2,
    };
    let mut callback3 = Callback {
        state: &state as *const _ as *mut RunManyState,
        id:    3,
    };
    let mut callback4 = Callback {
        state: &state as *const _ as *mut RunManyState,
        id:    4,
    };

    {
        let mut env = env_rc.borrow_mut();
        env.schedule(callback_run, &mut callback1 as *mut _ as *mut c_void);
        env.schedule(callback_run, &mut callback2 as *mut _ as *mut c_void);
        env.schedule(callback_run, &mut callback3 as *mut _ as *mut c_void);
        env.schedule(callback_run, &mut callback4 as *mut _ as *mut c_void);
    }

    let mut last_id_guard = state
        .mu
        .lock()
        .expect("env_test_run_many: state mutex poisoned");

    while *last_id_guard != 4 {
        trace!(
            last_id = *last_id_guard,
            "env_test_run_many: waiting for all callbacks to run"
        );
        last_id_guard = state
            .cvar
            .wait(last_id_guard)
            .expect("env_test_run_many: Condvar wait poisoned");
    }

    info!("env_test_run_many: completed");
}
