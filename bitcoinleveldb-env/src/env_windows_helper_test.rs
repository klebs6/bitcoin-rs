// ---------------- [ File: bitcoinleveldb-env/src/env_windows_helper_test.rs ]
crate::ix!();

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
    pub fn set_read_only_mmap_limit(limit: i32)  {
        
        todo!();
        /*
        
        */
    }
}
