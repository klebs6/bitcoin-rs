// ---------------- [ File: bitcoinleveldb-env/tests/env_windows.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_windows_test.cc]

const MMAP_LIMIT: i32 = 4;

struct EnvWindowsTest {
    env: Rc<RefCell<dyn Env>>,
}

impl Default for EnvWindowsTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(Env::Default())
        */
    }
}

impl EnvWindowsTest {

    pub fn set_file_limits(mmap_limit: i32)  {
        
        todo!();
        /*
            EnvWindowsTestHelper::SetReadOnlyMMapLimit(mmap_limit);
        */
    }
}

#[test] fn env_windows_test_open_on_read() {
    todo!();
    /*
    
      // Write some test data to a single file that will be opened |n| times.
      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string test_file = test_dir + "/open_on_read.txt";

      FILE* f = fopen(test_file.c_str(), "w");
      ASSERT_TRUE(f != nullptr);
      const char kFileData[] = "abcdefghijklmnopqrstuvwxyz";
      fputs(kFileData, f);
      fclose(f);

      // Open test file some number above the sum of the two limits to force
      // leveldb::WindowsEnv to switch from mapping the file into memory
      // to basic file reading.
      const int kNumFiles = kMMapLimit + 5;
      leveldb::RandomAccessFile* files[kNumFiles] = {0};
      for (int i = 0; i < kNumFiles; i++) {
        ASSERT_OK(env_->NewRandomAccessFile(test_file, &files[i]));
      }
      char scratch;
      Slice read_result;
      for (int i = 0; i < kNumFiles; i++) {
        ASSERT_OK(files[i]->Read(i, 1, &read_result, &scratch));
        ASSERT_EQ(kFileData[i], read_result[0]);
      }
      for (int i = 0; i < kNumFiles; i++) {
        delete files[i];
      }
      ASSERT_OK(env_->DeleteFile(test_file));

    */
}

fn testenv_windows_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            // All tests currently run with the same read-only file limits.
      leveldb::EnvWindowsTest::SetFileLimits(leveldb::kMMapLimit);
      return leveldb::test::RunAllTests();
        */
}
