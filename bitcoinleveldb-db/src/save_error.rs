// ---------------- [ File: bitcoinleveldb-db/src/save_error.rs ]
crate::ix!();

pub fn save_error(
        errptr: *mut *mut u8,
        s:      &Status) -> bool {
    
    todo!();
        /*
            assert(errptr != nullptr);
          if (s.ok()) {
            return false;
          } else if (*errptr == nullptr) {
            *errptr = strdup(s.ToString().c_str());
          } else {
            // TODO(sanjay): Merge with existing error?
            free(*errptr);
            *errptr = strdup(s.ToString().c_str());
          }
          return true;
        */
}

pub fn save_error(errptr: *mut *mut u8, s: &Status) -> bool {
    trace!(target: "bitcoinleveldb_db::c_api", "SaveError entry"; "status_ok" => s.is_ok());

    assert!(!errptr.is_null());

    if s.is_ok() {
        trace!(target: "bitcoinleveldb_db::c_api", "SaveError exit (no error)");
        return false;
    }

    // Allocate a NUL-terminated copy of Status::to_string().
    let msg = s.to_string();
    let bytes = msg.as_bytes();
    let len = bytes.len();

    unsafe {
        let new_err: *mut u8 = libc::malloc(len + 1) as *mut u8;
        if new_err.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "SaveError malloc failed"; "len" => len + 1);
            // Best-effort: keep existing errptr unchanged.
            return true;
        }

        if len > 0 {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), new_err, len);
        }
        *new_err.add(len) = 0;

        if (*errptr).is_null() {
            *errptr = new_err;
        } else {
            // TODO(sanjay): Merge with existing error?
            libc::free(*errptr as *mut core::ffi::c_void);
            *errptr = new_err;
        }
    }

    trace!(target: "bitcoinleveldb_db::c_api", "SaveError exit (error stored)"; "msg_len" => len);

    true

    /*
        assert(errptr != nullptr);
      if (s.ok()) {
        return false;
      } else if (*errptr == nullptr) {
        *errptr = strdup(s.ToString().c_str());
      } else {
        // TODO(sanjay): Merge with existing error?
        free(*errptr);
        *errptr = strdup(s.ToString().c_str());
      }
      return true;
    */
}
