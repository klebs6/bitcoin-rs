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
            &user_key,
            &internal_key_,
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

#[cfg(test)]
mod version_record_read_sample_state_and_callback_tests {
    use super::*;
    use std::ffi::c_void;

    fn build_record_read_sample_state() -> RecordReadSampleState {
        RecordReadSampleStateBuilder::default()
            .stats(VersionGetStats::default())
            .matches(0)
            .build()
            .expect("RecordReadSampleStateBuilder must succeed")
    }

    #[traced_test]
    fn record_read_sample_match_cb_first_match_sets_stats_and_returns_true() {
        let mut state = build_record_read_sample_state();
        let state_ptr: *mut RecordReadSampleState = &mut state;

        let mut file_meta = FileMetaData::default();
        let mut file_meta_box = Box::new(file_meta);
        let file_ptr: *mut FileMetaData = &mut *file_meta_box;

        let continue_iterating = record_read_sample_match_cb(
            state_ptr as *mut c_void,
            3,
            file_ptr,
        );

        assert_eq!(
            *state.matches(),
            1,
            "First invocation must increment matches to 1"
        );
        assert!(
            continue_iterating,
            "Callback must return true when matches == 1"
        );
        assert_eq!(
            *state.stats().seek_file(),
            file_ptr,
            "stats.seek_file must be set to first matching file pointer"
        );
        assert_eq!(
            *state.stats().seek_file_level(),
            3,
            "stats.seek_file_level must equal level of first match"
        );
    }

    #[traced_test]
    fn record_read_sample_match_cb_second_match_returns_false_without_overwriting_stats() {
        let mut state = build_record_read_sample_state();
        let state_ptr: *mut RecordReadSampleState = &mut state;

        let mut first_meta = FileMetaData::default();
        let mut first_box = Box::new(first_meta);
        let first_ptr: *mut FileMetaData = &mut *first_box;

        let cont1 = record_read_sample_match_cb(
            state_ptr as *mut c_void,
            2,
            first_ptr,
        );
        assert!(cont1, "Iteration should continue after first match");

        let mut second_meta = FileMetaData::default();
        let mut second_box = Box::new(second_meta);
        let second_ptr: *mut FileMetaData = &mut *second_box;

        let cont2 = record_read_sample_match_cb(
            state_ptr as *mut c_void,
            5,
            second_ptr,
        );

        assert_eq!(
            *state.matches(),
            2,
            "Two invocations of callback must increment matches to 2"
        );
        assert!(
            !cont2,
            "Callback must return false once matches reaches 2"
        );
        assert_eq!(
            *state.stats().seek_file(),
            first_ptr,
            "stats.seek_file must still refer to the first matching file"
        );
        assert_eq!(
            *state.stats().seek_file_level(),
            2,
            "stats.seek_file_level must remain that of the first match"
        );
    }
}

#[cfg(test)]
mod version_record_read_sample_behavior_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[traced_test]
    fn record_read_sample_returns_false_when_internal_key_parse_fails() {
        let mut version = helpers::build_empty_version();

        let bad_bytes: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let key_slice = Slice::from(&bad_bytes[..]);

        let triggered = version.record_read_sample(key_slice);
        assert!(
            !triggered,
            "record_read_sample must return false when ParseInternalKey fails"
        );
    }

    #[traced_test]
    fn record_read_sample_method_signature_remains_stable() {
        let _fn_ptr: fn(&mut Version, Slice) -> bool =
            Version::record_read_sample;
        let _ = _fn_ptr;
    }
}
