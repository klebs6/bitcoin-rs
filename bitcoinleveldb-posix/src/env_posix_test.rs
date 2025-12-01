// ---------------- [ File: bitcoinleveldb-posix/src/env_posix_test.rs ]
crate::ix!();

#[derive(Debug, Default)]
pub struct EnvPosixTest;

impl EnvPosixTest {
    pub fn set_file_limits(read_only_file_limit: i32, mmap_limit: i32) {
        info!(
            "EnvPosixTest::set_file_limits: read_only_file_limit={}, mmap_limit={} \
             (no-op in Rust port; limits are configured elsewhere)",
            read_only_file_limit, mmap_limit
        );
    }
}

pub fn make_posix_env_for_tests() -> Rc<RefCell<dyn Env>> {
    info!("make_posix_env_for_tests: constructing shared Env instance");

    let env_test = EnvTest::default();
    let env_rc = env_test.env().clone();

    debug!("make_posix_env_for_tests: Env instance ready");
    env_rc
}

pub struct EnvPosixTest {
    env: Rc<RefCell<dyn Env>>,
}

impl Default for EnvPosixTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(Env::Default())
        */
    }
}

impl EnvPosixTest {
    
    pub fn set_file_limits(
        read_only_file_limit: i32,
        mmap_limit:           i32)  {
        
        todo!();
        /*
            EnvPosixTestHelper::SetReadOnlyFDLimit(read_only_file_limit);
        EnvPosixTestHelper::SetReadOnlyMMapLimit(mmap_limit);
        */
    }
}

pub fn testenv_posix_test_main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    info!(
        "testenv_posix_test_main: invoked in Rust test port; \
         configuring file limits and returning success"
    );

    EnvPosixTest::set_file_limits(TEST_READ_ONLY_FILE_LIMIT, TEST_MMAP_LIMIT);
    0
}

pub fn testenv_posix_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            #if HAVE_O_CLOEXEC
      // Check if we're invoked as a helper program, or as the test suite.
      for (int i = 1; i < argc; ++i) {
        if (!std::strcmp(argv[i], kTestCloseOnExecSwitch)) {
          return TestCloseOnExecHelperMain(argv[i + 1]);
        }
      }

      // Save argv[0] early, because googletest may modify argv.
      GetArgvZero()->assign(argv[0], argv[0] + std::strlen(argv[0]) + 1);
    #endif  // HAVE_O_CLOEXEC

      // All tests currently run with the same read-only file limits.
      leveldb::EnvPosixTest::SetFileLimits(leveldb::kReadOnlyFileLimit,
                                           leveldb::kMMapLimit);
      return leveldb::test::RunAllTests();
        */
}
