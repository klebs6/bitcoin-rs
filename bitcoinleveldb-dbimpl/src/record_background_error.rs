// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_background_error.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn record_background_error(&mut self, s: &Status)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      if (bg_error_.ok()) {
        bg_error_ = s;
        background_work_finished_signal_.SignalAll();
      }
        */
    }
}
