// ---------------- [ File: bitcoinleveldb-dbimpl/src/install_compaction_results.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status {
        self.mutex.assert_held();

        tracing::info!(
            n0 = unsafe { (*(*compact).compaction()).num_input_files(0) },
            l0 = unsafe { (*(*compact).compaction()).level() },
            n1 = unsafe { (*(*compact).compaction()).num_input_files(1) },
            l1 = unsafe { (*(*compact).compaction()).level() + 1 },
            total_bytes = unsafe { (*compact).total_bytes() },
            "Compacted inputs => outputs"
        );

        unsafe {
            // Add compaction outputs
            (*(*compact).compaction()).add_input_deletions((*(*compact).compaction()).edit());

            let level: i32 = (*(*compact).compaction()).level();
            for out in (*compact).outputs().iter() {
                (*(*(*compact).compaction()).edit()).add_file(
                    level + 1,
                    *out.number(),
                    *out.file_size(),
                    out.smallest(),
                    out.largest(),
                );
            }

            (*self.versions).log_and_apply((*(*compact).compaction()).edit(), &mut self.mutex)
        }
    }
}
