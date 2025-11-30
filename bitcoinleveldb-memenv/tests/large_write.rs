// ---------------- [ File: bitcoinleveldb-memenv/tests/large_write.rs ]
use bitcoinleveldb_memenv::*;
use bitcoin_imports::*;

#[test] fn mem_env_test_large_write() {
    todo!();
    /*
    
      const size_t kWriteSize = 300 * 1024;
      char* scratch = new char[kWriteSize * 2];

      std::string write_data;
      for (size_t i = 0; i < kWriteSize; ++i) {
        write_data.append(1, static_cast<char>(i));
      }

      WritableFile* writable_file;
      ASSERT_OK(env_->NewWritableFile("/dir/f", &writable_file));
      ASSERT_OK(writable_file->Append("foo"));
      ASSERT_OK(writable_file->Append(write_data));
      delete writable_file;

      SequentialFile* seq_file;
      Slice result;
      ASSERT_OK(env_->NewSequentialFile("/dir/f", &seq_file));
      ASSERT_OK(seq_file->Read(3, &result, scratch));  // Read "foo".
      ASSERT_EQ(0, result.compare("foo"));

      size_t read = 0;
      std::string read_data;
      while (read < kWriteSize) {
        ASSERT_OK(seq_file->Read(kWriteSize - read, &result, scratch));
        read_data.append(result.data(), result.size());
        read += result.size();
      }
      ASSERT_TRUE(write_data == read_data);
      delete seq_file;
      delete[] scratch;

    */
}
