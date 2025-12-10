// ---------------- [ File: bitcoinleveldb-version/src/record_read_sample_match_cb.rs ]
crate::ix!();

// State used by Version::record_read_sample callback.
#[derive(Builder,Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
#[builder(pattern = "owned")]
pub struct RecordReadSampleState {
    stats:   VersionGetStats, // Holds first matching file
    matches: i32,
}

pub fn record_read_sample_match_cb(
    arg:   *mut c_void,
    level: i32,
    f:     *mut FileMetaData,
) -> bool {
    trace!(
        "record_read_sample_match_cb: level={}, file_ptr={:?}",
        level,
        f
    );

    unsafe {
        let state: &mut RecordReadSampleState = &mut *(arg as *mut RecordReadSampleState);
        *state.matches_mut() += 1;

        if *state.matches() == 1 {
            // Remember first match.
            state.stats_mut().set_seek_file(f);
            state.stats_mut().set_seek_file_level(level);
            trace!(
                "record_read_sample_match_cb: recorded first match at level {}",
                level
            );
        }

        // We can stop iterating once we have a second match.
        *state.matches() < 2
    }
}
