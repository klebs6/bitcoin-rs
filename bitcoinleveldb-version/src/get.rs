// ---------------- [ File: bitcoinleveldb-version/src/get.rs ]
crate::ix!();

impl Version {

    pub fn get(&mut self, 
        options: &ReadOptions,
        k:       &LookupKey,
        value:   *mut String,
        stats:   *mut VersionGetStats) -> Status {
        
        todo!();
        /*
            stats->seek_file = nullptr;
      stats->seek_file_level = -1;

      struct State {
        Saver saver;
        GetStats* stats;
        const ReadOptions* options;
        Slice ikey;
        FileMetaData* last_file_read;
        int last_file_read_level;

        VersionSet* vset;
        Status s;
        bool found;

        static bool Match(c_void* arg, int level, FileMetaData* f) {
          State* state = reinterpret_cast<State*>(arg);

          if (state->stats->seek_file == nullptr &&
              state->last_file_read != nullptr) {
            // We have had more than one seek for this read.  Charge the 1st file.
            state->stats->seek_file = state->last_file_read;
            state->stats->seek_file_level = state->last_file_read_level;
          }

          state->last_file_read = f;
          state->last_file_read_level = level;

          state->s = state->vset->table_cache_->Get(*state->options, f->number,
                                                    f->file_size, state->ikey,
                                                    &state->saver, SaveValue);
          if (!state->s.ok()) {
            state->found = true;
            return false;
          }
          switch (state->saver.state) {
            case kNotFound:
              return true;  // Keep searching in other files
            case kFound:
              state->found = true;
              return false;
            case kDeleted:
              return false;
            case kCorrupt:
              state->s =
                  Status::Corruption("corrupted key for ", state->saver.user_key);
              state->found = true;
              return false;
          }

          // Not reached. Added to avoid false compilation warnings of
          // "control reaches end of non-c_void function".
          return false;
        }
      };

      State state;
      state.found = false;
      state.stats = stats;
      state.last_file_read = nullptr;
      state.last_file_read_level = -1;

      state.options = &options;
      state.ikey = k.internal_key();
      state.vset = vset_;

      state.saver.state = kNotFound;
      state.saver.ucmp = vset_->icmp_.user_comparator();
      state.saver.user_key = k.user_key();
      state.saver.value = value;

      ForEachOverlapping(state.saver.user_key, state.ikey, &state, &State::Match);

      return state.found ? state.s : Status::NotFound(Slice());
        */
    }
}
