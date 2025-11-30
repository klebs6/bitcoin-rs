use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_time_now_micros_and_sleep() {
    trace!("env_time_now_micros_and_sleep: start");

    let test_env = EnvTest::default();
    let env_rc   = test_env.env().clone();

    {
        let mut env = env_rc.borrow_mut();

        let t1 = env.now_micros();
        env.sleep_for_microseconds(2000);
        let t2 = env.now_micros();

        assert!(
            t2 >= t1,
            "now_micros should be non-decreasing (t1={}, t2={})",
            t1,
            t2
        );

        // Non-positive durations should be a fast no-op.
        env.sleep_for_microseconds(0);
        env.sleep_for_microseconds(-100);
    }

    info!("env_time_now_micros_and_sleep: completed");
}
