// ---------------- [ File: bitcoinleveldb-posix/src/env_posix_test_helper.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix_test_helper.h]

/**
  A helper for the POSIX Env to facilitate testing.
*/
pub struct EnvPosixTestHelper;

impl EnvPosixTestHelper {
    /**
      Set the maximum number of read-only files that will be opened.

      In the original C++ implementation this adjusted internal global
      limits before creating the Env instance. In this Rust port the
      limits are configured inside the POSIX Env implementation, so this
      helper currently only logs the requested value.
    */
    pub fn set_read_only_fd_limit(limit: i32) {
        info!(
            "EnvPosixTestHelper::set_read_only_fd_limit: requested limit={}",
            limit
        );
    }

    /**
      Set the maximum number of read-only files that will be mapped via mmap.

      As with set_read_only_fd_limit, the Rust port configures limits
      inside the POSIX Env implementation; this helper logs the requested
      value for transparency.
    */
    pub fn set_read_only_mmap_limit(limit: i32) {
        info!(
            "EnvPosixTestHelper::set_read_only_mmap_limit: requested limit={}",
            limit
        );
    }
}

/**
   A helper for the POSIX Env to facilitate
   testing.
  */
pub struct EnvPosixTestHelper {

}

impl EnvPosixTestHelper {

    /**
      | Set the maximum number of read-only files
      | that will be opened.
      |
      | Must be called before creating an Env.
      */
    pub fn set_read_only_fd_limit(&mut self, limit: i32)  {
        
        todo!();
        /*
            PosixDefaultEnv::AssertEnvNotInitialized();
      g_open_read_only_file_limit = limit;
        */
    }
    
    /**
      | Set the maximum number of read-only files
      | that will be mapped via mmap.
      |
      | Must be called before creating an Env.
      */
    pub fn set_read_only_mmap_limit(&mut self, limit: i32)  {
        
        todo!();
        /*
            PosixDefaultEnv::AssertEnvNotInitialized();
      g_mmap_limit = limit;
        */
    }
}
