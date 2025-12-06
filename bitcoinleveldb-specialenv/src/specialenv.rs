crate::ix!();

/**
  | Special Env used to delay background
  | operations.
  |
  */
pub struct SpecialEnv {

    base:                 crate::EnvWrapper,

    /**
      | sstable/log Sync() calls are blocked
      | while this pointer is non-null.
      |
      */
    delay_data_sync:      AtomicBool,

    /**
      | sstable/log Sync() calls return an
      | error.
      |
      */
    data_sync_error:      AtomicBool,

    /**
      | Simulate no-space errors while this
      | pointer is non-null.
      |
      */
    no_space:             AtomicBool,

    /**
      | Simulate non-writable file system
      | while this pointer is non-null.
      |
      */
    non_writable:         AtomicBool,

    /**
      | Force sync of manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_sync_error:  AtomicBool,

    /**
      | Force write to manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_write_error: AtomicBool,

    count_random_reads:   bool,
    random_read_counter:  AtomicCounter,
}

impl SpecialEnv {

    pub fn new(base: Rc<RefCell<dyn crate::Env>>) -> Self {
    
        todo!();
        /*
        : env_wrapper(base),
        : delay_data_sync(false),
        : data_sync_error(false),
        : no_space(false),
        : non_writable(false),
        : manifest_sync_error(false),
        : manifest_write_error(false),
        : count_random_reads(false),

        
        */
    }
    
    pub fn new_writable_file(&mut self, 
        f: &String,
        r: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            class DataFile : public WritableFile {
         
          SpecialEnv* const env_;
          WritableFile* const base_;

         
          DataFile(SpecialEnv* env, WritableFile* base) : env_(env), base_(base) {}
          ~DataFile() { delete base_; }
          Status Append(const Slice& data) {
            if (env_->no_space_.load(std::memory_order_acquire)) {
              // Drop writes on the floor
              return Status::OK();
            } else {
              return base_->Append(data);
            }
          }
          Status Close() { return base_->Close(); }
          Status Flush() { return base_->Flush(); }
          Status Sync() {
            if (env_->data_sync_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated data sync error");
            }
            while (env_->delay_data_sync_.load(std::memory_order_acquire)) {
              DelayMilliseconds(100);
            }
            return base_->Sync();
          }
          std::string GetName() const override { return ""; }
        };
        class ManifestFile : public WritableFile {
         
          SpecialEnv* env_;
          WritableFile* base_;

         
          ManifestFile(SpecialEnv* env, WritableFile* b) : env_(env), base_(b) {}
          ~ManifestFile() { delete base_; }
          Status Append(const Slice& data) {
            if (env_->manifest_write_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated writer error");
            } else {
              return base_->Append(data);
            }
          }
          Status Close() { return base_->Close(); }
          Status Flush() { return base_->Flush(); }
          Status Sync() {
            if (env_->manifest_sync_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated sync error");
            } else {
              return base_->Sync();
            }
          }
          std::string GetName() const override { return ""; }
        };

        if (non_writable_.load(std::memory_order_acquire)) {
          return Status::IOError("simulated write error");
        }

        Status s = target()->NewWritableFile(f, r);
        if (s.ok()) {
          if (strstr(f.c_str(), ".ldb") != nullptr ||
              strstr(f.c_str(), ".log") != nullptr) {
            *r = new DataFile(this, *r);
          } else if (strstr(f.c_str(), "MANIFEST") != nullptr) {
            *r = new ManifestFile(this, *r);
          }
        }
        return s;
        */
    }
    
    pub fn new_random_access_file(&mut self, 
        f: &String,
        r: *mut *mut dyn RandomAccessFile) -> crate::Status {
        
        todo!();
        /*
            class CountingFile : public RandomAccessFile {
         
          RandomAccessFile* target_;
          AtomicCounter* counter_;

         
          CountingFile(RandomAccessFile* target, AtomicCounter* counter)
              : target_(target), counter_(counter) {}
          ~CountingFile() override { delete target_; }
          Status Read(uint64_t offset, size_t n, Slice* result,
                      char* scratch) const override {
            counter_->Increment();
            return target_->Read(offset, n, result, scratch);
          }
          std::string GetName() const override { return ""; }
        };

        Status s = target()->NewRandomAccessFile(f, r);
        if (s.ok() && count_random_reads_) {
          *r = new CountingFile(*r, &random_read_counter_);
        }
        return s;
        */
    }
}
