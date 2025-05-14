// ---------------- [ File: bitcoinleveldb-version/src/setbuilder.rs ]
crate::ix!();

/**
  | A helper class so we can efficiently apply
  | a whole sequence of edits to a particular state
  | without creating intermediate Versions that
  | contain full copies of the intermediate state.
  */
pub struct VersionSetBuilder {
    vset:   *mut VersionSet,
    base:   *mut Version,
    levels: [version_set_builder::LevelState; NUM_LEVELS],
}

pub mod version_set_builder {

    use super::*;

    /**
      | Helper to sort by
      | v->files_[file_number].smallest
      |
      */
    pub struct BySmallestKey {
        internal_comparator: *const InternalKeyComparator,
    }

    impl BySmallestKey {

        pub fn invoke(&self, 
            f1: *mut FileMetaData,
            f2: *mut FileMetaData) -> bool {
            
            todo!();
            /*
                int r = internal_comparator->Compare(f1->smallest, f2->smallest);
              if (r != 0) {
                return (r < 0);
              } else {
                // Break ties by file number
                return (f1->number < f2->number);
              }
            */
        }
    }

    pub type FileSet = HashSet<*mut FileMetaData,BySmallestKey>;

    pub struct LevelState {
        deleted_files: HashSet<u64>,
        added_files:   *mut FileSet,
    }
}

impl Drop for VersionSetBuilder {

    fn drop(&mut self) {
        todo!();
        /*
            for (int level = 0; level < config::NUM_LEVELS; level++) {
          const FileSet* added = levels_[level].added_files;
          std::vector<FileMetaData*> to_unref;
          to_unref.reserve(added->size());
          for (FileSet::const_iterator it = added->begin(); it != added->end();
               ++it) {
            to_unref.push_back(*it);
          }
          delete added;
          for (uint32_t i = 0; i < to_unref.size(); i++) {
            FileMetaData* f = to_unref[i];
            f->refs--;
            if (f->refs <= 0) {
              delete f;
            }
          }
        }
        base_->Unref();
        */
    }
}

impl VersionSetBuilder {

    /**
      | Initialize a builder with the files
      | from *base and other info from *vset
      |
      */
    pub fn new(
        vset: *mut VersionSet,
        base: *mut Version) -> Self {
    
        todo!();
        /*
        : vset(vset),
        : base(base),

            base_->Ref();
        BySmallestKey cmp;
        cmp.internal_comparator = &vset_->icmp_;
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          levels_[level].added_files = new FileSet(cmp);
        }
        */
    }

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

    /**
      | Save the current state in *v.
      |
      */
    pub fn save_to(&mut self, v: *mut Version)  {
        
        todo!();
        /*
            BySmallestKey cmp;
        cmp.internal_comparator = &vset_->icmp_;
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          // Merge the set of added files with the set of pre-existing files.
          // Drop any deleted files.  Store the result in *v.
          const std::vector<FileMetaData*>& base_files = base_->files_[level];
          std::vector<FileMetaData*>::const_iterator base_iter = base_files.begin();
          std::vector<FileMetaData*>::const_iterator base_end = base_files.end();
          const FileSet* added_files = levels_[level].added_files;
          v->files_[level].reserve(base_files.size() + added_files->size());
          for (const auto& added_file : *added_files) {
            // Add all smaller files listed in base_
            for (std::vector<FileMetaData*>::const_iterator bpos =
                     std::upper_bound(base_iter, base_end, added_file, cmp);
                 base_iter != bpos; ++base_iter) {
              MaybeAddFile(v, level, *base_iter);
            }

            MaybeAddFile(v, level, added_file);
          }

          // Add remaining base files
          for (; base_iter != base_end; ++base_iter) {
            MaybeAddFile(v, level, *base_iter);
          }

    #ifndef NDEBUG
          // Make sure there is no overlap in levels > 0
          if (level > 0) {
            for (uint32_t i = 1; i < v->files_[level].size(); i++) {
              const InternalKey& prev_end = v->files_[level][i - 1]->largest;
              const InternalKey& this_begin = v->files_[level][i]->smallest;
              if (vset_->icmp_.Compare(prev_end, this_begin) >= 0) {
                fprintf(stderr, "overlapping ranges in same level %s vs. %s\n",
                        prev_end.DebugString().c_str(),
                        this_begin.DebugString().c_str());
                abort();
              }
            }
          }
    #endif
        }
        */
    }
    
    pub fn maybe_add_file(&mut self, 
        v:     *mut Version,
        level: i32,
        f:     *mut FileMetaData)  {
        
        todo!();
        /*
            if (levels_[level].deleted_files.count(f->number) > 0) {
          // File is deleted: do nothing
        } else {
          std::vector<FileMetaData*>* files = &v->files_[level];
          if (level > 0 && !files->empty()) {
            // Must not overlap
            assert(vset_->icmp_.Compare((*files)[files->size() - 1]->largest,
                                        f->smallest) < 0);
          }
          f->refs++;
          files->push_back(f);
        }
        */
    }
}
