// ---------------- [ File: bitcoinleveldbt-faultinjection/tests/fault_injection.rs ]
/*!
  | This test uses a custom Env to keep track of
  | the state of a filesystem as of the last
  | "sync". It then checks for data loss errors by
  | purposely dropping file data (or entire files)
  | not protected by a "sync".
  */
use bitcoinleveldbt_faultinjection::*;
use traced_test::*;
use tracing_setup::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/fault_injection_test.cc]
#[traced_test]
fn fault_injection_test_no_log_reuse() {
    let mut t = FaultInjectionTest::default();
    t.reuse_logs(false);
    t.do_test();
}

#[traced_test]
fn fault_injection_test_with_log_reuse() {
    let mut t = FaultInjectionTest::default();
    t.reuse_logs(true);
    t.do_test();
}
