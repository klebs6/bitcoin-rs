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
            PosixDefaultEnv::assert_env_not_initialized();
        }

        unsafe {
            debug!(
                "EnvPosixTestHelper::set_read_only_fd_limit: \
                 updating OPEN_READ_ONLY_FILE_LIMIT from {} to {}",
                OPEN_READ_ONLY_FILE_LIMIT.load(atomic::Ordering::SeqCst),
                limit
            );
            OPEN_READ_ONLY_FILE_LIMIT.store(limit.into(),atomic::Ordering::SeqCst);
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
            PosixDefaultEnv::assert_env_not_initialized();
        }

        unsafe {
            debug!(
                "EnvPosixTestHelper::set_read_only_mmap_limit: \
                 updating mmap_limit from {} to {}",
                MMAP_LIMIT.load(atomic::Ordering::SeqCst),
                limit
            );
            MMAP_LIMIT.store(limit.into(),atomic::Ordering::SeqCst);
        }
    }
}
