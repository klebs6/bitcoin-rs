// ---------------- [ File: bitcoinleveldbt-faultinjection/src/file_state.rs ]
crate::ix!();

#[derive(Eq,PartialEq,Clone,Getters,Setters,MutGetters,Builder)]
#[builder(setter(into))]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct FileState {
    filename:          String,
    pos:               i64,
    pos_at_last_sync:  i64,
    pos_at_last_flush: i64,
}

impl Default for FileState {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_default_entry"
        );

        let out = Self {
            filename: String::new(),
            pos: -1i64,
            pos_at_last_sync: -1i64,
            pos_at_last_flush: -1i64,
        };

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_default_exit"
        );

        out
    }
}

impl FileState {
    pub fn new(filename: &String) -> Self {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_new_entry",
            filename_len = filename.len()
        );

        let out = Self {
            filename: filename.clone(),
            pos: -1i64,
            pos_at_last_sync: -1i64,
            pos_at_last_flush: -1i64,
        };

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_new_exit"
        );

        out
    }

    pub fn is_fully_synced(&self) -> bool {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_is_fully_synced_entry",
            pos = self.pos,
            pos_at_last_sync = self.pos_at_last_sync
        );

        let out = self.pos <= 0i64 || self.pos == self.pos_at_last_sync;

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_is_fully_synced_exit",
            result = out
        );

        out
    }

    pub fn drop_unsynced_data(&self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_drop_unsynced_data_entry",
            filename = %self.filename,
            pos = self.pos,
            pos_at_last_sync = self.pos_at_last_sync
        );

        let sync_pos: i64 = if self.pos_at_last_sync == -1i64 {
            0i64
        } else {
            self.pos_at_last_sync
        };

        let status = faultinjection_test_truncate(
            self.filename(),
            sync_pos as u64,
        );

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "file_state_drop_unsynced_data_exit",
            filename = %self.filename,
            ok = status.is_ok(),
            sync_pos = sync_pos
        );

        status
    }
}
