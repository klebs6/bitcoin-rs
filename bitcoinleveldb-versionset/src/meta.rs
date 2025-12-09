// ---------------- [ File: bitcoinleveldb-versionset/src/meta.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.h]

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.cc]

pub fn target_file_size(options: *const Options) -> usize {
    
    todo!();
        /*
            return options->max_file_size;
        */
}

/**
  | Maximum bytes of overlaps in grandparent
  | (i.e., level+2) before we stop building
  | a single file in a level->level+1 compaction.
  |
  */
pub fn max_grand_parent_overlap_bytes(options: *const Options) -> i64 {
    
    todo!();
        /*
            return 10 * TargetFileSize(options);
        */
}

/**
  | Maximum number of bytes in all compacted files.
  | We avoid expanding the lower level file set of
  | a compaction if it would make the total
  | compaction cover more than this many bytes.
  */
pub fn expanded_compaction_byte_size_limit(options: *const Options) -> i64 {
    
    todo!();
        /*
            return 25 * TargetFileSize(options);
        */
}

pub fn max_bytes_for_level(
        options: *const Options,
        level:   i32) -> f64 {
    
    todo!();
        /*
            // Note: the result for level zero is not really used since we set
      // the level-0 compaction threshold based on number of files.

      // Result for both level-0 and level-1
      double result = 10. * 1048576.0;
      while (level > 1) {
        result *= 10;
        level--;
      }
      return result;
        */
}

pub fn max_file_size_for_level(
        options: *const Options,
        level:   i32) -> u64 {
    
    todo!();
        /*
            // We could vary per level to reduce number of files?
      return TargetFileSize(options);
        */
}

pub fn total_file_size(files: &Vec<*mut FileMetaData>) -> i64 {
    
    todo!();
        /*
            int64_t sum = 0;
      for (size_t i = 0; i < files.size(); i++) {
        sum += files[i]->file_size;
      }
      return sum;
        */
}

/**
  | Return the smallest index i such that
  | files[i]->largest >= key.
  |
  | Return files.size() if there is no such file.
  |
  | REQUIRES: "files" contains a sorted list of
  | non-overlapping files.
  */
pub fn find_file(
        icmp:  &InternalKeyComparator,
        files: &Vec<*mut FileMetaData>,
        key_:   &Slice) -> i32 {
    
    todo!();
        /*
            uint32_t left = 0;
      uint32_t right = files.size();
      while (left < right) {
        uint32_t mid = (left + right) / 2;
        const FileMetaData* f = files[mid];
        if (icmp.InternalKeyComparator::Compare(f->largest.Encode(), key) < 0) {
          // Key at "mid.largest" is < "target".  Therefore all
          // files at or before "mid" are uninteresting.
          left = mid + 1;
        } else {
          // Key at "mid.largest" is >= "target".  Therefore all files
          // after "mid" are uninteresting.
          right = mid;
        }
      }
      return right;
        */
}

pub fn after_file(
        ucmp:     Box<dyn SliceComparator>,
        user_key_: *const Slice,
        f:        *const FileMetaData) -> bool {
    
    todo!();
        /*
            // null user_key occurs before all keys and is therefore never after *f
      return (user_key != nullptr &&
              ucmp->Compare(*user_key, f->largest.user_key()) > 0);
        */
}

pub fn before_file(
        ucmp:     Box<dyn SliceComparator>,
        user_key_: *const Slice,
        f:        *const FileMetaData) -> bool {
    
    todo!();
        /*
            // null user_key occurs after all keys and is therefore never before *f
      return (user_key != nullptr &&
              ucmp->Compare(*user_key, f->smallest.user_key()) < 0);
        */
}

/**
  | Returns true iff some file in "files" overlaps
  | the user key range [*smallest,*largest].
  |
  | smallest==nullptr represents a key smaller than
  | all keys in the DB.
  |
  | largest==nullptr represents a key largest than
  | all keys in the DB.
  |
  | REQUIRES: If disjoint_sorted_files, files[]
  |           contains disjoint ranges in sorted
  |           order.
  */
pub fn some_file_overlaps_range(
        icmp:                  &InternalKeyComparator,
        disjoint_sorted_files: bool,
        files:                 &Vec<*mut FileMetaData>,
        smallest_user_key_:     *const Slice,
        largest_user_key_:      *const Slice) -> bool {
    
    todo!();
        /*
            const Comparator* ucmp = icmp.user_comparator();
      if (!disjoint_sorted_files) {
        // Need to check against all files
        for (size_t i = 0; i < files.size(); i++) {
          const FileMetaData* f = files[i];
          if (AfterFile(ucmp, smallest_user_key, f) ||
              BeforeFile(ucmp, largest_user_key, f)) {
            // No overlap
          } else {
            return true;  // Overlap
          }
        }
        return false;
      }

      // Binary search over file list
      uint32_t index = 0;
      if (smallest_user_key != nullptr) {
        // Find the earliest possible internal key for smallest_user_key
        InternalKey small_key(*smallest_user_key, kMaxSequenceNumber,
                              kValueTypeForSeek);
        index = FindFile(icmp, files, small_key.Encode());
      }

      if (index >= files.size()) {
        // beginning of range is after all files, so no overlap.
        return false;
      }

      return !BeforeFile(ucmp, largest_user_key, files[index]);
        */
}

pub fn get_file_iterator(
        arg:        *mut c_void,
        options:    &ReadOptions,
        file_value: &Slice) -> *mut LevelDBIterator {
    
    todo!();
        /*
            TableCache* cache = reinterpret_cast<TableCache*>(arg);
      if (file_value.size() != 16) {
        return NewErrorIterator(
            Status::Corruption("FileReader invoked with unexpected value"));
      } else {
        return cache->NewIterator(options, DecodeFixed64(file_value.data()),
                                  DecodeFixed64(file_value.data() + 8));
      }
        */
}

/**
  | Callback from TableCache::Get()
  |
  */
pub enum SaverState {
    NotFound,
    Found,
    Deleted,
    Corrupt,
}

pub struct Saver {
    state:    SaverState,
    ucmp:     Box<dyn SliceComparator>,
    user_key_: Slice,
    value:    *mut String,
}

pub fn save_value(
        arg:  *mut c_void,
        ikey_: &Slice,
        v:    &Slice)  {
    
    todo!();
        /*
            Saver* s = reinterpret_cast<Saver*>(arg);
      ParsedInternalKey parsed_key;
      if (!ParseInternalKey(ikey, &parsed_key)) {
        s->state = kCorrupt;
      } else {
        if (s->ucmp->Compare(parsed_key.user_key, s->user_key) == 0) {
          s->state = (parsed_key.type == kTypeValue) ? kFound : kDeleted;
          if (s->state == kFound) {
            s->value->assign(v.data(), v.size());
          }
        }
      }
        */
}

pub fn newest_first(
        a: *mut FileMetaData,
        b: *mut FileMetaData) -> bool {
    
    todo!();
        /*
            return a->number > b->number;
        */
}

/**
  | Finds the largest key in a vector of files.
  | Returns true if files it not empty.
  |
  */
pub fn find_largest_key(
        icmp:        &InternalKeyComparator,
        files:       &Vec<*mut FileMetaData>,
        largest_key_: *mut InternalKey) -> bool {
    
    todo!();
        /*
            if (files.empty()) {
        return false;
      }
      *largest_key = files[0]->largest;
      for (size_t i = 1; i < files.size(); ++i) {
        FileMetaData* f = files[i];
        if (icmp.Compare(f->largest, *largest_key) > 0) {
          *largest_key = f->largest;
        }
      }
      return true;
        */
}

/**
  | Finds minimum file b2=(l2, u2) in level
  | file for which l2 > u1 and user_key(l2)
  | = user_key(u1)
  |
  */
pub fn find_smallest_boundary_file(
        icmp:        &InternalKeyComparator,
        level_files: &Vec<*mut FileMetaData>,
        largest_key_: &InternalKey) -> *mut FileMetaData {
    
    todo!();
        /*
            const Comparator* user_cmp = icmp.user_comparator();
      FileMetaData* smallest_boundary_file = nullptr;
      for (size_t i = 0; i < level_files.size(); ++i) {
        FileMetaData* f = level_files[i];
        if (icmp.Compare(f->smallest, largest_key) > 0 &&
            user_cmp->Compare(f->smallest.user_key(), largest_key.user_key()) ==
                0) {
          if (smallest_boundary_file == nullptr ||
              icmp.Compare(f->smallest, smallest_boundary_file->smallest) < 0) {
            smallest_boundary_file = f;
          }
        }
      }
      return smallest_boundary_file;
        */
}

/**
  | Extracts the largest file b1 from
  | |compaction_files| and then searches for a b2
  | in |level_files| for which user_key(u1)
  | = user_key(l2). If it finds such a file b2
  | (known as a boundary file) it adds it to
  | |compaction_files| and then searches again
  | using this new upper bound.
  |
  | If there are two blocks, b1=(l1, u1) and
  | b2=(l2, u2) and user_key(u1) = user_key(l2),
  | and if we compact b1 but not b2 then
  | a subsequent get operation will yield an
  | incorrect result because it will return the
  | record from b2 in level i rather than from b1
  | because it searches level by level for records
  | matching the supplied user key.
  |
  | parameters:
  |
  |   in     level_files:      List of files to
  |   search for boundary files.
  |
  |   in/out compaction_files: List of files to
  |   extend by adding boundary files.
  */
pub fn add_boundary_inputs(
        icmp:             &InternalKeyComparator,
        level_files:      &Vec<*mut FileMetaData>,
        compaction_files: *mut Vec<*mut FileMetaData>)  {
    
    todo!();
        /*
            InternalKey largest_key;

      // Quick return if compaction_files is empty.
      if (!FindLargestKey(icmp, *compaction_files, &largest_key)) {
        return;
      }

      bool continue_searching = true;
      while (continue_searching) {
        FileMetaData* smallest_boundary_file =
            FindSmallestBoundaryFile(icmp, level_files, largest_key);

        // If a boundary file was found advance largest_key, otherwise we're done.
        if (smallest_boundary_file != NULL) {
          compaction_files->push_back(smallest_boundary_file);
          largest_key = smallest_boundary_file->largest;
        } else {
          continue_searching = false;
        }
      }
        */
}
