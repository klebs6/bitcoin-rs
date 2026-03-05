// ---------------- [ File: bitcoinleveldb-db/src/leveldb_property_value.rs ]
crate::ix!();

pub fn leveldb_property_value(db: *mut LevelDB, propname: *const u8) -> *mut u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        propname_is_null = propname.is_null(),
        "leveldb_property_value entry"
    );

    unsafe {
        if db.is_null() || propname.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_property_value received null input"
            );
            return core::ptr::null_mut();
        }

        let cstr = std::ffi::CStr::from_ptr(propname as *const core::ffi::c_char);
        let prop: String = cstr.to_string_lossy().into_owned();

        let mut tmp: String = String::new();
        let ok = (*db)
            .rep()
            .borrow_mut()
            .get_property(prop.as_str(), (&mut tmp) as *mut String);

        if ok {
            let bytes = tmp.as_bytes();
            let len = bytes.len();

            let out = libc::malloc(len + 1) as *mut u8;
            if out.is_null() {
                error!(
                    target: "bitcoinleveldb_db::c_api",
                    len = (len + 1),
                    "leveldb_property_value malloc failed"
                );
                return core::ptr::null_mut();
            }

            if len > 0 {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, len);
            }
            *out.add(len) = 0;

            trace!(
                target: "bitcoinleveldb_db::c_api",
                len = len,
                ptr = (out as usize),
                "leveldb_property_value ok"
            );
            out
        } else {
            trace!(
                target: "bitcoinleveldb_db::c_api",
                property = %prop,
                "leveldb_property_value not found/unsupported"
            );
            core::ptr::null_mut()
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_property_value_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_property_value_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__property_value_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_property_value_rs__null_inputs_return_null() {
        unsafe {
            let p: *mut u8 = leveldb_property_value(core::ptr::null_mut(), core::ptr::null());
            assert!(p.is_null());
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_property_value_rs__supported_or_unsupported_property_is_safe_and_freeable_when_non_null() {
        unsafe {
            let opt: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!opt.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(opt, 1u8);

            let dbname_bytes: Vec<u8> = bitcoinleveldb_db__leveldb_property_value_rs__make_unique_dbname_bytes();
            let mut oerr: *mut u8 = core::ptr::null_mut();
            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let prop: &[u8] = b"leveldb.stats\0";
            let out: *mut u8 = leveldb_property_value(db, prop.as_ptr());

            if out.is_null() {
                assert!(true);
            } else {
                let mut found_nul: bool = false;
                let mut i: usize = 0usize;
                while i < 16384usize {
                    let b: u8 = *out.add(i);
                    if b == 0u8 {
                        found_nul = true;
                        break;
                    }
                    i = i + 1;
                }
                assert!(found_nul);
                crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);
            }

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(opt, dbname_bytes.as_ptr(), (&mut derr) as *mut *mut u8);
            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            crate::leveldb_options::leveldb_options_destroy(opt);
        }
    }
}
