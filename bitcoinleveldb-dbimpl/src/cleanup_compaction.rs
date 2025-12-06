crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      if (compact->builder != nullptr) {
        // May happen if we get a shutdown call in the middle of compaction
        compact->builder->Abandon();
        delete compact->builder;
      } else {
        assert(compact->outfile == nullptr);
      }
      delete compact->outfile;
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        const CompactionState::Output& out = compact->outputs[i];
        pending_outputs_.erase(out.number);
      }
      delete compact;
        */
    }
}
