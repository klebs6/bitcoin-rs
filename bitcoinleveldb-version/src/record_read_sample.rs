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
        trace!(
            "Version::record_read_sample: internal_key_len={}",
            *internal_key_.size()
        );

        let mut ikey = ParsedInternalKey::default();
        let parsed_ok =
            parse_internal_key(&internal_key_, &mut ikey as *mut ParsedInternalKey);

        if !parsed_ok {
            debug!(
                "Version::record_read_sample: ParseInternalKey failed; ignoring sample"
            );
            return false;
        }

        let user_key_slice_ref = ikey.user_key();
        let user_key = unsafe {
            Slice::from_ptr_len(
                *user_key_slice_ref.data(),
                *user_key_slice_ref.size(),
            )
        };

        let mut state = RecordReadSampleStateBuilder::default()
            .stats(VersionGetStats::default())
            .matches(0)
            .build()
            .unwrap();

        self.for_each_overlapping(
            user_key,
            internal_key_,
            &mut state as *mut RecordReadSampleState as *mut c_void,
            record_read_sample_match_cb,
        );

        // Must have at least two matches since we want to merge across files. 
        //
        // But what if we have a single file that contains many overwrites and
        // deletions?  
        //
        // Should we have another mechanism for finding such files?
        //
        if *state.matches() >= 2 {
            // 1MB cost is about 1 seek (see comment in Builder::Apply).
            trace!(
                "Version::record_read_sample: found {} overlapping files; updating stats",
                state.matches()
            );
            let triggered = self.update_stats(state.stats_mut());
            debug!(
                "Version::record_read_sample: UpdateStats returned {}",
                triggered
            );
            triggered
        } else {
            trace!(
                "Version::record_read_sample: matches < 2 ({}); no compaction trigger",
                state.matches()
            );
            false
        }
    }
}
