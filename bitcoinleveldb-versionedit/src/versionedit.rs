// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit.rs ]
crate::ix!();

///--------------
pub struct VersionEdit {
    comparator:           String,
    log_number:           u64,
    prev_log_number:      u64,
    next_file_number:     u64,
    last_sequence:        SequenceNumber,
    has_comparator:       bool,
    has_log_number:       bool,
    has_prev_log_number:  bool,
    has_next_file_number: bool,
    has_last_sequence:    bool,
    compact_pointers:     Vec<(i32,InternalKey)>,
    deleted_files:        VersionEditDeletedFileSet,
    new_files:            Vec<(i32,FileMetaData)>,
}

pub type VersionEditDeletedFileSet = HashSet<(i32,u64)>;

impl Default for VersionEdit {
    
    fn default() -> Self {
        todo!();
        /*
           Clear();
           */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit.cc]
impl VersionEdit {

    pub fn set_comparator_name(&mut self, name: &Slice)  {
        
        todo!();
        /*
            has_comparator_ = true;
        comparator_ = name.ToString();
        */
    }
    
    pub fn set_log_number(&mut self, num: u64)  {
        
        todo!();
        /*
            has_log_number_ = true;
        log_number_ = num;
        */
    }
    
    pub fn set_prev_log_number(&mut self, num: u64)  {
        
        todo!();
        /*
            has_prev_log_number_ = true;
        prev_log_number_ = num;
        */
    }
    
    pub fn set_next_file(&mut self, num: u64)  {
        
        todo!();
        /*
            has_next_file_number_ = true;
        next_file_number_ = num;
        */
    }
    
    pub fn set_last_sequence(&mut self, seq: SequenceNumber)  {
        
        todo!();
        /*
            has_last_sequence_ = true;
        last_sequence_ = seq;
        */
    }
    
    pub fn set_compact_pointer(&mut self, 
        level: i32,
        key_:   &InternalKey)  {
        
        todo!();
        /*
            compact_pointers_.push_back(std::make_pair(level, key));
        */
    }

    /**
      | Add the specified file at the specified
      | number.
      |
      | REQUIRES: This version has not been saved
      | (see VersionSet::SaveTo)
      |
      | REQUIRES: "smallest" and "largest" are
      | smallest and largest keys in file
      */
    pub fn add_file(&mut self, 
        level:     i32,
        file:      u64,
        file_size: u64,
        smallest:  &InternalKey,
        largest:   &InternalKey)  {
        
        todo!();
        /*
            FileMetaData f;
        f.number = file;
        f.file_size = file_size;
        f.smallest = smallest;
        f.largest = largest;
        new_files_.push_back(std::make_pair(level, f));
        */
    }

    /**
      | Delete the specified "file" from the
      | specified "level".
      |
      */
    pub fn delete_file(&mut self, 
        level: i32,
        file:  u64)  {
        
        todo!();
        /*
            deleted_files_.insert(std::make_pair(level, file));
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            comparator_.clear();
      log_number_ = 0;
      prev_log_number_ = 0;
      last_sequence_ = 0;
      next_file_number_ = 0;
      has_comparator_ = false;
      has_log_number_ = false;
      has_prev_log_number_ = false;
      has_next_file_number_ = false;
      has_last_sequence_ = false;
      deleted_files_.clear();
      new_files_.clear();
        */
    }
    
    pub fn encode_to(&self, dst: *mut String)  {
        
        todo!();
        /*
            if (has_comparator_) {
        PutVarint32(dst, kComparator);
        PutLengthPrefixedSlice(dst, comparator_);
      }
      if (has_log_number_) {
        PutVarint32(dst, kLogNumber);
        PutVarint64(dst, log_number_);
      }
      if (has_prev_log_number_) {
        PutVarint32(dst, kPrevLogNumber);
        PutVarint64(dst, prev_log_number_);
      }
      if (has_next_file_number_) {
        PutVarint32(dst, kNextFileNumber);
        PutVarint64(dst, next_file_number_);
      }
      if (has_last_sequence_) {
        PutVarint32(dst, kLastSequence);
        PutVarint64(dst, last_sequence_);
      }

      for (size_t i = 0; i < compact_pointers_.size(); i++) {
        PutVarint32(dst, kCompactPointer);
        PutVarint32(dst, compact_pointers_[i].first);  // level
        PutLengthPrefixedSlice(dst, compact_pointers_[i].second.Encode());
      }

      for (const auto& deleted_file_kvp : deleted_files_) {
        PutVarint32(dst, kDeletedFile);
        PutVarint32(dst, deleted_file_kvp.first);   // level
        PutVarint64(dst, deleted_file_kvp.second);  // file number
      }

      for (size_t i = 0; i < new_files_.size(); i++) {
        const FileMetaData& f = new_files_[i].second;
        PutVarint32(dst, kNewFile);
        PutVarint32(dst, new_files_[i].first);  // level
        PutVarint64(dst, f.number);
        PutVarint64(dst, f.file_size);
        PutLengthPrefixedSlice(dst, f.smallest.Encode());
        PutLengthPrefixedSlice(dst, f.largest.Encode());
      }
        */
    }
    
    pub fn decode_from(&mut self, src: &Slice) -> Status {
        
        todo!();
        /*
            Clear();
      Slice input = src;
      const char* msg = nullptr;
      uint32_t tag;

      // Temporary storage for parsing
      int level;
      uint64_t number;
      FileMetaData f;
      Slice str;
      InternalKey key;

      while (msg == nullptr && GetVarint32(&input, &tag)) {
        switch (tag) {
          case kComparator:
            if (GetLengthPrefixedSlice(&input, &str)) {
              comparator_ = str.ToString();
              has_comparator_ = true;
            } else {
              msg = "comparator name";
            }
            break;

          case kLogNumber:
            if (GetVarint64(&input, &log_number_)) {
              has_log_number_ = true;
            } else {
              msg = "log number";
            }
            break;

          case kPrevLogNumber:
            if (GetVarint64(&input, &prev_log_number_)) {
              has_prev_log_number_ = true;
            } else {
              msg = "previous log number";
            }
            break;

          case kNextFileNumber:
            if (GetVarint64(&input, &next_file_number_)) {
              has_next_file_number_ = true;
            } else {
              msg = "next file number";
            }
            break;

          case kLastSequence:
            if (GetVarint64(&input, &last_sequence_)) {
              has_last_sequence_ = true;
            } else {
              msg = "last sequence number";
            }
            break;

          case kCompactPointer:
            if (GetLevel(&input, &level) && GetInternalKey(&input, &key)) {
              compact_pointers_.push_back(std::make_pair(level, key));
            } else {
              msg = "compaction pointer";
            }
            break;

          case kDeletedFile:
            if (GetLevel(&input, &level) && GetVarint64(&input, &number)) {
              deleted_files_.insert(std::make_pair(level, number));
            } else {
              msg = "deleted file";
            }
            break;

          case kNewFile:
            if (GetLevel(&input, &level) && GetVarint64(&input, &f.number) &&
                GetVarint64(&input, &f.file_size) &&
                GetInternalKey(&input, &f.smallest) &&
                GetInternalKey(&input, &f.largest)) {
              new_files_.push_back(std::make_pair(level, f));
            } else {
              msg = "new-file entry";
            }
            break;

          default:
            msg = "unknown tag";
            break;
        }
      }

      if (msg == nullptr && !input.empty()) {
        msg = "invalid tag";
      }

      Status result;
      if (msg != nullptr) {
        result = Status::Corruption("VersionEdit", msg);
      }
      return result;
        */
    }
    
    pub fn debug_string(&self) -> String {
        
        todo!();
        /*
            std::string r;
      r.append("VersionEdit {");
      if (has_comparator_) {
        r.append("\n  Comparator: ");
        r.append(comparator_);
      }
      if (has_log_number_) {
        r.append("\n  LogNumber: ");
        AppendNumberTo(&r, log_number_);
      }
      if (has_prev_log_number_) {
        r.append("\n  PrevLogNumber: ");
        AppendNumberTo(&r, prev_log_number_);
      }
      if (has_next_file_number_) {
        r.append("\n  NextFile: ");
        AppendNumberTo(&r, next_file_number_);
      }
      if (has_last_sequence_) {
        r.append("\n  LastSeq: ");
        AppendNumberTo(&r, last_sequence_);
      }
      for (size_t i = 0; i < compact_pointers_.size(); i++) {
        r.append("\n  CompactPointer: ");
        AppendNumberTo(&r, compact_pointers_[i].first);
        r.append(" ");
        r.append(compact_pointers_[i].second.DebugString());
      }
      for (const auto& deleted_files_kvp : deleted_files_) {
        r.append("\n  DeleteFile: ");
        AppendNumberTo(&r, deleted_files_kvp.first);
        r.append(" ");
        AppendNumberTo(&r, deleted_files_kvp.second);
      }
      for (size_t i = 0; i < new_files_.size(); i++) {
        const FileMetaData& f = new_files_[i].second;
        r.append("\n  AddFile: ");
        AppendNumberTo(&r, new_files_[i].first);
        r.append(" ");
        AppendNumberTo(&r, f.number);
        r.append(" ");
        AppendNumberTo(&r, f.file_size);
        r.append(" ");
        r.append(f.smallest.DebugString());
        r.append(" .. ");
        r.append(f.largest.DebugString());
      }
      r.append("\n}\n");
      return r;
        */
    }
}
