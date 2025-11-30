// ---------------- [ File: bitcoinleveldb-posixenv/tests/start_thread.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_test_start_thread() {
    trace!("env_test_start_thread: start");

    let test_env = EnvTest::default();
    let env_rc = test_env.env().clone();

    let state = State::new(0, 3);

    {
        let mut env = env_rc.borrow_mut();
        for _ in 0..3 {
            env.start_thread(thread_body, &state as *const _ as *mut c_void);
        }
    }

    let mut inner = state
        .mu()
        .lock()
        .expect("env_test_start_thread: state mutex poisoned");

    while inner.num_running != 0 {
        trace!(
            num_running = inner.num_running,
            "env_test_start_thread: waiting for all threads to decrement num_running"
        );
        inner = state
            .cvar()
            .wait(inner)
            .expect("env_test_start_thread: Condvar wait poisoned");
    }

    assert_eq!(inner.val, 3, "Expected three thread callbacks");

    info!("env_test_start_thread: completed");
}
