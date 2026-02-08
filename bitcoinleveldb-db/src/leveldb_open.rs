// ---------------- [ File: bitcoinleveldb-db/src/leveldb_open.rs ]
crate::ix!();

pub fn leveldb_open(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8) -> *mut LevelDB {
    
    todo!();
        /*
            DB* db;
          if (SaveError(errptr, DB::Open(options->rep, std::string(name), &db))) {
            return nullptr;
          }
          leveldb_t* result = new leveldb_t;
          result->rep = db;
          return result;
        */
}

pub fn leveldb_open(
    options: *const LevelDBOptions,
    name: *const u8,
    errptr: *mut *mut u8,
) -> *mut LevelDB {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_open entry"; "options_is_null" => options.is_null(), "name_is_null" => name.is_null());

    unsafe {
        if options.is_null() || name.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_open received null input pointer");
            let msg = Slice::from_str("leveldb_open: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            return core::ptr::null_mut();
        }

        let opts: &Options = &(*options).rep;

        let cstr = std::ffi::CStr::from_ptr(name as *const core::ffi::c_char);
        let dbname: String = cstr.to_string_lossy().into_owned();

        debug!(target: "bitcoinleveldb_db::c_api", "leveldb_open dbname parsed"; "dbname" => %dbname);

        // Create a DBImpl and open it through the DB interface.
        let rep: Rc<RefCell<dyn DB>> = Rc::new(RefCell::new(DBImpl::new(opts, &dbname)));

        let status = {
            let mut db_mut = rep.borrow_mut();

            // Provide an out-parameter consistent with the translated DBOpen interface.
            let mut out: *mut dyn DB = (&mut *db_mut) as *mut dyn DB;

            db_mut.open(opts, &dbname, (&mut out) as *mut *mut dyn DB)
        };

        if save_error(errptr, &status) {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_open failed"; "dbname" => %dbname, "status" => %status.to_string());
            return core::ptr::null_mut();
        }

        let result = Box::new(LevelDB { rep });
        let p = Box::into_raw(result);

        info!(target: "bitcoinleveldb_db::c_api", "leveldb_open ok"; "dbname" => %dbname, "handle" => (p as usize));

        p
    }

    /*
        DB* db;
      if (SaveError(errptr, DB::Open(options->rep, std::string(name), &db))) {
        return nullptr;
      }
      leveldb_t* result = new leveldb_t;
      result->rep = db;
      return result;
    */
}
