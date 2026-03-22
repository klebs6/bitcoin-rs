// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/recording_writable_file_state.rs ]
crate::ix!();

/// Guarantees zero is the only valid initial counter state and the append buffer preserves exact
/// append-call byte order for the lifetime of the shared recording state.
#[derive(Default, Debug)]
pub struct WritableFileRecordingState {
    /// Bytes appended so far, concatenated in exact append-call order. Consumers must not infer
    /// flush or sync boundaries from this buffer alone.
    appended_bytes: Vec<u8>,

    /// Total number of `append` invocations observed so far.
    append_call_count: usize,

    /// Total number of `close` invocations observed so far.
    close_call_count: usize,

    /// Total number of `flush` invocations observed so far.
    flush_call_count: usize,

    /// Total number of `sync` invocations observed so far.
    sync_call_count: usize,
}

impl WritableFileRecordingState {
    /// Guarantees the returned slice is the exact append-order byte log without copying.
    pub fn appended_bytes(&self) -> &[u8] {
        self.appended_bytes.as_slice()
    }

    /// Guarantees the returned count is monotone non-decreasing across the state lifetime.
    pub fn append_call_count(&self) -> usize {
        self.append_call_count
    }

    /// Guarantees the returned count is monotone non-decreasing across the state lifetime.
    pub fn close_call_count(&self) -> usize {
        self.close_call_count
    }

    /// Guarantees the returned count is monotone non-decreasing across the state lifetime.
    pub fn flush_call_count(&self) -> usize {
        self.flush_call_count
    }

    /// Guarantees the returned count is monotone non-decreasing across the state lifetime.
    pub fn sync_call_count(&self) -> usize {
        self.sync_call_count
    }

    /// Postconditions: `append_call_count` increases by one and `appended_bytes` is extended by
    /// exactly the bytes in `appended_bytes`.
    pub fn record_append_bytes(
        &mut self,
        appended_bytes: &[u8],
    ) -> (usize, usize) {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_append_bytes_enter",
            appended_length = appended_bytes.len()
        );

        self.append_call_count += 1;
        self.appended_bytes.extend_from_slice(appended_bytes);

        let append_call_count = self.append_call_count;
        let total_appended_length = self.appended_bytes.len();

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_append_bytes_exit",
            append_call_count = append_call_count,
            total_appended_length = total_appended_length
        );

        (append_call_count, total_appended_length)
    }

    /// Postconditions: `close_call_count` increases by one.
    pub fn record_close_call(&mut self) -> usize {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_close_call_enter"
        );

        self.close_call_count += 1;

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_close_call_exit",
            close_call_count = self.close_call_count
        );

        self.close_call_count
    }

    /// Postconditions: `flush_call_count` increases by one.
    pub fn record_flush_call(&mut self) -> usize {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_flush_call_enter"
        );

        self.flush_call_count += 1;

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_flush_call_exit",
            flush_call_count = self.flush_call_count
        );

        self.flush_call_count
    }

    /// Postconditions: `sync_call_count` increases by one.
    pub fn record_sync_call(&mut self) -> usize {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_sync_call_enter"
        );

        self.sync_call_count += 1;

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file_state",
            event = "writable_file_recording_state_record_sync_call_exit",
            sync_call_count = self.sync_call_count
        );

        self.sync_call_count
    }
}
