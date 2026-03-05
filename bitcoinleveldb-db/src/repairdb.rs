// ---------------- [ File: bitcoinleveldb-db/src/repairdb.rs ]
crate::ix!();

/**
  | If a DB cannot be opened, you may attempt to
  | call this method to resurrect as much of the
  | contents of the database as possible.
  |
  | Some data may be lost, so be careful when
  | calling this function on a database that
  | contains important information.
  */
pub fn leveldb_repairdb_inner(dbname: &String, options: &Options) -> crate::Status {
    trace!(
        target: "bitcoinleveldb_db::db",
        dbname = %dbname,
        has_env = options.env().is_some(),
        "RepairDB entry"
    );

    let status: crate::Status = repairdb(dbname, options);

    if status.is_ok() {
        info!(
            target: "bitcoinleveldb_db::db",
            dbname = %dbname,
            "RepairDB ok"
        );
    } else {
        warn!(
            target: "bitcoinleveldb_db::db",
            dbname = %dbname,
            status = %status.to_string(),
            "RepairDB failed"
        );
    }

    trace!(
        target: "bitcoinleveldb_db::db",
        dbname = %dbname,
        ok = status.is_ok(),
        "RepairDB exit"
    );

    status
}

#[cfg(test)]
mod bitcoinleveldb_db__repairdb_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__repairdb_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__repairdb_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__repairdb_rs__repairdb_runs_on_existing_db_directory() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__repairdb_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );
            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            assert!(!wopt.is_null());

            let key: [u8; 2] = [b'k', b'1'];
            let val: [u8; 2] = [b'v', b'1'];

            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut oerr) as *mut *mut u8,
            );
            assert!(oerr.is_null());

            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);
            crate::leveldb_close::leveldb_close(db);

            let cstr =
                std::ffi::CStr::from_ptr(dbname_bytes.as_ptr() as *const core::ffi::c_char);
            let dbname: String = cstr.to_string_lossy().into_owned();

            let st: crate::Status = super::leveldb_repairdb_inner(&dbname, (*options).rep());
            assert!(st.is_ok());

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );
            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}
