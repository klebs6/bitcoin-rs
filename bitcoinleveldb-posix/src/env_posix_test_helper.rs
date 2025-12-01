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

      Must be called before creating an Env.
    */
    pub fn set_read_only_fd_limit(limit: i32) {
        info!(
            "EnvPosixTestHelper::set_read_only_fd_limit: requested limit={}",
            limit
        );

        #[cfg(debug_assertions)]
        {
            debug!(
                "EnvPosixTestHelper::set_read_only_fd_limit: \
                 asserting PosixDefaultEnv has not been initialized yet"
            );
            PosixDefaultEnv::AssertEnvNotInitialized();
        }

        unsafe {
            debug!(
                "EnvPosixTestHelper::set_read_only_fd_limit: \
                 updating g_open_read_only_file_limit from {} to {}",
                g_open_read_only_file_limit,
                limit
            );
            g_open_read_only_file_limit = limit;
        }
    }

    /**
      Set the maximum number of read-only files that will be mapped via mmap.

      Must be called before creating an Env.
    */
    pub fn set_read_only_mmap_limit(limit: i32) {
        info!(
            "EnvPosixTestHelper::set_read_only_mmap_limit: requested limit={}",
            limit
        );

        #[cfg(debug_assertions)]
        {
            debug!(
                "EnvPosixTestHelper::set_read_only_mmap_limit: \
                 asserting PosixDefaultEnv has not been initialized yet"
            );
            PosixDefaultEnv::AssertEnvNotInitialized();
        }

        unsafe {
            debug!(
                "EnvPosixTestHelper::set_read_only_mmap_limit: \
                 updating g_mmap_limit from {} to {}",
                g_mmap_limit,
                limit
            );
            g_mmap_limit = limit;
        }
    }
}
