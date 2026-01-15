// ---------------- [ File: bitcoinleveldb-dbimpl/src/write_level0_table.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn write_level_0table(
        &mut self,
        mem: *mut MemTable,
        edit: *mut VersionEdit,
        base: *mut Version,
    ) -> crate::Status {
        self.mutex.assert_held();

        let env_rc: Rc<RefCell<dyn Env>> = match self.options.env().as_ref() {
            Some(e) => e.clone(),
            None => {
                tracing::error!("write_level_0table: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        let start_micros: u64 = self.env.as_mut().now_micros();

        let mut meta: FileMetaData = Default::default();
        meta.set_number(unsafe { (*(self.versions as *mut VersionSet)).new_file_number() });

        self.pending_outputs.insert(*meta.number());

        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };

        tracing::info!(file_number = meta.number(), "Level-0 table started");

        let mut s: Status = Status::ok();

        unsafe {
            self.mutex.unlock();
        }

        s = build_table(
            &self.dbname,
            env_rc.clone(),
            &self.options,
            self.table_cache as *mut TableCache,
            iter,
            &mut meta,
        );

        self.mutex.lock();

        tracing::info!(
            file_number = meta.number(),
            bytes = meta.file_size(),
            status = %s.to_string(),
            "Level-0 table finished"
        );

        unsafe {
            drop(Box::from_raw(iter));
        }

        self.pending_outputs.remove(meta.number());

        // Note that if file_size is zero, the file has been deleted and
        // should not be added to the manifest.
        let mut level: i32 = 0;

        if s.is_ok() && *meta.file_size() > 0 {
            let min_user_key: Slice = meta.smallest().user_key();
            let max_user_key: Slice = meta.largest().user_key();

            if !base.is_null() {
                level =
                    unsafe { (*base).pick_level_for_mem_table_output(&min_user_key, &max_user_key) };
            }

            unsafe {
                (*edit).add_file(
                    level,
                    *meta.number(),
                    *meta.file_size(),
                    meta.smallest(),
                    meta.largest(),
                );
            }
        }

        let mut stats: CompactionStats = Default::default();
        stats.set_micros((self.env.as_mut().now_micros() - start_micros) as i64);
        stats.set_bytes_written(*meta.file_size() as i64);

        self.stats[level as usize].add(&stats);

        s
    }
}

#[cfg(test)]
mod dbimpl_write_level0_table_contract_suite {
    use super::*;

    #[traced_test]
    fn write_level_0table_signature_is_stable_for_dbimpl_public_method() {
        tracing::info!("Asserting DBImpl::write_level_0table signature is stable");

        type WriteL0Sig = fn(&mut DBImpl, *mut MemTable, *mut VersionEdit, *mut Version) -> Status;
        let _sig: WriteL0Sig = DBImpl::write_level_0table;

        tracing::debug!("DBImpl::write_level_0table signature check compiled");
    }

    #[traced_test]
    fn file_metadata_number_and_file_size_are_borrowed_u64_references() {
        let mut meta: FileMetaData = Default::default();

        let n0: &u64 = meta.number();
        let sz0: &u64 = meta.file_size();

        tracing::debug!(number = *n0, file_size = *sz0, "Default FileMetaData getters");
        assert_eq!(*n0, 0);
        assert_eq!(*sz0, 0);

        meta.set_number(42);
        meta.set_file_size(1_234);

        let n1: &u64 = meta.number();
        let sz1: &u64 = meta.file_size();

        tracing::debug!(number = *n1, file_size = *sz1, "Updated FileMetaData getters");
        assert_eq!(*n1, 42);
        assert_eq!(*sz1, 1_234);
    }

    #[traced_test]
    fn pending_outputs_insert_requires_u64_and_accepts_deref_of_meta_number() {
        use std::collections::HashSet;

        let mut meta: FileMetaData = Default::default();
        meta.set_number(7);

        let mut pending: HashSet<u64> = HashSet::new();

        let n_ref: &u64 = meta.number();
        tracing::trace!(number_ref = *n_ref, "Inserting dereferenced meta.number() into HashSet<u64>");
        assert!(pending.insert(*n_ref));
        assert!(pending.contains(n_ref));

        tracing::debug!(pending_len = pending.len(), "Pending outputs set size after insert");
        assert_eq!(pending.len(), 1);
    }

    #[traced_test]
    fn compactionstats_add_requires_reference_argument_by_interface_contract() {
        tracing::info!("Asserting CompactionStats::add takes &CompactionStats");

        type AddSig = fn(&mut CompactionStats, &CompactionStats);
        let _sig: AddSig = CompactionStats::add;

        let mut a: CompactionStats = Default::default();
        let b: CompactionStats = Default::default();

        // Runtime smoke: call with reference as required.
        a.add(&b);
        tracing::debug!("CompactionStats::add(&CompactionStats) invoked successfully");
    }

    #[traced_test]
    fn version_pick_level_for_mem_table_output_requires_borrowed_slices() {
        tracing::info!("Asserting Version::pick_level_for_mem_table_output takes (&Slice, &Slice)");

        type PickSig = fn(&mut Version, &Slice, &Slice) -> i32;
        let _sig: PickSig = Version::pick_level_for_mem_table_output;

        tracing::debug!("Version::pick_level_for_mem_table_output signature check compiled");
    }

    #[traced_test]
    fn file_size_reference_can_be_dereferenced_and_cast_to_i64_for_stats_bytes_written() {
        let mut meta: FileMetaData = Default::default();
        meta.set_file_size(9_876);

        let sz_ref: &u64 = meta.file_size();
        let bytes_written: i64 = *sz_ref as i64;

        tracing::debug!(
            file_size_u64 = *sz_ref,
            bytes_written_i64 = bytes_written,
            "Dereferenced file_size() and cast to i64"
        );

        assert_eq!(bytes_written, 9_876_i64);
    }

    #[traced_test]
    fn const_table_cache_pointer_can_be_cast_to_mut_for_ffi_boundary_call_sites() {
        tracing::info!("Validating *const TableCache -> *mut TableCache cast compiles for boundary APIs");

        let p_const: *const TableCache = core::ptr::null::<TableCache>();
        let p_mut: *mut TableCache = p_const as *mut TableCache;

        tracing::trace!(p_const = ?p_const, p_mut = ?p_mut, "Pointer cast results");
        assert!(p_mut.is_null());
    }
}
