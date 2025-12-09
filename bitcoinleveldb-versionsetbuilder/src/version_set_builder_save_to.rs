// ---------------- [ File: bitcoinleveldb-versionsetbuilder/src/version_set_builder_save_to.rs ]
crate::ix!();

impl VersionSetBuilder {

    /**
      | Save the current state in *v.
      |
      */
    pub fn save_to(&mut self, v: *mut Version)  {
        
        todo!();
        /*
            BySmallestKeyComparator cmp;
        cmp.internal_comparator = &vset_->icmp_;
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          // Merge the set of added files with the set of pre-existing files.
          // Drop any deleted files.  Store the result in *v.
          const std::vector<FileMetaData*>& base_files = base_->files_[level];
          std::vector<FileMetaData*>::const_iterator base_iter = base_files.begin();
          std::vector<FileMetaData*>::const_iterator base_end = base_files.end();
          const VersionSetBuilderFileSet* added_files = levels_[level].added_files;
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
}
