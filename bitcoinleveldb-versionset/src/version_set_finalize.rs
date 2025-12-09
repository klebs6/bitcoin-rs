// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_finalize.rs ]
crate::ix!();

impl FinalizeVersionSet for VersionSet {
    
    fn finalize(&mut self, v: *mut Version)  {
        
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
}
