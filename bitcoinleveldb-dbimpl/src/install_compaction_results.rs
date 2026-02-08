// ---------------- [ File: bitcoinleveldb-dbimpl/src/install_compaction_results.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status {
        self.mutex.assert_held();

        let compaction: *mut Compaction = unsafe { *(*compact).compaction() as *mut Compaction };

        tracing::info!(
            n0 = unsafe { (*compaction).num_input_files(0) },
            l0 = unsafe { (*compaction).level() },
            n1 = unsafe { (*compaction).num_input_files(1) },
            l1 = unsafe { (*compaction).level() + 1 },
            total_bytes = unsafe { (*compact).total_bytes() },
            "Compacted inputs => outputs"
        );

        unsafe {
            let level: i32 = (*compaction).level();

            // Delete compaction inputs.
            for which in 0..2 {
                let n: i32 = (*compaction).num_input_files(which);
                for i in 0..n {
                    let f: *mut FileMetaData = (*compaction).input(which, i);
                    (*(*compaction).edit()).delete_file(level + which, *(*f).number());
                }
            }

            // Add compaction outputs.
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

    }
}

#[cfg(test)]
mod install_compaction_results_interface_contract_suite {
    use super::*;

    #[traced_test]
    fn install_compaction_results_signature_is_stable() {
        tracing::info!("Asserting DBImpl::install_compaction_results signature is stable");
        type Sig = fn(&mut DBImpl, *mut CompactionState) -> Status;
        let _sig: Sig = DBImpl::install_compaction_results;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn install_compaction_results_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::install_compaction_results method item is addressable");
        let _m = DBImpl::install_compaction_results;
        let _ = _m;
    }
}
