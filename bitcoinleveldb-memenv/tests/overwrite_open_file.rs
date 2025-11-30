// ---------------- [ File: bitcoinleveldb-memenv/tests/overwrite_open_file.rs ]
use bitcoinleveldb_memenv::*;
use bitcoin_imports::*;

#[test] fn mem_env_test_overwrite_open_file() {
    todo!();
    /*
    
      const char kWrite1Data[] = "Write #1 data";
      const size_t kFileDataLen = sizeof(kWrite1Data) - 1;
      const std::string kTestFileName = test::TmpDir() + "/leveldb-TestFile.dat";

      ASSERT_OK(WriteStringToFile(env_, kWrite1Data, kTestFileName));

      RandomAccessFile* rand_file;
      ASSERT_OK(env_->NewRandomAccessFile(kTestFileName, &rand_file));

      const char kWrite2Data[] = "Write #2 data";
      ASSERT_OK(WriteStringToFile(env_, kWrite2Data, kTestFileName));

      // Verify that overwriting an open file will result in the new file data
      // being read from files opened before the write.
      Slice result;
      char scratch[kFileDataLen];
      ASSERT_OK(rand_file->Read(0, kFileDataLen, &result, scratch));
      ASSERT_EQ(0, result.compare(kWrite2Data));

      delete rand_file;

    */
}
