// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_db.rs ]
crate::ix!();

impl DBImpl {

    pub fn newdb(&mut self) -> crate::Status {
        let env_rc: Rc<RefCell<dyn Env>> = match self.options.env().as_ref() {
            Some(e) => e.clone(),
            None => {
                tracing::error!("DBImpl::newdb: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        let mut new_db: VersionEdit = Default::default();

        let comparator = self.user_comparator();
        let comparator_name_cow = comparator.name();
        let comparator_name_string: String = comparator_name_cow.into_owned();
        let comparator_name_slice: Slice = Slice::from_str(&comparator_name_string);

        new_db.set_comparator_name(&comparator_name_slice);
        new_db.set_log_number(0);
        new_db.set_next_file(2);
        new_db.set_last_sequence(0);

        let manifest: String = descriptor_file_name(&self.dbname, 1);
        let mut file_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let mut s: Status = self
            .env
            .as_mut()
            .new_writable_file(&manifest, &mut file_ptr);

        if !s.is_ok() {
            return s;
        }

        if file_ptr.is_null() {
            tracing::error!(
                file = %manifest,
                "Env::new_writable_file returned ok but output file pointer was null"
            );
            let msg: Slice = Slice::from_str("new_writable_file returned ok but output was null");
            let fname_slice: Slice = Slice::from_str(&manifest);
            return Status::corruption(&msg, Some(&fname_slice));
        }

        let file_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(file_ptr) };
        let file_rc: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(file_box));

        {
            let mut log: LogWriter = LogWriter::new(file_rc.clone(), 0);

            let mut record: String = String::new();
            new_db.encode_to(&mut record);

            let record_slice: Slice = Slice::from_str(&record);
            s = log.add_record(&record_slice);

            if s.is_ok() {
                s = file_rc.borrow_mut().close();
            }
        }

        if s.is_ok() {
            // Make "CURRENT" file that points to the new manifest file.
            s = set_current_file(env_rc.clone(), &self.dbname, 1);
        } else {
            let _ = self.env.as_mut().delete_file(&manifest);
        }

        s
    }
}

#[cfg(test)]
mod new_db_interface_and_filesystem_suite {
    use super::*;

    fn build_temp_db_path_for_new_db_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_or_panic_for_new_db_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run newdb suite");
            panic!();
        }

        options
    }

    #[traced_test]
    fn newdb_signature_is_stable() {
        tracing::info!("Asserting DBImpl::newdb signature is stable");
        type Sig = fn(&mut DBImpl) -> Status;
        let _sig: Sig = DBImpl::newdb;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn newdb_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::newdb method item is addressable");
        let _m = DBImpl::newdb;
        let _ = _m;
    }

    #[traced_test]
    fn newdb_creates_manifest_and_current_file_for_empty_directory() {
        let dbname = build_temp_db_path_for_new_db_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_new_db_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        tracing::info!(dbname = %dbname, "Calling DBImpl::newdb()");
        let s: Status = db.newdb();

        tracing::debug!(status = %s.to_string(), "DBImpl::newdb returned");
        assert!(s.is_ok(), "newdb must succeed in a writable, empty directory");

        let manifest_path: String = descriptor_file_name(&dbname, 1);
        let current_path: String = current_file_name(&dbname);

        tracing::debug!(manifest_path = %manifest_path, current_path = %current_path, "Expected files");

        assert!(
            std::fs::metadata(&manifest_path).is_ok(),
            "MANIFEST file must exist after newdb"
        );
        assert!(
            std::fs::metadata(&current_path).is_ok(),
            "CURRENT file must exist after newdb"
        );

        let current_contents = std::fs::read_to_string(&current_path).unwrap_or_else(|e| {
            tracing::error!(path = %current_path, error = %format!("{:?}", e), "Failed reading CURRENT");
            panic!();
        });

        tracing::debug!(current_contents = %current_contents.trim_end(), "CURRENT contents");
        assert!(
            current_contents.contains("MANIFEST-000001"),
            "CURRENT must point at MANIFEST-000001"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn newdb_returns_error_when_options_env_is_none() {
        let dbname = build_temp_db_path_for_new_db_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_new_db_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        db.options.set_env(None);

        let s: Status = db.newdb();

        tracing::debug!(status = %s.to_string(), "DBImpl::newdb returned with env=None");
        assert!(
            !s.is_ok(),
            "newdb must return non-OK when Options.env is None"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
