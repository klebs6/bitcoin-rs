// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_apply.rs ]
crate::ix!();

impl VersionSetBuilder {
    /**
      | Apply all of the edits in *edit to the
      | current state.
      |
      */
    pub fn apply(&mut self, edit: *mut VersionEdit)  {
        
        todo!();
        /*
            // Update compaction pointers
        for (size_t i = 0; i < edit->compact_pointers_.size(); i++) {
          const int level = edit->compact_pointers_[i].first;
          vset_->compact_pointer_[level] =
              edit->compact_pointers_[i].second.Encode().ToString();
        }

        // Delete files
        for (const auto& deleted_file_set_kvp : edit->deleted_files_) {
          const int level = deleted_file_set_kvp.first;
          const uint64_t number = deleted_file_set_kvp.second;
          levels_[level].deleted_files.insert(number);
        }

        // Add new files
        for (size_t i = 0; i < edit->new_files_.size(); i++) {
          const int level = edit->new_files_[i].first;
          FileMetaData* f = new FileMetaData(edit->new_files_[i].second);
          f->refs = 1;

          // We arrange to automatically compact this file after
          // a certain number of seeks.  Let's assume:
          //   (1) One seek costs 10ms
          //   (2) Writing or reading 1MB costs 10ms (100MB/s)
          //   (3) A compaction of 1MB does 25MB of IO:
          //         1MB read from this level
          //         10-12MB read from next level (boundaries may be misaligned)
          //         10-12MB written to next level
          // This implies that 25 seeks cost the same as the compaction
          // of 1MB of data.  I.e., one seek costs approximately the
          // same as the compaction of 40KB of data.  We are a little
          // conservative and allow approximately one seek for every 16KB
          // of data before triggering a compaction.
          f->allowed_seeks = static_cast<int>((f->file_size / 16384U));
          if (f->allowed_seeks < 100) f->allowed_seeks = 100;

          levels_[level].deleted_files.erase(f->number);
          levels_[level].added_files->insert(f);
        }
        */
    }
}
