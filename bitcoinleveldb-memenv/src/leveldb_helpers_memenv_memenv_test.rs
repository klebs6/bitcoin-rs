// ---------------- [ File: bitcoinleveldb-memenv/src/leveldb_helpers_memenv_memenv_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv_test.cc]

struct MemEnvTest {
    env: Rc<RefCell<dyn Env>>,
}

impl Default for MemEnvTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(NewMemEnv(Env::Default()))
        */
    }
}

impl Drop for MemEnvTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete env_;
        */
    }
}

#[test] fn mem_env_test_basics() {
    todo!();
    /*
    
      uint64_t file_size;
      WritableFile* writable_file;
      std::vector<std::string> children;

      ASSERT_OK(env_->CreateDir("/dir"));

      // Check that the directory is empty.
      ASSERT_TRUE(!env_->FileExists("/dir/non_existent"));
      ASSERT_TRUE(!env_->GetFileSize("/dir/non_existent", &file_size).ok());
      ASSERT_OK(env_->GetChildren("/dir", &children));
      ASSERT_EQ(0, children.size());

      // Create a file.
      ASSERT_OK(env_->NewWritableFile("/dir/f", &writable_file));
      ASSERT_OK(env_->GetFileSize("/dir/f", &file_size));
      ASSERT_EQ(0, file_size);
      delete writable_file;

      // Check that the file exists.
      ASSERT_TRUE(env_->FileExists("/dir/f"));
      ASSERT_OK(env_->GetFileSize("/dir/f", &file_size));
      ASSERT_EQ(0, file_size);
      ASSERT_OK(env_->GetChildren("/dir", &children));
      ASSERT_EQ(1, children.size());
      ASSERT_EQ("f", children[0]);

      // Write to the file.
      ASSERT_OK(env_->NewWritableFile("/dir/f", &writable_file));
      ASSERT_OK(writable_file->Append("abc"));
      delete writable_file;

      // Check that append works.
      ASSERT_OK(env_->NewAppendableFile("/dir/f", &writable_file));
      ASSERT_OK(env_->GetFileSize("/dir/f", &file_size));
      ASSERT_EQ(3, file_size);
      ASSERT_OK(writable_file->Append("hello"));
      delete writable_file;

      // Check for expected size.
      ASSERT_OK(env_->GetFileSize("/dir/f", &file_size));
      ASSERT_EQ(8, file_size);

      // Check that renaming works.
      ASSERT_TRUE(!env_->RenameFile("/dir/non_existent", "/dir/g").ok());
      ASSERT_OK(env_->RenameFile("/dir/f", "/dir/g"));
      ASSERT_TRUE(!env_->FileExists("/dir/f"));
      ASSERT_TRUE(env_->FileExists("/dir/g"));
      ASSERT_OK(env_->GetFileSize("/dir/g", &file_size));
      ASSERT_EQ(8, file_size);

      // Check that opening non-existent file fails.
      SequentialFile* seq_file;
      RandomAccessFile* rand_file;
      ASSERT_TRUE(!env_->NewSequentialFile("/dir/non_existent", &seq_file).ok());
      ASSERT_TRUE(!seq_file);
      ASSERT_TRUE(!env_->NewRandomAccessFile("/dir/non_existent", &rand_file).ok());
      ASSERT_TRUE(!rand_file);

      // Check that deleting works.
      ASSERT_TRUE(!env_->DeleteFile("/dir/non_existent").ok());
      ASSERT_OK(env_->DeleteFile("/dir/g"));
      ASSERT_TRUE(!env_->FileExists("/dir/g"));
      ASSERT_OK(env_->GetChildren("/dir", &children));
      ASSERT_EQ(0, children.size());
      ASSERT_OK(env_->DeleteDir("/dir"));

    */
}

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

#[test] fn mem_env_test_locks() {
    todo!();
    /*
    
      FileLock* lock;

      // These are no-ops, but we test they return success.
      ASSERT_OK(env_->LockFile("some file", &lock));
      ASSERT_OK(env_->UnlockFile(lock));

    */
}

#[test] fn mem_env_test_misc() {
    todo!();
    /*
    
      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      ASSERT_TRUE(!test_dir.empty());

      WritableFile* writable_file;
      ASSERT_OK(env_->NewWritableFile("/a/b", &writable_file));

      // These are no-ops, but we test they return success.
      ASSERT_OK(writable_file->Sync());
      ASSERT_OK(writable_file->Flush());
      ASSERT_OK(writable_file->Close());
      delete writable_file;

    */
}

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

#[test] fn mem_env_test_db() {
    todo!();
    /*
    
      Options options;
      options.create_if_missing = true;
      options.env = env_;
      DB* db;

      const Slice keys[] = {Slice("aaa"), Slice("bbb"), Slice("ccc")};
      const Slice vals[] = {Slice("foo"), Slice("bar"), Slice("baz")};

      ASSERT_OK(DB::Open(options, "/dir/db", &db));
      for (size_t i = 0; i < 3; ++i) {
        ASSERT_OK(db->Put(WriteOptions(), keys[i], vals[i]));
      }

      for (size_t i = 0; i < 3; ++i) {
        std::string res;
        ASSERT_OK(db->Get(ReadOptions(), keys[i], &res));
        ASSERT_TRUE(res == vals[i]);
      }

      Iterator* iterator = db->NewIterator(ReadOptions());
      iterator->SeekToFirst();
      for (size_t i = 0; i < 3; ++i) {
        ASSERT_TRUE(iterator->Valid());
        ASSERT_TRUE(keys[i] == iterator->key());
        ASSERT_TRUE(vals[i] == iterator->value());
        iterator->Next();
      }
      ASSERT_TRUE(!iterator->Valid());
      delete iterator;

      DBImpl* dbi = reinterpret_cast<DBImpl*>(db);
      ASSERT_OK(dbi->TEST_CompactMemTable());

      for (size_t i = 0; i < 3; ++i) {
        std::string res;
        ASSERT_OK(db->Get(ReadOptions(), keys[i], &res));
        ASSERT_TRUE(res == vals[i]);
      }

      delete db;

    */
}

fn testleveldb_helpers_memenv_memenv_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
