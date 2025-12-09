// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_compact_range.rs ]
crate::ix!();

impl CompactRange for VersionSet {

    /**
      | Return a compaction object for compacting the
      | range [begin,end] in the specified level.
      | Returns nullptr if there is nothing in that
      | level that overlaps the specified range.
      | Caller should delete the result.
      */
    fn compact_range(&mut self, 
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
