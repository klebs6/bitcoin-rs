// ---------------- [ File: bitcoinleveldb-dbimpl/src/cleanup_compaction.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState) {
        self.mutex.assert_held();

        unsafe {
            if !(*compact).builder().is_null() {
                // May happen if we get a shutdown call in the middle of compaction
                (*(*compact).builder()).abandon();
                drop(Box::from_raw((*compact).builder()));
                (*compact).set_builder(core::ptr::null_mut());
            } else {
                assert!((*compact).outfile().is_null());
            }

            if !(*compact).outfile().is_null() {
                drop(Box::from_raw((*compact).outfile()));
                (*compact).set_outfile(core::ptr::null_mut());
            }

            for out in (*compact).outputs().iter() {
                self.pending_outputs().remove(out.number());
            }

            drop(Box::from_raw(compact));
        }
    }
}
