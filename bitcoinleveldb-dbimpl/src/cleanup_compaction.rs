// ---------------- [ File: bitcoinleveldb-dbimpl/src/cleanup_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState) {
        self.mutex.assert_held();

        unsafe {
            let builder_ptr: *mut TableBuilder = *(*compact).builder();

            if let Some(builder) = builder_ptr.as_mut() {
                // May happen if shutdown occurs mid-compaction
                builder.abandon();
                drop(Box::from_raw(builder_ptr));
                (*compact).set_builder(core::ptr::null_mut());
            }

            for out in (*compact).outputs().iter() {
                self.pending_outputs.remove(out.number());
            }

            drop(Box::from_raw(compact));
        }
    }
}
