// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/recording_writable_file.rs ]
crate::ix!();

/// Guarantees every writable-file trait method updates the shared recording state synchronously
/// and emits structured tracing for each externally visible state transition.
#[derive(Debug)]
pub struct WritableFileRecorder {
    /// Shared mutable recording state. The `Rc<RefCell<_>>` topology is intentionally explicit so
    /// tests can inspect the same state instance after operations complete.
    shared_state: Rc<RefCell<WritableFileRecordingState>>,

    /// Stable label used only for tracing and the `Named` trait surface.
    stable_label: String,
}

impl WritableFileRecorder {
    /// Postconditions: the returned recorder shares ownership of `shared_state` and associates it
    /// with `stable_label` for all future trace and `Named` outputs.
    pub fn bind_shared_state_with_label(
        shared_state: Rc<RefCell<WritableFileRecordingState>>,
        stable_label: &str,
    ) -> Self {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_bind_shared_state_with_label_enter",
            stable_label = stable_label
        );

        let recorder = Self {
            shared_state,
            stable_label: stable_label.to_string(),
        };

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_bind_shared_state_with_label_exit",
            stable_label = recorder.stable_label.as_str()
        );

        recorder
    }

    /// Guarantees the returned bytes are an owned copy of the exact slice contents at call time.
    pub fn copy_slice_bytes(data: &Slice) -> Vec<u8> {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_copy_slice_bytes_enter"
        );

        let owned_bytes = unsafe {
            let data_ptr: *const u8 = *data.data();
            let data_len: usize = *data.size();
            StdSlice::from_raw_parts(data_ptr, data_len).to_vec()
        };

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_copy_slice_bytes_exit",
            owned_length = owned_bytes.len()
        );

        owned_bytes
    }
}

impl WritableFileAppend for WritableFileRecorder {
    fn append(&mut self, data: &Slice) -> Status {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_append_enter",
            stable_label = self.stable_label.as_str()
        );

        let appended_bytes = Self::copy_slice_bytes(data);
        let (append_call_count, total_appended_length) =
            self.shared_state
                .borrow_mut()
                .record_append_bytes(appended_bytes.as_slice());

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_append_exit",
            stable_label = self.stable_label.as_str(),
            append_call_count = append_call_count,
            appended_length = appended_bytes.len(),
            total_appended_length = total_appended_length
        );

        Status::ok()
    }
}

impl WritableFileClose for WritableFileRecorder {
    fn close(&mut self) -> Status {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_close_enter",
            stable_label = self.stable_label.as_str()
        );

        let close_call_count = self.shared_state.borrow_mut().record_close_call();

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_close_exit",
            stable_label = self.stable_label.as_str(),
            close_call_count = close_call_count
        );

        Status::ok()
    }
}

impl WritableFileFlush for WritableFileRecorder {
    fn flush(&mut self) -> Status {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_flush_enter",
            stable_label = self.stable_label.as_str()
        );

        let flush_call_count = self.shared_state.borrow_mut().record_flush_call();

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_flush_exit",
            stable_label = self.stable_label.as_str(),
            flush_call_count = flush_call_count
        );

        Status::ok()
    }
}

impl WritableFileSync for WritableFileRecorder {
    fn sync(&mut self) -> Status {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_sync_enter",
            stable_label = self.stable_label.as_str()
        );

        let sync_call_count = self.shared_state.borrow_mut().record_sync_call();

        trace!(
            target: "bitcoinleveldb_versionsettestutil::recording_writable_file",
            event = "writable_file_recorder_sync_exit",
            stable_label = self.stable_label.as_str(),
            sync_call_count = sync_call_count
        );

        Status::ok()
    }
}

impl Named for WritableFileRecorder {
    fn name(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            "[recording-writablefile:{}]",
            self.stable_label
        ))
    }
}

impl WritableFile for WritableFileRecorder {}
