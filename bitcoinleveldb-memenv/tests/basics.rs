// ---------------- [ File: bitcoinleveldb-memenv/tests/basics.rs ]
use bitcoinleveldb_memenv::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv_test.cc]

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
