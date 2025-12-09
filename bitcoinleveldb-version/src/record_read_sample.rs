// ---------------- [ File: bitcoinleveldb-version/src/record_read_sample.rs ]
crate::ix!();

impl Version {

    /**
      | Record a sample of bytes read at the
      | specified internal key.
      |
      | Samples are taken approximately once every
      | config::kReadBytesPeriod bytes.  Returns true
      | if a new compaction may need to be triggered.
      |
      | REQUIRES: lock is held
      */
    pub fn record_read_sample(&mut self, internal_key_: Slice) -> bool {
        
        todo!();
        /*
            ParsedInternalKey ikey;
      if (!ParseInternalKey(internal_key, &ikey)) {
        return false;
      }

      struct State {
        GetStats stats;  // Holds first matching file
        int matches;

        static bool Match(c_void* arg, int level, FileMetaData* f) {
          State* state = reinterpret_cast<State*>(arg);
          state->matches++;
          if (state->matches == 1) {
            // Remember first match.
            state->stats.seek_file = f;
            state->stats.seek_file_level = level;
          }
          // We can stop iterating once we have a second match.
          return state->matches < 2;
        }
      };

      State state;
      state.matches = 0;
      ForEachOverlapping(ikey.user_key, internal_key, &state, &State::Match);

      // Must have at least two matches since we want to merge across
      // files. But what if we have a single file that contains many
      // overwrites and deletions?  Should we have another mechanism for
      // finding such files?
      if (state.matches >= 2) {
        // 1MB cost is about 1 seek (see comment in Builder::Apply).
        return UpdateStats(state.stats);
      }
      return false;
        */
    }
}
