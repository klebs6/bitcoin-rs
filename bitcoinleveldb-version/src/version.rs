// ---------------- [ File: bitcoinleveldb-version/src/version.rs ]
/*!
  | The representation of a DBImpl consists of
  | a set of Versions. The newest version is
  | called "current". Older versions may be kept
  | around to provide a consistent view to live
  | iterators.
  |
  | Each Version keeps track of a set of Table
  | files per level. The entire set of versions is
  | maintained in a VersionSet.
  |
  | Version,VersionSet are thread-compatible, but
  | require external synchronization on all
  | accesses.
  */

crate::ix!();

pub struct Version {

    /**
      | VersionSet to which this Version belongs
      |
      */
    vset:                  *mut VersionSet,

    /**
      | Next version in linked list
      |
      */
    next:                  *mut Version,

    /**
      | Previous version in linked list
      |
      */
    prev:                  *mut Version,

    /**
      | Number of live refs to this version
      |
      */
    refs:                  i32,

    /**
      | List of files per level
      |
      */
    files:                 [Vec<*mut FileMetaData>; NUM_LEVELS],

    /**
      | Next file to compact based on seek stats.
      |
      */
    file_to_compact:       *mut FileMetaData,

    file_to_compact_level: i32,

    /**
      | Level that should be compacted next
      | and its compaction score. Score < 1 means
      | compaction is not strictly needed.
      | These fields are initialized by Finalize().
      |
      */
    compaction_score:      f64,
    compaction_level:      i32,
}

impl Drop for Version {

    fn drop(&mut self) {
        todo!();
        /*
            assert(refs_ == 0);

      // Remove from linked list
      prev_->next_ = next_;
      next_->prev_ = prev_;

      // Drop references to files
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        for (size_t i = 0; i < files_[level].size(); i++) {
          FileMetaData* f = files_[level][i];
          assert(f->refs > 0);
          f->refs--;
          if (f->refs <= 0) {
            delete f;
          }
        }
      }
        */
    }
}

/**
  | Lookup the value for key.  If found, store it
  | in *val and return OK.  Else return a non-OK
  | status.  Fills *stats.
  |
  | REQUIRES: lock is not held
  */
pub struct VersionGetStats {
    seek_file:       *mut FileMetaData,
    seek_file_level: i32,
}

impl Version {

    pub fn num_files(&self, level: i32) -> i32 {
        
        todo!();
        /*
            return files_[level].size();
        */
    }

    pub fn new(vset: *mut VersionSet) -> Self {
    
        todo!();
        /*
        : vset(vset),
        : next(this),
        : prev(this),
        : refs(0),
        : file_to_compact(nullptr),
        : file_to_compact_level(-1),
        : compaction_score(-1),
        : compaction_level(-1),

        
        */
    }
    
    pub fn new_concatenating_iterator(&self, 
        options: &ReadOptions,
        level:   i32) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return NewTwoLevelIterator(
          new LevelFileNumIterator(vset_->icmp_, &files_[level]), &GetFileIterator,
          vset_->table_cache_, options);
        */
    }
    
    /**
      | Append to *iters a sequence of iterators that
      | will yield the contents of this Version when
      | merged together.
      |
      | REQUIRES: This version has been saved (see
      | VersionSet::SaveTo)
      */
    pub fn add_iterators(&mut self, 
        options: &ReadOptions,
        iters:   *mut Vec<*mut LevelDBIterator>)  {
        
        todo!();
        /*
            // Merge all level zero files together since they may overlap
      for (size_t i = 0; i < files_[0].size(); i++) {
        iters->push_back(vset_->table_cache_->NewIterator(
            options, files_[0][i]->number, files_[0][i]->file_size));
      }

      // For levels > 0, we can use a concatenating iterator that sequentially
      // walks through the non-overlapping files in the level, opening them
      // lazily.
      for (int level = 1; level < config::NUM_LEVELS; level++) {
        if (!files_[level].empty()) {
          iters->push_back(NewConcatenatingIterator(options, level));
        }
      }
        */
    }
    
    /**
      | Call func(arg, level, f) for every file that
      | overlaps user_key in order from newest to
      | oldest.  If an invocation of func returns
      | false, makes no more calls.
      |
      | REQUIRES: user portion of internal_key ==
      | user_key.
      */
    pub fn for_each_overlapping(&mut self, 
        user_key_:     Slice,
        internal_key_: Slice,
        arg:          *mut c_void,
        func:         fn(
                _0: *mut c_void,
                _1: i32,
                _2: *mut FileMetaData
        ) -> bool)  {
        
        todo!();
        /*
            const Comparator* ucmp = vset_->icmp_.user_comparator();

      // Search level-0 in order from newest to oldest.
      std::vector<FileMetaData*> tmp;
      tmp.reserve(files_[0].size());
      for (uint32_t i = 0; i < files_[0].size(); i++) {
        FileMetaData* f = files_[0][i];
        if (ucmp->Compare(user_key, f->smallest.user_key()) >= 0 &&
            ucmp->Compare(user_key, f->largest.user_key()) <= 0) {
          tmp.push_back(f);
        }
      }
      if (!tmp.empty()) {
        std::sort(tmp.begin(), tmp.end(), NewestFirst);
        for (uint32_t i = 0; i < tmp.size(); i++) {
          if (!(*func)(arg, 0, tmp[i])) {
            return;
          }
        }
      }

      // Search other levels.
      for (int level = 1; level < config::NUM_LEVELS; level++) {
        size_t num_files = files_[level].size();
        if (num_files == 0) continue;

        // Binary search to find earliest index whose largest key >= internal_key.
        uint32_t index = FindFile(vset_->icmp_, files_[level], internal_key);
        if (index < num_files) {
          FileMetaData* f = files_[level][index];
          if (ucmp->Compare(user_key, f->smallest.user_key()) < 0) {
            // All of "f" is past any data for user_key
          } else {
            if (!(*func)(arg, level, f)) {
              return;
            }
          }
        }
      }
        */
    }
    
    pub fn get(&mut self, 
        options: &ReadOptions,
        k:       &LookupKey,
        value:   *mut String,
        stats:   *mut VersionGetStats) -> Status {
        
        todo!();
        /*
            stats->seek_file = nullptr;
      stats->seek_file_level = -1;

      struct State {
        Saver saver;
        GetStats* stats;
        const ReadOptions* options;
        Slice ikey;
        FileMetaData* last_file_read;
        int last_file_read_level;

        VersionSet* vset;
        Status s;
        bool found;

        static bool Match(c_void* arg, int level, FileMetaData* f) {
          State* state = reinterpret_cast<State*>(arg);

          if (state->stats->seek_file == nullptr &&
              state->last_file_read != nullptr) {
            // We have had more than one seek for this read.  Charge the 1st file.
            state->stats->seek_file = state->last_file_read;
            state->stats->seek_file_level = state->last_file_read_level;
          }

          state->last_file_read = f;
          state->last_file_read_level = level;

          state->s = state->vset->table_cache_->Get(*state->options, f->number,
                                                    f->file_size, state->ikey,
                                                    &state->saver, SaveValue);
          if (!state->s.ok()) {
            state->found = true;
            return false;
          }
          switch (state->saver.state) {
            case kNotFound:
              return true;  // Keep searching in other files
            case kFound:
              state->found = true;
              return false;
            case kDeleted:
              return false;
            case kCorrupt:
              state->s =
                  Status::Corruption("corrupted key for ", state->saver.user_key);
              state->found = true;
              return false;
          }

          // Not reached. Added to avoid false compilation warnings of
          // "control reaches end of non-c_void function".
          return false;
        }
      };

      State state;
      state.found = false;
      state.stats = stats;
      state.last_file_read = nullptr;
      state.last_file_read_level = -1;

      state.options = &options;
      state.ikey = k.internal_key();
      state.vset = vset_;

      state.saver.state = kNotFound;
      state.saver.ucmp = vset_->icmp_.user_comparator();
      state.saver.user_key = k.user_key();
      state.saver.value = value;

      ForEachOverlapping(state.saver.user_key, state.ikey, &state, &State::Match);

      return state.found ? state.s : Status::NotFound(Slice());
        */
    }
    
    /**
      | Adds "stats" into the current state.  Returns
      | true if a new compaction may need to be
      | triggered, false otherwise.
      |
      | REQUIRES: lock is held
      */
    pub fn update_stats(&mut self, stats: &VersionGetStats) -> bool {
        
        todo!();
        /*
            FileMetaData* f = stats.seek_file;
      if (f != nullptr) {
        f->allowed_seeks--;
        if (f->allowed_seeks <= 0 && file_to_compact_ == nullptr) {
          file_to_compact_ = f;
          file_to_compact_level_ = stats.seek_file_level;
          return true;
        }
      }
      return false;
        */
    }
    
    /**
      | Record a sample of bytes read at the
      | specified internal key.
      |
      | Samples are taken approximately once every
      | config::kReadBytesPeriod bytes.  Returns true
      | if a new compaction may need to be triggered.
      |
      | REQUIRES: lock is held
      */
    pub fn record_read_sample(&mut self, internal_key_: Slice) -> bool {
        
        todo!();
        /*
            ParsedInternalKey ikey;
      if (!ParseInternalKey(internal_key, &ikey)) {
        return false;
      }

      struct State {
        GetStats stats;  // Holds first matching file
        int matches;

        static bool Match(c_void* arg, int level, FileMetaData* f) {
          State* state = reinterpret_cast<State*>(arg);
          state->matches++;
          if (state->matches == 1) {
            // Remember first match.
            state->stats.seek_file = f;
            state->stats.seek_file_level = level;
          }
          // We can stop iterating once we have a second match.
          return state->matches < 2;
        }
      };

      State state;
      state.matches = 0;
      ForEachOverlapping(ikey.user_key, internal_key, &state, &State::Match);

      // Must have at least two matches since we want to merge across
      // files. But what if we have a single file that contains many
      // overwrites and deletions?  Should we have another mechanism for
      // finding such files?
      if (state.matches >= 2) {
        // 1MB cost is about 1 seek (see comment in Builder::Apply).
        return UpdateStats(state.stats);
      }
      return false;
        */
    }
    
    /**
      | Reference count management (so Versions
      | do not disappear out from under live
      | iterators)
      |
      */
    pub fn ref_(&mut self)  {
        
        todo!();
        /*
            ++refs_;
        */
    }
    
    pub fn unref(&mut self)  {
        
        todo!();
        /*
            assert(this != &vset_->dummy_versions_);
      assert(refs_ >= 1);
      --refs_;
      if (refs_ == 0) {
        delete this;
      }
        */
    }
    
    /**
      | Returns true iff some file in the specified
      | level overlaps some part of
      | [*smallest_user_key,*largest_user_key].
      |
      | smallest_user_key==nullptr represents a key
      | smaller than all the DB's keys.
      |
      | largest_user_key==nullptr represents a key
      | largest than all the DB's keys.
      */
    pub fn overlap_in_level(&mut self, 
        level:             i32,
        smallest_user_key_: *const Slice,
        largest_user_key_:  *const Slice) -> bool {
        
        todo!();
        /*
            return SomeFileOverlapsRange(vset_->icmp_, (level > 0), files_[level],
                                   smallest_user_key, largest_user_key);
        */
    }
    
    /**
      | Return the level at which we should place
      | a new memtable compaction result that covers
      | the range
      | [smallest_user_key,largest_user_key].
      */
    pub fn pick_level_for_mem_table_output(&mut self, 
        smallest_user_key_: &Slice,
        largest_user_key_:  &Slice) -> i32 {
        
        todo!();
        /*
            int level = 0;
      if (!OverlapInLevel(0, &smallest_user_key, &largest_user_key)) {
        // Push to next level if there is no overlap in next level,
        // and the #bytes overlapping in the level after that are limited.
        InternalKey start(smallest_user_key, kMaxSequenceNumber, kValueTypeForSeek);
        InternalKey limit(largest_user_key, 0, static_cast<ValueType>(0));
        std::vector<FileMetaData*> overlaps;
        while (level < config::kMaxMemCompactLevel) {
          if (OverlapInLevel(level + 1, &smallest_user_key, &largest_user_key)) {
            break;
          }
          if (level + 2 < config::NUM_LEVELS) {
            // Check that file does not overlap too many grandparent bytes.
            GetOverlappingInputs(level + 2, &start, &limit, &overlaps);
            const int64_t sum = TotalFileSize(overlaps);
            if (sum > MaxGrandParentOverlapBytes(vset_->options_)) {
              break;
            }
          }
          level++;
        }
      }
      return level;
        */
    }

    /**
      | Store in "*inputs" all files in "level"
      | that overlap [begin,end]
      |
      */
    pub fn get_overlapping_inputs(&mut self, 
        level:  i32,

        /*
           | nullptr means before all keys
           |
           */
        begin:  *const InternalKey,

        /*
           | nullptr means after all keys
           |
           */
        end:    *const InternalKey,

        inputs: *mut Vec<*mut FileMetaData>)  {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      inputs->clear();
      Slice user_begin, user_end;
      if (begin != nullptr) {
        user_begin = begin->user_key();
      }
      if (end != nullptr) {
        user_end = end->user_key();
      }
      const Comparator* user_cmp = vset_->icmp_.user_comparator();
      for (size_t i = 0; i < files_[level].size();) {
        FileMetaData* f = files_[level][i++];
        const Slice file_start = f->smallest.user_key();
        const Slice file_limit = f->largest.user_key();
        if (begin != nullptr && user_cmp->Compare(file_limit, user_begin) < 0) {
          // "f" is completely before specified range; skip it
        } else if (end != nullptr && user_cmp->Compare(file_start, user_end) > 0) {
          // "f" is completely after specified range; skip it
        } else {
          inputs->push_back(f);
          if (level == 0) {
            // Level-0 files may overlap each other.  So check if the newly
            // added file has expanded the range.  If so, restart search.
            if (begin != nullptr && user_cmp->Compare(file_start, user_begin) < 0) {
              user_begin = file_start;
              inputs->clear();
              i = 0;
            } else if (end != nullptr &&
                       user_cmp->Compare(file_limit, user_end) > 0) {
              user_end = file_limit;
              inputs->clear();
              i = 0;
            }
          }
        }
      }
        */
    }
    
    /**
      | Return a human readable string that
      | describes this version's contents.
      |
      */
    pub fn debug_string(&self) -> String {
        
        todo!();
        /*
            std::string r;
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        // E.g.,
        //   --- level 1 ---
        //   17:123['a' .. 'd']
        //   20:43['e' .. 'g']
        r.append("--- level ");
        AppendNumberTo(&r, level);
        r.append(" ---\n");
        const std::vector<FileMetaData*>& files = files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          r.push_back(' ');
          AppendNumberTo(&r, files[i]->number);
          r.push_back(':');
          AppendNumberTo(&r, files[i]->file_size);
          r.append("[");
          r.append(files[i]->smallest.DebugString());
          r.append(" .. ");
          r.append(files[i]->largest.DebugString());
          r.append("]\n");
        }
      }
      return r;
        */
    }
}
