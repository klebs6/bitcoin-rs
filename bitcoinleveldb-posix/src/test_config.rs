// ---------------- [ File: bitcoinleveldb-posix/src/test_config.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix_test.cc]

/**
  | Exit codes for the helper process spawned by
  | TestCloseOnExec* tests.
  |
  | Useful for debugging test failures.
  */
#[cfg(HAVE_O_CLOEXEC)]
pub const TEXT_CLOSE_ON_EXEC_HELPER_EXEC_FAILED_CODE:   i32 = 61;

#[cfg(HAVE_O_CLOEXEC)]
pub const TEXT_CLOSE_ON_EXEC_HELPER_DUP_2FAILED_CODE:   i32 = 62;

#[cfg(HAVE_O_CLOEXEC)]
pub const TEXT_CLOSE_ON_EXEC_HELPER_FOUND_OPEN_FD_CODE: i32 = 63;

/**
   Command-line switch used to run this test as
   the CloseOnExecSwitch helper.
  */
#[cfg(HAVE_O_CLOEXEC)]
pub const TEST_CLOSE_ON_EXEC_SWITCH: &str = "--test-close-on-exec-helper";

pub const TEST_READ_ONLY_FILE_LIMIT: i32 = 4;
pub const TEST_MMAP_LIMIT:           i32 = 4;
