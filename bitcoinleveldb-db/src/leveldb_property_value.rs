// ---------------- [ File: bitcoinleveldb-db/src/leveldb_property_value.rs ]
crate::ix!();

pub fn leveldb_property_value(
        db:       *mut LevelDB,
        propname: *const u8) -> *mut u8 {
    
    todo!();
        /*
            std::string tmp;
          if (db->rep->GetProperty(Slice(propname), &tmp)) {
            // We use strdup() since we expect human readable output.
            return strdup(tmp.c_str());
          } else {
            return nullptr;
          }
        */
}

pub fn leveldb_property_value(db: *mut LevelDB, propname: *const u8) -> *mut u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_property_value entry";
        "db_is_null" => db.is_null(),
        "propname_is_null" => propname.is_null()
    );

    unsafe {
        if db.is_null() || propname.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_property_value received null input");
            return core::ptr::null_mut();
        }

        let cstr = std::ffi::CStr::from_ptr(propname as *const core::ffi::c_char);
        let prop: String = cstr.to_string_lossy().into_owned();

        let mut tmp: String = String::new();
        let ok = (*db)
            .rep
            .borrow_mut()
            .get_property(prop.as_str(), (&mut tmp) as *mut String);

        if ok {
            // strdup-like: allocate NUL-terminated copy.
            let bytes = tmp.as_bytes();
            let len = bytes.len();

            let out = libc::malloc(len + 1) as *mut u8;
            if out.is_null() {
                error!(target: "bitcoinleveldb_db::c_api", "leveldb_property_value malloc failed"; "len" => len + 1);
                return core::ptr::null_mut();
            }

            if len > 0 {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, len);
            }
            *out.add(len) = 0;

            trace!(target: "bitcoinleveldb_db::c_api", "leveldb_property_value ok"; "len" => len, "ptr" => (out as usize));
            out
        } else {
            trace!(target: "bitcoinleveldb_db::c_api", "leveldb_property_value not found/unsupported"; "property" => %prop);
            core::ptr::null_mut()
        }
    }

    /*
        std::string tmp;
      if (db->rep->GetProperty(Slice(propname), &tmp)) {
        // We use strdup() since we expect human readable output.
        return strdup(tmp.c_str());
      } else {
        return nullptr;
      }
    */
}
