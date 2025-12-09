// ---------------- [ File: bitcoinleveldb-dbimpl/src/install_compaction_results.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            mutex_.AssertHeld();
      Log(options_.info_log, "Compacted %d@%d + %d@%d files => %lld bytes",
          compact->compaction->num_input_files(0), compact->compaction->level(),
          compact->compaction->num_input_files(1), compact->compaction->level() + 1,
          static_cast<long long>(compact->total_bytes));

      // Add compaction outputs
      compact->compaction->AddInputDeletions(compact->compaction->edit());
      const int level = compact->compaction->level();
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        const CompactionState::Output& out = compact->outputs[i];
        compact->compaction->edit()->AddFile(level + 1, out.number, out.file_size,
                                             out.smallest, out.largest);
      }
      return versions_->LogAndApply(compact->compaction->edit(), &mutex_);
        */
    }
}
