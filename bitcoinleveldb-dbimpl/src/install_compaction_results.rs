// ---------------- [ File: bitcoinleveldb-dbimpl/src/install_compaction_results.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status { 
        todo!(); 
        /*
        self.mutex.assert_held();

        let compaction: *const Compaction = (*compact).compaction();

        tracing::info!(
            n0 = unsafe { (*compaction).num_input_files(0) },
            l0 = unsafe { (*compaction).level() },
            n1 = unsafe { (*compaction).num_input_files(1) },
            l1 = unsafe { (*compaction).level() + 1 },
            total_bytes = unsafe { (*compact).total_bytes() },
            "Compacted inputs => outputs"
        );

        unsafe {
            // Add compaction outputs
            (*compaction).add_input_deletions((*compaction).edit());

            let level: i32 = (*compaction).level();
            for out in (*compact).outputs().iter() {
                (*(*compaction).edit()).add_file(
                    level + 1,
                    *out.number(),
                    *out.file_size(),
                    out.smallest(),
                    out.largest(),
                );
            }

            (*self.versions).log_and_apply((*compaction).edit(), &mut self.mutex)
        }
                                                                                                   */
    }
}
