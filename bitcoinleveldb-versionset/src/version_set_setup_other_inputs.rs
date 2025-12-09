// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_setup_other_inputs.rs ]
crate::ix!();

impl SetupOtherInputs for VersionSet {
    
    fn setup_other_inputs(&mut self, c: *mut Compaction)  {
        
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
}
