/*!
  | This test uses a custom Env to keep track of
  | the state of a filesystem as of the last
  | "sync". It then checks for data loss errors by
  | purposely dropping file data (or entire files)
  | not protected by a "sync".
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/fault_injection_test.cc]

const VALUE_SIZE:     i32 = 1000;
const MAX_NUM_VALUES: i32 = 2000;
const NUM_ITERATIONS: usize = 3;

/**
  | Assume a filename, and not a directory
  | name like "/foo/bar/"
  |
  */
fn get_dir_name(filename: &String) -> String {
    
    todo!();
        /*
            size_t found = filename.find_last_of("/\\");
      if (found == std::string::npos) {
        return "";
      } else {
        return filename.substr(0, found);
      }
        */
}

fn sync_dir(dir: &String) -> crate::Status {
    
    todo!();
        /*
            // As this is a test it isn't required to *actually* sync this directory.
      return Status::OK();
        */
}

/**
  | A basic file truncation function suitable
  | for this test.
  |
  */
fn truncate(
        filename: &String,
        length:   u64) -> crate::Status {
    
    todo!();
        /*
            leveldb::Env* env = leveldb::Env::Default();

      SequentialFile* orig_file;
      Status s = env->NewSequentialFile(filename, &orig_file);
      if (!s.ok()) return s;

      char* scratch = new char[length];
      leveldb::Slice result;
      s = orig_file->Read(length, &result, scratch);
      delete orig_file;
      if (s.ok()) {
        std::string tmp_name = GetDirName(filename) + "/truncate.tmp";
        WritableFile* tmp_file;
        s = env->NewWritableFile(tmp_name, &tmp_file);
        if (s.ok()) {
          s = tmp_file->Append(result);
          delete tmp_file;
          if (s.ok()) {
            s = env->RenameFile(tmp_name, filename);
          } else {
            env->DeleteFile(tmp_name);
          }
        }
      }

      delete[] scratch;

      return s;
        */
}

struct FileState {
    filename:          String,
    pos:               i64,
    pos_at_last_sync:  i64,
    pos_at_last_flush: i64,
}

impl Default for FileState {
    
    fn default() -> Self {
        todo!();
        /*
        : pos(-1),
        : pos_at_last_sync(-1),
        : pos_at_last_flush(-1),
        */
    }
}

impl FileState {

    pub fn new(filename: &String) -> Self {
    
        todo!();
        /*

            : filename_(filename),
            pos_(-1),
            pos_at_last_sync_(-1),
            pos_at_last_flush_(-1)
        */
    }
    
    pub fn is_fully_synced(&self) -> bool {
        
        todo!();
        /*
            return pos_ <= 0 || pos_ == pos_at_last_sync_;
        */
    }
}

/**
  | A wrapper around WritableFile which
  | informs another Env whenever this file
  | is written to or sync'ed.
  |
  */
struct TestWritableFile {
    state:                FileState,
    target:               *mut dyn WritableFile,
    writable_file_opened: bool,
    env:                  *mut FaultInjectionTestEnv,
}

impl WritableFile for TestWritableFile {

}

impl GetName for TestWritableFile {
    
    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "";
        */
    }
}

///-------------------------
struct FaultInjectionTestEnv {
    base:  EnvWrapper,
    mutex: Mutex<fault_injection_test_env::Inner>,
}

mod fault_injection_test_env {

    use super::*;

    pub struct Inner {

        db_file_state:                 HashMap<String,FileState>,
        new_files_since_last_dir_sync: HashSet<String>,

        /**
          | Record flushes, syncs, writes
          |
          */
        filesystem_active:             bool,
    }
}

impl Default for FaultInjectionTestEnv {
    
    fn default() -> Self {
        todo!();
        /*
            : EnvWrapper(Env::Default()), filesystem_active_(true)
        */
    }
}

impl FaultInjectionTestEnv {

    /**
      | Setting the filesystem to inactive is the
      | test equivalent to simulating a system
      | reset. Setting to inactive will freeze our
      | saved filesystem state so that it will stop
      | being recorded. It can then be reset back to
      | the state at the time of the reset.
      */
    #[LOCKS_EXCLUDED(mutex_)]
    pub fn is_filesystem_active(&mut self) -> bool {
        
        todo!();
        /*
            MutexLock l(&mutex_);
        return filesystem_active_;
        */
    }

    #[LOCKS_EXCLUDED(mutex_)]
    pub fn set_filesystem_active(&mut self, active: bool)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
        filesystem_active_ = active;
        */
    }
}

impl Drop for TestWritableFile {
    fn drop(&mut self) {
        todo!();
        /*
            if (writable_file_opened_) {
        Close();
      }
      delete target_;
        */
    }
}

impl TestWritableFile {

    pub fn new(
        state: &FileState,
        f:     *mut dyn WritableFile,
        env:   *mut FaultInjectionTestEnv) -> Self {
    
        todo!();
        /*


            : state_(state), target_(f), writable_file_opened_(true), env_(env) 
      assert(f != nullptr);
        */
    }
}

impl WritableFileAppend for TestWritableFile {
    
    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            Status s = target_->Append(data);
      if (s.ok() && env_->IsFilesystemActive()) {
        state_.pos_ += data.size();
      }
      return s;
        */
    }
}
    
impl WritableFileClose for TestWritableFile {

    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            writable_file_opened_ = false;
      Status s = target_->Close();
      if (s.ok()) {
        env_->WritableFileClosed(state_);
      }
      return s;
        */
    }
}
    
impl WritableFileFlush for TestWritableFile {

    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            Status s = target_->Flush();
      if (s.ok() && env_->IsFilesystemActive()) {
        state_.pos_at_last_flush_ = state_.pos_;
      }
      return s;
        */
    }
}

impl WritableFileSync for TestWritableFile {

    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            if (!env_->IsFilesystemActive()) {
        return Status::OK();
      }
      // Ensure new files referred to by the manifest are in the filesystem.
      Status s = target_->Sync();
      if (s.ok()) {
        state_.pos_at_last_sync_ = state_.pos_;
      }
      if (env_->IsFileCreatedSinceLastDirSync(state_.filename_)) {
        Status ps = SyncParent();
        if (s.ok() && !ps.ok()) {
          s = ps;
        }
      }
      return s;
        */
    }
}

impl TestWritableFile {

    fn sync_parent(&mut self) -> crate::Status {
        
        todo!();
        /*
            Status s = SyncDir(GetDirName(state_.filename_));
      if (s.ok()) {
        env_->DirWasSynced();
      }
      return s;
        */
    }
}

impl FaultInjectionTestEnv {

    pub fn new_writable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            WritableFile* actual_writable_file;
      Status s = target()->NewWritableFile(fname, &actual_writable_file);
      if (s.ok()) {
        FileState state(fname);
        state.pos_ = 0;
        *result = new TestWritableFile(state, actual_writable_file, this);
        // NewWritableFile doesn't append to files, so if the same file is
        // opened again then it will be truncated - so forget our saved
        // state.
        UntrackFile(fname);
        MutexLock l(&mutex_);
        new_files_since_last_dir_sync_.insert(fname);
      }
      return s;
        */
    }
    
    pub fn new_appendable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            WritableFile* actual_writable_file;
      Status s = target()->NewAppendableFile(fname, &actual_writable_file);
      if (s.ok()) {
        FileState state(fname);
        state.pos_ = 0;
        {
          MutexLock l(&mutex_);
          if (db_file_state_.count(fname) == 0) {
            new_files_since_last_dir_sync_.insert(fname);
          } else {
            state = db_file_state_[fname];
          }
        }
        *result = new TestWritableFile(state, actual_writable_file, this);
      }
      return s;
        */
    }
    
    pub fn drop_unsynced_file_data(&mut self) -> crate::Status {
        
        todo!();
        /*
            Status s;
      MutexLock l(&mutex_);
      for (const auto& kvp : db_file_state_) {
        if (!s.ok()) {
          break;
        }
        const FileState& state = kvp.second;
        if (!state.IsFullySynced()) {
          s = state.DropUnsyncedData();
        }
      }
      return s;
        */
    }
    
    pub fn dir_was_synced(&mut self)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      new_files_since_last_dir_sync_.clear();
        */
    }
    
    pub fn is_file_created_since_last_dir_sync(&mut self, filename: &String) -> bool {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      return new_files_since_last_dir_sync_.find(filename) !=
             new_files_since_last_dir_sync_.end();
        */
    }
    
    pub fn untrack_file(&mut self, f: &String)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      db_file_state_.erase(f);
      new_files_since_last_dir_sync_.erase(f);
        */
    }
    
    pub fn delete_file(&mut self, f: &String) -> crate::Status {
        
        todo!();
        /*
            Status s = EnvWrapper::DeleteFile(f);
      ASSERT_OK(s);
      if (s.ok()) {
        UntrackFile(f);
      }
      return s;
        */
    }
    
    pub fn rename_file(&mut self, 
        s: &String,
        t: &String) -> crate::Status {
        
        todo!();
        /*
            Status ret = EnvWrapper::RenameFile(s, t);

      if (ret.ok()) {
        MutexLock l(&mutex_);
        if (db_file_state_.find(s) != db_file_state_.end()) {
          db_file_state_[t] = db_file_state_[s];
          db_file_state_.erase(s);
        }

        if (new_files_since_last_dir_sync_.erase(s) != 0) {
          assert(new_files_since_last_dir_sync_.find(t) ==
                 new_files_since_last_dir_sync_.end());
          new_files_since_last_dir_sync_.insert(t);
        }
      }

      return ret;
        */
    }
    
    pub fn reset_state(&mut self)  {
        
        todo!();
        /*
            // Since we are not destroying the database, the existing files
      // should keep their recorded synced/flushed state. Therefore
      // we do not reset db_file_state_ and new_files_since_last_dir_sync_.
      SetFilesystemActive(true);
        */
    }
    
    pub fn delete_files_created_after_last_dir_sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            // Because DeleteFile access this container make a copy to avoid deadlock
      mutex_.Lock();
      std::set<std::string> new_files(new_files_since_last_dir_sync_.begin(),
                                      new_files_since_last_dir_sync_.end());
      mutex_.Unlock();
      Status status;
      for (const auto& new_file : new_files) {
        Status delete_status = DeleteFile(new_file);
        if (!delete_status.ok() && status.ok()) {
          status = std::move(delete_status);
        }
      }
      return status;
        */
    }
    
    pub fn writable_file_closed(&mut self, state: &FileState)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      db_file_state_[state.filename_] = state;
        */
    }
}

impl FileState {
    
    pub fn drop_unsynced_data(&self) -> crate::Status {
        
        todo!();
        /*
            int64_t sync_pos = pos_at_last_sync_ == -1 ? 0 : pos_at_last_sync_;
      return Truncate(filename_, sync_pos);
        */
    }
}

///-------------------
struct FaultInjectionTest {
    env:        *mut FaultInjectionTestEnv,
    dbname:     String,
    tiny_cache: *mut Cache,
    options:    Options,
    db:         *mut dyn DB,
}

enum ExpectedVerifResult { 
    VAL_EXPECT_NO_ERROR, 
    VAL_EXPECT_ERROR 
}

enum ResetMethod { 
    RESET_DROP_UNSYNCED_DATA, 
    RESET_DELETE_UNSYNCED_FILES 
}

impl Default for FaultInjectionTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(new FaultInjectionTestEnv),
            tiny_cache_(NewLRUCache(100)),
            db_(nullptr) 

        dbname_ = test::TmpDir() + "/fault_test";
        DestroyDB(dbname_, Options());  // Destroy any db from earlier run
        options_.reuse_logs = true;
        options_.env = env_;
        options_.paranoid_checks = true;
        options_.block_cache = tiny_cache_;
        options_.create_if_missing = true;
        */
    }
}

impl Drop for FaultInjectionTest {
    fn drop(&mut self) {
        todo!();
        /*
            CloseDB();
        DestroyDB(dbname_, Options());
        delete tiny_cache_;
        delete env_;
        */
    }
}

impl FaultInjectionTest {

    pub fn reuse_logs(&mut self, reuse: bool)  {
        
        todo!();
        /*
            options_.reuse_logs = reuse;
        */
    }
    
    pub fn build(&mut self, 
        start_idx: i32,
        num_vals:  i32)  {
        
        todo!();
        /*
            std::string key_space, value_space;
        WriteBatch batch;
        for (int i = start_idx; i < start_idx + num_vals; i++) {
          Slice key = Key(i, &key_space);
          batch.Clear();
          batch.Put(key, Value(i, &value_space));
          WriteOptions options;
          ASSERT_OK(db_->Write(options, &batch));
        }
        */
    }
    
    pub fn read_value(&self, 
        i:   i32,
        val: *mut String) -> crate::Status {
        
        todo!();
        /*
            std::string key_space, value_space;
        Slice key = Key(i, &key_space);
        Value(i, &value_space);
        ReadOptions options;
        return db_->Get(options, key, val);
        */
    }
    
    pub fn verify(&self, 
        start_idx: i32,
        num_vals:  i32,
        expected:  ExpectedVerifResult) -> crate::Status {
        
        todo!();
        /*
            std::string val;
        std::string value_space;
        Status s;
        for (int i = start_idx; i < start_idx + num_vals && s.ok(); i++) {
          Value(i, &value_space);
          s = ReadValue(i, &val);
          if (expected == VAL_EXPECT_NO_ERROR) {
            if (s.ok()) {
              ASSERT_EQ(value_space, val);
            }
          } else if (s.ok()) {
            fprintf(stderr, "Expected an error at %d, but was OK\n", i);
            s = Status::IOError(dbname_, "Expected value error:");
          } else {
            s = Status::OK();  // An expected error
          }
        }
        return s;
        */
    }

    /**
      | Return the ith key
      |
      */
    pub fn key(&self, 
        i:       i32,
        storage: *mut String) -> Slice {
        
        todo!();
        /*
            char buf[100];
        snprintf(buf, sizeof(buf), "%016d", i);
        storage->assign(buf, strlen(buf));
        return Slice(*storage);
        */
    }

    /**
      | Return the value to associate with the
      | specified key
      |
      */
    pub fn value(&self, 
        k:       i32,
        storage: *mut String) -> Slice {
        
        todo!();
        /*
            Random r(k);
        return test::RandomString(&r, kValueSize, storage);
        */
    }
    
    pub fn opendb(&mut self) -> crate::Status {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        env_->ResetState();
        return DB::Open(options_, dbname_, &db_);
        */
    }
    
    pub fn closedb(&mut self)  {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        */
    }
    
    pub fn delete_all_data(&mut self)  {
        
        todo!();
        /*
            Iterator* iter = db_->NewIterator(ReadOptions());
        for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
          ASSERT_OK(db_->Delete(WriteOptions(), iter->key()));
        }

        delete iter;
        */
    }
    
    pub fn reset_db_state(&mut self, reset_method: ResetMethod)  {
        
        todo!();
        /*
            switch (reset_method) {
          case RESET_DROP_UNSYNCED_DATA:
            ASSERT_OK(env_->DropUnsyncedFileData());
            break;
          case RESET_DELETE_UNSYNCED_FILES:
            ASSERT_OK(env_->DeleteFilesCreatedAfterLastDirSync());
            break;
          default:
            assert(false);
        }
        */
    }
    
    pub fn partial_compact_test_pre_fault(&mut self, 
        num_pre_sync:  i32,
        num_post_sync: i32)  {
        
        todo!();
        /*
            DeleteAllData();
        Build(0, num_pre_sync);
        db_->CompactRange(nullptr, nullptr);
        Build(num_pre_sync, num_post_sync);
        */
    }
    
    pub fn partial_compact_test_reopen_with_fault(&mut self, 
        reset_method:  ResetMethod,
        num_pre_sync:  i32,
        num_post_sync: i32)  {
        
        todo!();
        /*
            env_->SetFilesystemActive(false);
        CloseDB();
        ResetDBState(reset_method);
        ASSERT_OK(OpenDB());
        ASSERT_OK(Verify(0, num_pre_sync, FaultInjectionTest::VAL_EXPECT_NO_ERROR));
        ASSERT_OK(Verify(num_pre_sync, num_post_sync,
                         FaultInjectionTest::VAL_EXPECT_ERROR));
        */
    }
    
    pub fn no_write_test_pre_fault(&mut self)  { }
    
    pub fn no_write_test_reopen_with_fault(&mut self, reset_method: ResetMethod)  {
        
        todo!();
        /*
            CloseDB();
        ResetDBState(reset_method);
        ASSERT_OK(OpenDB());
        */
    }
    
    pub fn do_test(&mut self)  {
        
        todo!();
        /*
            Random rnd(0);
        ASSERT_OK(OpenDB());
        for (size_t idx = 0; idx < kNumIterations; idx++) {
          int num_pre_sync = rnd.Uniform(kMaxNumValues);
          int num_post_sync = rnd.Uniform(kMaxNumValues);

          PartialCompactTestPreFault(num_pre_sync, num_post_sync);
          PartialCompactTestReopenWithFault(RESET_DROP_UNSYNCED_DATA, num_pre_sync,
                                            num_post_sync);

          NoWriteTestPreFault();
          NoWriteTestReopenWithFault(RESET_DROP_UNSYNCED_DATA);

          PartialCompactTestPreFault(num_pre_sync, num_post_sync);
          // No new files created so we expect all values since no files will be
          // dropped.
          PartialCompactTestReopenWithFault(RESET_DELETE_UNSYNCED_FILES,
                                            num_pre_sync + num_post_sync, 0);

          NoWriteTestPreFault();
          NoWriteTestReopenWithFault(RESET_DELETE_UNSYNCED_FILES);
        }
        */
    }
}

#[test] fn fault_injection_test_no_log_reuse() {
    todo!();
    /*
    
      ReuseLogs(false);
      DoTest();

    */
}

#[test] fn fault_injection_test_with_log_reuse() {
    todo!();
    /*
    
      ReuseLogs(true);
      DoTest();

    */
}

fn dbfault_injection_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
