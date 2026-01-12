// ---------------- [ File: bitcoinleveldb-dbimpl/src/install_compaction_results.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status {
        self.mutex.assert_held();

        tracing::info!(
            n0 = unsafe { (*(*compact).compaction).num_input_files(0) },
            l0 = unsafe { (*(*compact).compaction).level() },
            n1 = unsafe { (*(*compact).compaction).num_input_files(1) },
            l1 = unsafe { (*(*compact).compaction).level() + 1 },
            total_bytes = unsafe { (*compact).total_bytes },
            "Compacted inputs => outputs"
        );

        unsafe {
            // Add compaction outputs
            (*(*compact).compaction).add_input_deletions((*(*compact).compaction).edit());

            let level: i32 = (*(*compact).compaction).level();
            for out in (*compact).outputs.iter() {
                (*(*(*compact).compaction).edit()).add_file(
                    level + 1,
                    out.number,
                    out.file_size,
                    out.smallest.clone(),
                    out.largest.clone(),
                );
            }

            (*self.versions).log_and_apply((*(*compact).compaction).edit(), &mut self.mutex)
        }
    }
}

#[cfg(test)]
#[disable]
mod install_compaction_results_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn compaction_results_installation_preserves_visibility_and_property_sstables_is_queryable() {
        let (dbname, mut db) =
            open_dbimpl_for_test("compaction_results_installation_preserves_visibility_and_property_sstables_is_queryable");

        fill_sequential(&mut *db, "ic", 350, 256);
        force_manual_compaction_full_range(&mut *db);

        let mut sst: String = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(&mut *db, "leveldb.sstables", (&mut sst) as *mut String);
        tracing::info!(ok, len = sst.len(), "sstables after compaction");
        assert!(ok, "sstables property should be available after compaction");

        assert_read_eq(&mut *db, "ic00000000", &"v".repeat(256));
        assert_read_eq(&mut *db, "ic00000349", &"v".repeat(256));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
