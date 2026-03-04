// ---------------- [ File: bitcoinleveldb-dbimpl/src/open_compaciton_output_file.rs ]
crate::ix!();

impl DBImpl {
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status { 
        assert!(!compact.is_null());
        assert!(unsafe { (*compact).builder() }.is_null());

        let file_number: u64;

        self.mutex.lock();
        file_number = unsafe { (*self.versions).new_file_number() };
        self.pending_outputs.insert(file_number);

        unsafe {

            let out = CompactionStateOutputBuilder::default()
                .number(file_number)
                .smallest(InternalKey::new_empty())
                .largest(InternalKey::new_empty())
                .file_size(0)
                .build()
                .unwrap();

            (*compact).outputs_mut().push(out);
        }

        unsafe { self.mutex.unlock() };

        // Make the output file
        let fname: String = table_file_name(&self.dbname, file_number);

        let mut outfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let mut s: Status = self
            .env
            .as_mut()
            .new_writable_file(&fname, &mut outfile_ptr);

        if s.is_ok() {
            unsafe {
                let outfile_box: Box<dyn WritableFile> = *Box::from_raw(outfile_ptr);
                let outfile_rc: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(outfile_box));
                (*compact).set_outfile(outfile_rc);

                let file_ptr: *mut dyn WritableFile = {
                    let mut borrow = (*compact).outfile().borrow_mut();
                    &mut *borrow as &mut dyn WritableFile as *mut dyn WritableFile
                };

                (*compact).set_builder(
                    Box::into_raw(Box::new(TableBuilder::new(&self.options, file_ptr)))
                );
            }
        }

        s
    }
}

#[cfg(test)]
mod open_compaction_output_file_interface_and_smoke_suite {
    use super::*;

    fn build_temp_db_path_for_open_compaction_output_file_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_or_panic_for_open_compaction_output_file_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run open_compaction_output_file suite");
            panic!();
        }

        options
    }

    #[traced_test]
    fn open_compaction_output_file_signature_is_stable() {
        tracing::info!("Asserting DBImpl::open_compaction_output_file signature is stable");
        type Sig = fn(&mut DBImpl, *mut CompactionState) -> Status;
        let _sig: Sig = DBImpl::open_compaction_output_file;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn open_compaction_output_file_panics_on_null_compaction_state_pointer() {
        let dbname = build_temp_db_path_for_open_compaction_output_file_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_open_compaction_output_file_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        tracing::info!("Calling open_compaction_output_file(NULL); expecting panic");
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = db.open_compaction_output_file(core::ptr::null_mut());
        }))
        .is_err();

        tracing::debug!(panicked, "Observed panic result for NULL compact pointer");
        assert!(panicked, "open_compaction_output_file must assert compact is non-null");

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn open_compaction_output_file_creates_builder_and_records_pending_output() {
        let dbname = build_temp_db_path_for_open_compaction_output_file_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_open_compaction_output_file_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let compaction_ptr: *mut Compaction = Box::into_raw(Box::new(Compaction::new(
            &options as *const Options,
            1,
        )));

        let state: CompactionState = CompactionState::new(compaction_ptr);
        let state_ptr: *mut CompactionState = Box::into_raw(Box::new(state));

        tracing::info!("Calling DBImpl::open_compaction_output_file on fresh CompactionState");
        let s: Status = db.open_compaction_output_file(state_ptr);

        tracing::debug!(status = %s.to_string(), "open_compaction_output_file returned");
        assert!(s.is_ok(), "open_compaction_output_file must succeed in a writable directory");

        let builder_ptr: *mut TableBuilder = unsafe { *(*state_ptr).builder() };
        tracing::debug!(builder_ptr = ?builder_ptr, "CompactionState.builder after open");
        assert!(!builder_ptr.is_null(), "open_compaction_output_file must set a non-null builder");

        let out_ptr: *mut CompactionStateOutput = unsafe { (*state_ptr).current_output() };
        let file_number: u64 = unsafe { *(*out_ptr).number() };

        tracing::debug!(file_number, "Allocated output file number");
        assert_ne!(file_number, 0, "Allocated file numbers must be non-zero");

        db.mutex.lock();
        let pending = db.pending_outputs.contains(&file_number);
        tracing::debug!(pending, file_number, "pending_outputs membership after open");
        assert!(pending, "open_compaction_output_file must insert the output file into pending_outputs");
        unsafe { db.mutex.unlock() };

        let table_path: String = table_file_name(&dbname, file_number);

        db.mutex.lock();
        tracing::info!("Cleaning up CompactionState via DBImpl::cleanup_compaction");
        db.cleanup_compaction(state_ptr);
        unsafe { db.mutex.unlock() };

        unsafe {
            drop(Box::from_raw(compaction_ptr));
        }

        let _ = std::fs::remove_file(&table_path);
        drop(db);

        let _ = std::fs::remove_dir_all(&dbname);
    }
}
