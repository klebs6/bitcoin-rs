// ---------------- [ File: bitcoinleveldb-env/tests/env_windows_helper.rs ]
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;
use bitcoinleveldb_log::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_windows_test_helper.h]

/**
  | A helper for the Windows Env to facilitate
  | testing.
  |
  */
struct EnvWindowsTestHelper {

}

impl EnvWindowsTestHelper {
    /**
      | Set the maximum number of read-only files
      | that will be mapped via mmap.
      |
      | Must be called before creating an Env.
      */
    pub fn set_read_only_mmap_limit(limit: i32) {
        use std::sync::atomic::{AtomicI32, Ordering};

        lazy_static! {
            static ref READ_ONLY_MMAP_LIMIT: AtomicI32 = AtomicI32::new(0);
        }

        info!(
            limit,
            "EnvWindowsTestHelper::set_read_only_mmap_limit: recording limit"
        );
        READ_ONLY_MMAP_LIMIT.store(limit, Ordering::SeqCst);
    }
}
