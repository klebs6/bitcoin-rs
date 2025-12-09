// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_pick_compaction.rs ]
crate::ix!();

impl PickCompaction for VersionSet {

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
    fn pick_compaction(&mut self) -> *mut Compaction {
        
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
}
