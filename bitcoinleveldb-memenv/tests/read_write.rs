// ---------------- [ File: bitcoinleveldb-memenv/tests/read_write.rs ]
use bitcoinleveldb_memenv::*;
use bitcoin_imports::*;

#[test] fn mem_env_test_read_write() {
    todo!();
    /*
    
      WritableFile* writable_file;
      SequentialFile* seq_file;
      RandomAccessFile* rand_file;
      Slice result;
      char scratch[100];

      ASSERT_OK(env_->CreateDir("/dir"));

      ASSERT_OK(env_->NewWritableFile("/dir/f", &writable_file));
      ASSERT_OK(writable_file->Append("hello "));
      ASSERT_OK(writable_file->Append("world"));
      delete writable_file;

      // Read sequentially.
      ASSERT_OK(env_->NewSequentialFile("/dir/f", &seq_file));
      ASSERT_OK(seq_file->Read(5, &result, scratch));  // Read "hello".
      ASSERT_EQ(0, result.compare("hello"));
      ASSERT_OK(seq_file->Skip(1));
      ASSERT_OK(seq_file->Read(1000, &result, scratch));  // Read "world".
      ASSERT_EQ(0, result.compare("world"));
      ASSERT_OK(seq_file->Read(1000, &result, scratch));  // Try reading past EOF.
      ASSERT_EQ(0, result.size());
      ASSERT_OK(seq_file->Skip(100));  // Try to skip past end of file.
      ASSERT_OK(seq_file->Read(1000, &result, scratch));
      ASSERT_EQ(0, result.size());
      delete seq_file;

      // Random reads.
      ASSERT_OK(env_->NewRandomAccessFile("/dir/f", &rand_file));
      ASSERT_OK(rand_file->Read(6, 5, &result, scratch));  // Read "world".
      ASSERT_EQ(0, result.compare("world"));
      ASSERT_OK(rand_file->Read(0, 5, &result, scratch));  // Read "hello".
      ASSERT_EQ(0, result.compare("hello"));
      ASSERT_OK(rand_file->Read(10, 100, &result, scratch));  // Read "d".
      ASSERT_EQ(0, result.compare("d"));

      // Too high offset.
      ASSERT_TRUE(!rand_file->Read(1000, 5, &result, scratch).ok());
      delete rand_file;

    */
}
