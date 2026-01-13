// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_background_error.rs ]
crate::ix!();

impl DBImpl {

    pub fn record_background_error(&mut self, s: &Status) { 
        self.mutex.assert_held();

        if self.bg_error.is_ok() {
            self.bg_error = s.clone();
            self.background_work_finished_signal.signal_all();
        }
    }
}
