// ---------------- [ File: bitcoinleveldb-db/src/save_error.rs ]
crate::ix!();

pub fn save_error(errptr: *mut *mut u8, s: &Status) -> bool {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        status_ok = s.is_ok(),
        "SaveError entry"
    );

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
            error!(
                target: "bitcoinleveldb_db::c_api",
                len = (len + 1),
                "SaveError malloc failed"
            );
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

    trace!(
        target: "bitcoinleveldb_db::c_api",
        msg_len = len,
        "SaveError exit (error stored)"
    );

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

#[cfg(test)]
mod bitcoinleveldb_db__save_error_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__save_error_rs__ok_status_returns_false_and_does_not_allocate() {
        let msg: Slice = Slice::from_str("ok");
        let st: Status = crate::Status::ok();

        let mut err: *mut u8 = core::ptr::null_mut();
        let r: bool = save_error((&mut err) as *mut *mut u8, &st);

        assert!(!r);
        assert!(err.is_null());

        drop(msg);
    }

    #[traced_test]
    fn bitcoinleveldb_db__save_error_rs__non_ok_status_allocates_and_replaces_previous_error() {
        let msg1: Slice = Slice::from_str("bad1");
        let st1: Status = crate::Status::invalid_argument(&msg1, None);

        let mut err: *mut u8 = core::ptr::null_mut();
        let r1: bool = save_error((&mut err) as *mut *mut u8, &st1);

        assert!(r1);
        assert!(!err.is_null());

        let first_bytes: Vec<u8> = unsafe {
            let mut v: Vec<u8> = Vec::new();
            let mut i: usize = 0usize;
            while i < 4096usize {
                let b: u8 = *err.add(i);
                if b == 0u8 {
                    break;
                }
                v.push(b);
                i = i + 1;
            }
            v
        };

        let msg2: Slice = Slice::from_str("bad2");
        let st2: Status = crate::Status::not_supported(&msg2, None);

        let r2: bool = save_error((&mut err) as *mut *mut u8, &st2);

        assert!(r2);
        assert!(!err.is_null());

        let second_bytes: Vec<u8> = unsafe {
            let mut v: Vec<u8> = Vec::new();
            let mut i: usize = 0usize;
            while i < 4096usize {
                let b: u8 = *err.add(i);
                if b == 0u8 {
                    break;
                }
                v.push(b);
                i = i + 1;
            }
            v
        };

        assert!(first_bytes != second_bytes);

        crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);

        drop(msg1);
        drop(msg2);
    }

    #[test]
    #[should_panic]
    fn bitcoinleveldb_db__save_error_rs__null_errptr_panics_by_contract() {
        let msg: Slice = Slice::from_str("bad");
        let st: Status = crate::Status::invalid_argument(&msg, None);
        let _ = save_error(core::ptr::null_mut(), &st);
    }
}
