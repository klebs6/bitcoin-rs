// ---------------- [ File: bitcoinleveldbt-util/src/run_all_tests.rs ]
crate::ix!();

/**
  | Run some of the tests registered by the TEST()
  | macro.  If the environment variable
  | "LEVELDB_TESTS" is not set, runs all tests.
  |
  | Otherwise, runs only the tests whose name
  | contains the value of "LEVELDB_TESTS" as
  | a substring.  E.g., suppose the tests are:
  |
  |    TEST(Foo, Hello) { ... }
  |    TEST(Foo, World) { ... }
  |
  | LEVELDB_TESTS=Hello will run the first test
  | LEVELDB_TESTS=o     will run both tests
  | LEVELDB_TESTS=Junk  will run no tests
  |
  | Returns 0 if all tests pass.
  |
  | Dies or returns a non-zero value if some test
  | fails.
  */
pub fn run_all_tests() -> i32 {
    trace!(
        target: "bitcoinleveldbt_util::harness",
        event = "run_all_tests_entry"
    );

    let matcher: Option<String> = unsafe {
        let p = libc::getenv(b"LEVELDB_TESTS\0".as_ptr() as *const c_char);
        if p.is_null() {
            None
        } else {
            Some(CStr::from_ptr(p).to_string_lossy().into_owned())
        }
    };

    let tests_snapshot: Vec<Test> = {
        let guard = BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS.lock();
        guard.clone()
    };

    let mut num: i32 = 0;

    for t in tests_snapshot.iter() {
        let mut full_name = t.base().clone();
        full_name.push('.');
        full_name.push_str(t.name().as_str());

        match matcher.as_ref() {
            Some(m) => {
                if !full_name.contains(m.as_str()) {
                    trace!(
                        target: "bitcoinleveldbt_util::harness",
                        event = "run_all_tests_skip",
                        test_name = %full_name,
                        matcher = %m
                    );
                    continue;
                }
            }
            None => {}
        }

        eprintln!("==== Test {}.{}", t.base(), t.name());

        trace!(
            target: "bitcoinleveldbt_util::harness",
            event = "run_all_tests_invoke",
            test_name = %full_name
        );

        (t.func())();
        num += 1;
    }

    eprintln!("==== PASSED {} tests", num);

    trace!(
        target: "bitcoinleveldbt_util::harness",
        event = "run_all_tests_exit",
        executed = num
    );

    0
}
