// ---------------- [ File: bitcoinleveldb-version/src/set.rs ]
crate::ix!();

///------------------
pub struct VersionSet {

    env:                  Box<dyn Env>,
    dbname:               String,
    options:              *const Options,
    table_cache:          *const TableCache,
    icmp:                 InternalKeyComparator,
    next_file_number:     u64,
    manifest_file_number: u64,
    last_sequence:        u64,
    log_number:           u64,

    /**
      | 0 or backing store for memtable being
      | compacted
      |
      */
    prev_log_number:      u64,

    /**
      | Opened lazily
      |
      */
    descriptor_file:      *mut dyn WritableFile,

    descriptor_log:       *mut LogWriter,

    /**
      | Head of circular doubly-linked list
      | of versions.
      |
      */
    dummy_versions:       Version,

    /**
      | == dummy_versions_.prev_
      |
      */
    current:              *mut Version,

    /**
      | Per-level key at which the next compaction
      | at that level should start.
      | 
      | Either an empty string, or a valid
      | 
      | InternalKey.
      |
      */
    compact_pointer:      [String; NUM_LEVELS],
}

impl Drop for VersionSet {

    fn drop(&mut self) {
        todo!();
        /*
            current_->Unref();
      assert(dummy_versions_.next_ == &dummy_versions_);  // List must be empty
      delete descriptor_log_;
      delete descriptor_file_;
        */
    }
}

impl VersionSet {

    /**
      | Return the current version.
      |
      */
    pub fn current(&self) -> *mut Version {
        
        todo!();
        /*
            return current_;
        */
    }

    /**
      | Return the current manifest file number
      |
      */
    pub fn manifest_file_number(&self) -> u64 {
        
        todo!();
        /*
            return manifest_file_number_;
        */
    }

    /**
      | Allocate and return a new file number
      |
      */
    pub fn new_file_number(&mut self) -> u64 {
        
        todo!();
        /*
            return next_file_number_++;
        */
    }

    /**
      | Arrange to reuse "file_number" unless a newer
      | file number has already been allocated.
      |
      | REQUIRES: "file_number" was returned by
      | a call to NewFileNumber().
      */
    pub fn reuse_file_number(&mut self, file_number: u64)  {
        
        todo!();
        /*
            if (next_file_number_ == file_number + 1) {
          next_file_number_ = file_number;
        }
        */
    }

    /**
      | Return the last sequence number.
      |
      */
    pub fn last_sequence(&self) -> u64 {
        
        todo!();
        /*
            return last_sequence_;
        */
    }

    /**
      | Set the last sequence number to s.
      |
      */
    pub fn set_last_sequence(&mut self, s: u64)  {
        
        todo!();
        /*
            assert(s >= last_sequence_);
        last_sequence_ = s;
        */
    }

    /**
      | Return the current log file number.
      |
      */
    pub fn log_number(&self) -> u64 {
        
        todo!();
        /*
            return log_number_;
        */
    }

    /**
      | Return the log file number for the log
      | file that is currently being compacted,
      | or zero if there is no such log file.
      |
      */
    pub fn prev_log_number(&self) -> u64 {
        
        todo!();
        /*
            return prev_log_number_;
        */
    }

    /**
      | Returns true iff some level needs a compaction.
      |
      */
    pub fn needs_compaction(&self) -> bool {
        
        todo!();
        /*
            Version* v = current_;
        return (v->compaction_score_ >= 1) || (v->file_to_compact_ != nullptr);
        */
    }
    
    pub fn new(
        dbname:      &String,
        options:     *const Options,
        table_cache: *mut TableCache,
        cmp:         *const InternalKeyComparator) -> Self {
    
        todo!();
        /*


            : env_(options->env),
          dbname_(dbname),
          options_(options),
          table_cache_(table_cache),
          icmp_(*cmp),
          next_file_number_(2),
          manifest_file_number_(0),  // Filled by Recover()
          last_sequence_(0),
          log_number_(0),
          prev_log_number_(0),
          descriptor_file_(nullptr),
          descriptor_log_(nullptr),
          dummy_versions_(this),
          current_(nullptr) 
      AppendVersion(new Version(this));
        */
    }
    
    pub fn append_version(&mut self, v: *mut Version)  {
        
        todo!();
        /*
            // Make "v" current
      assert(v->refs_ == 0);
      assert(v != current_);
      if (current_ != nullptr) {
        current_->Unref();
      }
      current_ = v;
      v->Ref();

      // Append to linked list
      v->prev_ = dummy_versions_.prev_;
      v->next_ = &dummy_versions_;
      v->prev_->next_ = v;
      v->next_->prev_ = v;
        */
    }
    
    /**
      | Apply *edit to the current version to form
      | a new descriptor that is both saved to
      | persistent state and installed as the new
      | current version.  Will release *mu while
      | actually writing to the file.
      |
      | REQUIRES: *mu is held on entry.
      |
      | REQUIRES: no other thread concurrently calls
      | LogAndApply()
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mu)]
    pub fn log_and_apply(&mut self, 
        edit: *mut VersionEdit,
        mu:   *mut RawMutex) -> Status {
        
        todo!();
        /*
            if (edit->has_log_number_) {
        assert(edit->log_number_ >= log_number_);
        assert(edit->log_number_ < next_file_number_);
      } else {
        edit->SetLogNumber(log_number_);
      }

      if (!edit->has_prev_log_number_) {
        edit->SetPrevLogNumber(prev_log_number_);
      }

      edit->SetNextFile(next_file_number_);
      edit->SetLastSequence(last_sequence_);

      Version* v = new Version(this);
      {
        Builder builder(this, current_);
        builder.Apply(edit);
        builder.SaveTo(v);
      }
      Finalize(v);

      // Initialize new descriptor log file if necessary by creating
      // a temporary file that contains a snapshot of the current version.
      std::string new_manifest_file;
      Status s;
      if (descriptor_log_ == nullptr) {
        // No reason to unlock *mu here since we only hit this path in the
        // first call to LogAndApply (when opening the database).
        assert(descriptor_file_ == nullptr);
        new_manifest_file = DescriptorFileName(dbname_, manifest_file_number_);
        edit->SetNextFile(next_file_number_);
        s = env_->NewWritableFile(new_manifest_file, &descriptor_file_);
        if (s.ok()) {
          descriptor_log_ = new LogWriter(descriptor_file_);
          s = WriteSnapshot(descriptor_log_);
        }
      }

      // Unlock during expensive MANIFEST log write
      {
        mu->Unlock();

        // Write new record to MANIFEST log
        if (s.ok()) {
          std::string record;
          edit->EncodeTo(&record);
          s = descriptor_log_->AddRecord(record);
          if (s.ok()) {
            s = descriptor_file_->Sync();
          }
          if (!s.ok()) {
            Log(options_->info_log, "MANIFEST write: %s\n", s.ToString().c_str());
          }
        }

        // If we just created a new descriptor file, install it by writing a
        // new CURRENT file that points to it.
        if (s.ok() && !new_manifest_file.empty()) {
          s = SetCurrentFile(env_, dbname_, manifest_file_number_);
        }

        mu->Lock();
      }

      // Install the new version
      if (s.ok()) {
        AppendVersion(v);
        log_number_ = edit->log_number_;
        prev_log_number_ = edit->prev_log_number_;
      } else {
        delete v;
        if (!new_manifest_file.empty()) {
          delete descriptor_log_;
          delete descriptor_file_;
          descriptor_log_ = nullptr;
          descriptor_file_ = nullptr;
          env_->DeleteFile(new_manifest_file);
        }
      }

      return s;
        */
    }
    
    /**
      | Recover the last saved descriptor from
      | persistent storage.
      |
      */
    pub fn recover(&mut self, save_manifest: *mut bool) -> Status {
        
        todo!();
        /*
            struct LogReporter : public LogReader::Reporter {
        Status* status;
        c_void Corruption(size_t bytes, const Status& s) override {
          if (this->status->ok()) *this->status = s;
        }
      };

      // Read "CURRENT" file, which contains a pointer to the current manifest file
      std::string current;
      Status s = ReadFileToString(env_, CurrentFileName(dbname_), &current);
      if (!s.ok()) {
        return s;
      }
      if (current.empty() || current[current.size() - 1] != '\n') {
        return Status::Corruption("CURRENT file does not end with newline");
      }
      current.resize(current.size() - 1);

      std::string dscname = dbname_ + "/" + current;
      SequentialFile* file;
      s = env_->NewSequentialFile(dscname, &file);
      if (!s.ok()) {
        if (s.IsNotFound()) {
          return Status::Corruption("CURRENT points to a non-existent file",
                                    s.ToString());
        }
        return s;
      }

      bool have_log_number = false;
      bool have_prev_log_number = false;
      bool have_next_file = false;
      bool have_last_sequence = false;
      uint64_t next_file = 0;
      uint64_t last_sequence = 0;
      uint64_t log_number = 0;
      uint64_t prev_log_number = 0;
      Builder builder(this, current_);

      {
        LogReporter reporter;
        reporter.status = &s;
        LogReader reader(file, &reporter, true /*checksum*/,
                           0 /*initial_offset*/);
        Slice record;
        std::string scratch;
        while (reader.ReadRecord(&record, &scratch) && s.ok()) {
          VersionEdit edit;
          s = edit.DecodeFrom(record);
          if (s.ok()) {
            if (edit.has_comparator_ &&
                edit.comparator_ != icmp_.user_comparator()->Name()) {
              s = Status::InvalidArgument(
                  edit.comparator_ + " does not match existing comparator ",
                  icmp_.user_comparator()->Name());
            }
          }

          if (s.ok()) {
            builder.Apply(&edit);
          }

          if (edit.has_log_number_) {
            log_number = edit.log_number_;
            have_log_number = true;
          }

          if (edit.has_prev_log_number_) {
            prev_log_number = edit.prev_log_number_;
            have_prev_log_number = true;
          }

          if (edit.has_next_file_number_) {
            next_file = edit.next_file_number_;
            have_next_file = true;
          }

          if (edit.has_last_sequence_) {
            last_sequence = edit.last_sequence_;
            have_last_sequence = true;
          }
        }
      }
      delete file;
      file = nullptr;

      if (s.ok()) {
        if (!have_next_file) {
          s = Status::Corruption("no meta-nextfile entry in descriptor");
        } else if (!have_log_number) {
          s = Status::Corruption("no meta-lognumber entry in descriptor");
        } else if (!have_last_sequence) {
          s = Status::Corruption("no last-sequence-number entry in descriptor");
        }

        if (!have_prev_log_number) {
          prev_log_number = 0;
        }

        MarkFileNumberUsed(prev_log_number);
        MarkFileNumberUsed(log_number);
      }

      if (s.ok()) {
        Version* v = new Version(this);
        builder.SaveTo(v);
        // Install recovered version
        Finalize(v);
        AppendVersion(v);
        manifest_file_number_ = next_file;
        next_file_number_ = next_file + 1;
        last_sequence_ = last_sequence;
        log_number_ = log_number;
        prev_log_number_ = prev_log_number;

        // See if we can reuse the existing MANIFEST file.
        if (ReuseManifest(dscname, current)) {
          // No need to save new manifest
        } else {
          *save_manifest = true;
        }
      }

      return s;
        */
    }
    
    pub fn reuse_manifest(&mut self, 
        dscname: &String,
        dscbase: &String) -> bool {
        
        todo!();
        /*
            if (!options_->reuse_logs) {
        return false;
      }
      FileType manifest_type;
      uint64_t manifest_number;
      uint64_t manifest_size;
      if (!ParseFileName(dscbase, &manifest_number, &manifest_type) ||
          manifest_type != kDescriptorFile ||
          !env_->GetFileSize(dscname, &manifest_size).ok() ||
          // Make new compacted MANIFEST if old one is too big
          manifest_size >= TargetFileSize(options_)) {
        return false;
      }

      assert(descriptor_file_ == nullptr);
      assert(descriptor_log_ == nullptr);
      Status r = env_->NewAppendableFile(dscname, &descriptor_file_);
      if (!r.ok()) {
        Log(options_->info_log, "Reuse MANIFEST: %s\n", r.ToString().c_str());
        assert(descriptor_file_ == nullptr);
        return false;
      }

      Log(options_->info_log, "Reusing MANIFEST %s\n", dscname.c_str());
      descriptor_log_ = new LogWriter(descriptor_file_, manifest_size);
      manifest_file_number_ = manifest_number;
      return true;
        */
    }
    
    /**
      | Mark the specified file number as used.
      |
      */
    pub fn mark_file_number_used(&mut self, number: u64)  {
        
        todo!();
        /*
            if (next_file_number_ <= number) {
        next_file_number_ = number + 1;
      }
        */
    }
    
    pub fn finalize(&mut self, v: *mut Version)  {
        
        todo!();
        /*
            // Precomputed best level for next compaction
      int best_level = -1;
      double best_score = -1;

      for (int level = 0; level < config::NUM_LEVELS - 1; level++) {
        double score;
        if (level == 0) {
          // We treat level-0 specially by bounding the number of files
          // instead of number of bytes for two reasons:
          //
          // (1) With larger write-buffer sizes, it is nice not to do too
          // many level-0 compactions.
          //
          // (2) The files in level-0 are merged on every read and
          // therefore we wish to avoid too many files when the individual
          // file size is small (perhaps because of a small write-buffer
          // setting, or very high compression ratios, or lots of
          // overwrites/deletions).
          score = v->files_[level].size() /
                  static_cast<double>(config::kL0_CompactionTrigger);
        } else {
          // Compute the ratio of current size to size limit.
          const uint64_t level_bytes = TotalFileSize(v->files_[level]);
          score =
              static_cast<double>(level_bytes) / MaxBytesForLevel(options_, level);
        }

        if (score > best_score) {
          best_level = level;
          best_score = score;
        }
      }

      v->compaction_level_ = best_level;
      v->compaction_score_ = best_score;
        */
    }
    
    /**
      | Save current contents to *log
      |
      */
    pub fn write_snapshot(&mut self, log: *mut LogWriter) -> Status {
        
        todo!();
        /*
            // TODO: Break up into multiple records to reduce memory usage on recovery?

      // Save metadata
      VersionEdit edit;
      edit.SetComparatorName(icmp_.user_comparator()->Name());

      // Save compaction pointers
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        if (!compact_pointer_[level].empty()) {
          InternalKey key;
          key.DecodeFrom(compact_pointer_[level]);
          edit.SetCompactPointer(level, key);
        }
      }

      // Save files
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        const std::vector<FileMetaData*>& files = current_->files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          const FileMetaData* f = files[i];
          edit.AddFile(level, f->number, f->file_size, f->smallest, f->largest);
        }
      }

      std::string record;
      edit.EncodeTo(&record);
      return log->AddRecord(record);
        */
    }
    
    /**
      | Return the number of Table files at the
      | specified level.
      |
      */
    pub fn num_level_files(&self, level: i32) -> i32 {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      return current_->files_[level].size();
        */
    }
    
    pub fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8 {
        
        todo!();
        /*
            // Update code if kNumLevels changes
      const_assert(config::NUM_LEVELS == 7, "");
      snprintf(scratch->buffer, sizeof(scratch->buffer),
               "files[ %d %d %d %d %d %d %d ]", int(current_->files_[0].size()),
               int(current_->files_[1].size()), int(current_->files_[2].size()),
               int(current_->files_[3].size()), int(current_->files_[4].size()),
               int(current_->files_[5].size()), int(current_->files_[6].size()));
      return scratch->buffer;
        */
    }
    
    /**
      | Return the approximate offset in the
      | database of the data for "key" as of version
      | "v".
      |
      */
    pub fn approximate_offset_of(&mut self, 
        v:    *mut Version,
        ikey_: &InternalKey) -> u64 {
        
        todo!();
        /*
            uint64_t result = 0;
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        const std::vector<FileMetaData*>& files = v->files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          if (icmp_.Compare(files[i]->largest, ikey) <= 0) {
            // Entire file is before "ikey", so just add the file size
            result += files[i]->file_size;
          } else if (icmp_.Compare(files[i]->smallest, ikey) > 0) {
            // Entire file is after "ikey", so ignore
            if (level > 0) {
              // Files other than level 0 are sorted by meta->smallest, so
              // no further files in this level will contain data for
              // "ikey".
              break;
            }
          } else {
            // "ikey" falls in the range for this table.  Add the
            // approximate offset of "ikey" within the table.
            Table* tableptr;
            Iterator* iter = table_cache_->NewIterator(
                ReadOptions(), files[i]->number, files[i]->file_size, &tableptr);
            if (tableptr != nullptr) {
              result += tableptr->ApproximateOffsetOf(ikey.Encode());
            }
            delete iter;
          }
        }
      }
      return result;
        */
    }
    
    /**
      | Add all files listed in any live version to
      | *live.
      |
      | May also mutate some internal state.
      */
    pub fn add_live_files(&mut self, live: *mut HashSet<u64>)  {
        
        todo!();
        /*
            for (Version* v = dummy_versions_.next_; v != &dummy_versions_;
           v = v->next_) {
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          const std::vector<FileMetaData*>& files = v->files_[level];
          for (size_t i = 0; i < files.size(); i++) {
            live->insert(files[i]->number);
          }
        }
      }
        */
    }
    
    /**
      | Return the combined file size of all
      | files at the specified level.
      |
      */
    pub fn num_level_bytes(&self, level: i32) -> i64 {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      return TotalFileSize(current_->files_[level]);
        */
    }
    
    /**
      | Return the maximum overlapping data
      | (in bytes) at next level for any file
      | at a level >= 1.
      |
      */
    pub fn max_next_level_overlapping_bytes(&mut self) -> i64 {
        
        todo!();
        /*
            int64_t result = 0;
      std::vector<FileMetaData*> overlaps;
      for (int level = 1; level < config::NUM_LEVELS - 1; level++) {
        for (size_t i = 0; i < current_->files_[level].size(); i++) {
          const FileMetaData* f = current_->files_[level][i];
          current_->GetOverlappingInputs(level + 1, &f->smallest, &f->largest,
                                         &overlaps);
          const int64_t sum = TotalFileSize(overlaps);
          if (sum > result) {
            result = sum;
          }
        }
      }
      return result;
        */
    }

    /**
      | Stores the minimal range that covers all
      | entries in inputs in *smallest, *largest.
      |
      | REQUIRES: inputs is not empty
      */
    pub fn get_range(&mut self, 
        inputs:   &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey)  {
        
        todo!();
        /*
            assert(!inputs.empty());
      smallest->Clear();
      largest->Clear();
      for (size_t i = 0; i < inputs.size(); i++) {
        FileMetaData* f = inputs[i];
        if (i == 0) {
          *smallest = f->smallest;
          *largest = f->largest;
        } else {
          if (icmp_.Compare(f->smallest, *smallest) < 0) {
            *smallest = f->smallest;
          }
          if (icmp_.Compare(f->largest, *largest) > 0) {
            *largest = f->largest;
          }
        }
      }
        */
    }

    /**
      | Stores the minimal range that covers all
      | entries in inputs1 and inputs2 in *smallest,
      | *largest.
      |
      | REQUIRES: inputs is not empty
      */
    pub fn get_range2(&mut self, 
        inputs1:  &Vec<*mut FileMetaData>,
        inputs2:  &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey)  {
        
        todo!();
        /*
            std::vector<FileMetaData*> all = inputs1;
      all.insert(all.end(), inputs2.begin(), inputs2.end());
      GetRange(all, smallest, largest);
        */
    }

    /**
      | Create an iterator that reads over the
      | compaction inputs for "*c".
      |
      | The caller should delete the iterator when no
      | longer needed.
      */
    pub fn make_input_iterator(&mut self, c: *mut Compaction) -> *mut LevelDBIterator {
        
        todo!();
        /*
            ReadOptions options;
      options.verify_checksums = options_->paranoid_checks;
      options.fill_cache = false;

      // Level-0 files have to be merged together.  For other levels,
      // we will make a concatenating iterator per level.
      // TODO(opt): use concatenating iterator for level-0 if there is no overlap
      const int space = (c->level() == 0 ? c->inputs_[0].size() + 1 : 2);
      Iterator** list = new Iterator*[space];
      int num = 0;
      for (int which = 0; which < 2; which++) {
        if (!c->inputs_[which].empty()) {
          if (c->level() + which == 0) {
            const std::vector<FileMetaData*>& files = c->inputs_[which];
            for (size_t i = 0; i < files.size(); i++) {
              list[num++] = table_cache_->NewIterator(options, files[i]->number,
                                                      files[i]->file_size);
            }
          } else {
            // Create concatenating iterator for the files from this level
            list[num++] = NewTwoLevelIterator(
                new Version::LevelFileNumIterator(icmp_, &c->inputs_[which]),
                &GetFileIterator, table_cache_, options);
          }
        }
      }
      assert(num <= space);
      Iterator* result = NewMergingIterator(&icmp_, list, num);
      delete[] list;
      return result;
        */
    }
    
    /**
      | Pick level and inputs for a new compaction.
      |
      | Returns nullptr if there is no compaction to
      | be done.
      |
      | Otherwise returns a pointer to
      | a heap-allocated object that describes the
      | compaction.  Caller should delete the result.
      */
    pub fn pick_compaction(&mut self) -> *mut Compaction {
        
        todo!();
        /*
            Compaction* c;
      int level;

      // We prefer compactions triggered by too much data in a level over
      // the compactions triggered by seeks.
      const bool size_compaction = (current_->compaction_score_ >= 1);
      const bool seek_compaction = (current_->file_to_compact_ != nullptr);
      if (size_compaction) {
        level = current_->compaction_level_;
        assert(level >= 0);
        assert(level + 1 < config::NUM_LEVELS);
        c = new Compaction(options_, level);

        // Pick the first file that comes after compact_pointer_[level]
        for (size_t i = 0; i < current_->files_[level].size(); i++) {
          FileMetaData* f = current_->files_[level][i];
          if (compact_pointer_[level].empty() ||
              icmp_.Compare(f->largest.Encode(), compact_pointer_[level]) > 0) {
            c->inputs_[0].push_back(f);
            break;
          }
        }
        if (c->inputs_[0].empty()) {
          // Wrap-around to the beginning of the key space
          c->inputs_[0].push_back(current_->files_[level][0]);
        }
      } else if (seek_compaction) {
        level = current_->file_to_compact_level_;
        c = new Compaction(options_, level);
        c->inputs_[0].push_back(current_->file_to_compact_);
      } else {
        return nullptr;
      }

      c->input_version_ = current_;
      c->input_version_->Ref();

      // Files in level 0 may overlap each other, so pick up all overlapping ones
      if (level == 0) {
        InternalKey smallest, largest;
        GetRange(c->inputs_[0], &smallest, &largest);
        // Note that the next call will discard the file we placed in
        // c->inputs_[0] earlier and replace it with an overlapping set
        // which will include the picked file.
        current_->GetOverlappingInputs(0, &smallest, &largest, &c->inputs_[0]);
        assert(!c->inputs_[0].empty());
      }

      SetupOtherInputs(c);

      return c;
        */
    }
    
    pub fn setup_other_inputs(&mut self, c: *mut Compaction)  {
        
        todo!();
        /*
            const int level = c->level();
      InternalKey smallest, largest;

      AddBoundaryInputs(icmp_, current_->files_[level], &c->inputs_[0]);
      GetRange(c->inputs_[0], &smallest, &largest);

      current_->GetOverlappingInputs(level + 1, &smallest, &largest,
                                     &c->inputs_[1]);

      // Get entire range covered by compaction
      InternalKey all_start, all_limit;
      GetRange2(c->inputs_[0], c->inputs_[1], &all_start, &all_limit);

      // See if we can grow the number of inputs in "level" without
      // changing the number of "level+1" files we pick up.
      if (!c->inputs_[1].empty()) {
        std::vector<FileMetaData*> expanded0;
        current_->GetOverlappingInputs(level, &all_start, &all_limit, &expanded0);
        AddBoundaryInputs(icmp_, current_->files_[level], &expanded0);
        const int64_t inputs0_size = TotalFileSize(c->inputs_[0]);
        const int64_t inputs1_size = TotalFileSize(c->inputs_[1]);
        const int64_t expanded0_size = TotalFileSize(expanded0);
        if (expanded0.size() > c->inputs_[0].size() &&
            inputs1_size + expanded0_size <
                ExpandedCompactionByteSizeLimit(options_)) {
          InternalKey new_start, new_limit;
          GetRange(expanded0, &new_start, &new_limit);
          std::vector<FileMetaData*> expanded1;
          current_->GetOverlappingInputs(level + 1, &new_start, &new_limit,
                                         &expanded1);
          if (expanded1.size() == c->inputs_[1].size()) {
            Log(options_->info_log,
                "Expanding@%d %d+%d (%ld+%ld bytes) to %d+%d (%ld+%ld bytes)\n",
                level, int(c->inputs_[0].size()), int(c->inputs_[1].size()),
                long(inputs0_size), long(inputs1_size), int(expanded0.size()),
                int(expanded1.size()), long(expanded0_size), long(inputs1_size));
            smallest = new_start;
            largest = new_limit;
            c->inputs_[0] = expanded0;
            c->inputs_[1] = expanded1;
            GetRange2(c->inputs_[0], c->inputs_[1], &all_start, &all_limit);
          }
        }
      }

      // Compute the set of grandparent files that overlap this compaction
      // (parent == level+1; grandparent == level+2)
      if (level + 2 < config::NUM_LEVELS) {
        current_->GetOverlappingInputs(level + 2, &all_start, &all_limit,
                                       &c->grandparents_);
      }

      // Update the place where we will do the next compaction for this level.
      // We update this immediately instead of waiting for the VersionEdit
      // to be applied so that if the compaction fails, we will try a different
      // key range next time.
      compact_pointer_[level] = largest.Encode().ToString();
      c->edit_.SetCompactPointer(level, largest);
        */
    }
    
    /**
      | Return a compaction object for compacting the
      | range [begin,end] in the specified level.
      | Returns nullptr if there is nothing in that
      | level that overlaps the specified range.
      | Caller should delete the result.
      */
    pub fn compact_range(&mut self, 
        level: i32,
        begin: *const InternalKey,
        end:   *const InternalKey) -> *mut Compaction {
        
        todo!();
        /*
            std::vector<FileMetaData*> inputs;
      current_->GetOverlappingInputs(level, begin, end, &inputs);
      if (inputs.empty()) {
        return nullptr;
      }

      // Avoid compacting too much in one shot in case the range is large.
      // But we cannot do this for level-0 since level-0 files can overlap
      // and we must not pick one file and drop another older file if the
      // two files overlap.
      if (level > 0) {
        const uint64_t limit = MaxFileSizeForLevel(options_, level);
        uint64_t total = 0;
        for (size_t i = 0; i < inputs.size(); i++) {
          uint64_t s = inputs[i]->file_size;
          total += s;
          if (total >= limit) {
            inputs.resize(i + 1);
            break;
          }
        }
      }

      Compaction* c = new Compaction(options_, level);
      c->input_version_ = current_;
      c->input_version_->Ref();
      c->inputs_[0] = inputs;
      SetupOtherInputs(c);
      return c;
        */
    }
}

/**
  | Return a human-readable short (single-line)
  | summary of the number of files per level.
  | 
  | Uses *scratch as backing store.
  |
  */
pub struct VersionSetLevelSummaryStorage {
    buffer: [u8; 100],
}
