// ---------------- [ File: bitcoinleveldb-file/src/file_names.rs ]
/*!
  | File names used by DB code
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/filename.h]

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/filename.cc]

pub fn make_file_name(dbname: &String, number: u64, suffix: *const u8) -> String {
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use tracing::{debug, trace};

    // SAFETY: All call sites in this module pass a static NUL-terminated byte string.
    let suffix_str: &str = unsafe {
        if suffix.is_null() {
            ""
        } else {
            // If suffix contains invalid UTF-8 (shouldn't happen for our constants),
            // fall back to empty suffix to preserve control-flow (still returns a name).
            CStr::from_ptr(suffix as *const c_char)
                .to_str()
                .unwrap_or("")
        }
    };

    trace!(dbname = %dbname, number, suffix = %suffix_str, "constructing file name");
    let buf = format!("/{number:06}.{suffix}", number = number, suffix = suffix_str);
    let out = format!("{db}{buf}", db = dbname, buf = buf);
    debug!(result = %out, "constructed file name");
    out
}

/**
  | Return the name of the sstable with the
  | specified number in the db named by "dbname".
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn table_file_name(dbname: &String, number: u64) -> String {
    use tracing::{debug, trace};

    assert!(number > 0);
    trace!(dbname = %dbname, number, "table_file_name");
    let out = make_file_name(dbname, number, b"ldb\0".as_ptr());
    debug!(result = %out, "table_file_name constructed");
    out
}

/**
  | Return the legacy file name for an sstable with
  | the specified number in the db named by
  | "dbname". The result will be prefixed with
  | "dbname".
  */
pub fn sst_table_file_name(dbname: &String, number: u64) -> String {
    use tracing::{debug, trace};

    assert!(number > 0);
    trace!(dbname = %dbname, number, "sst_table_file_name");
    let out = make_file_name(dbname, number, b"sst\0".as_ptr());
    debug!(result = %out, "sst_table_file_name constructed");
    out
}

/**
  | Return the name of the descriptor file for the
  | db named by "dbname" and the specified
  | incarnation number.  The result will be
  | prefixed with "dbname".
  */
pub fn descriptor_file_name(dbname: &String, number: u64) -> String {
    use tracing::{debug, trace};

    assert!(number > 0);
    trace!(dbname = %dbname, number, "descriptor_file_name");
    let buf = format!("/MANIFEST-{num:06}", num = number);
    let out = format!("{db}{buf}", db = dbname, buf = buf);
    debug!(result = %out, "descriptor_file_name constructed");
    out
}

/**
  | Return the name of the current file.
  | This file contains the name of the current
  | manifest file.
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn current_file_name(dbname: &String) -> String {
    use tracing::{debug, trace};

    trace!(dbname = %dbname, "current_file_name");
    let out = format!("{}/CURRENT", dbname);
    debug!(result = %out, "current_file_name constructed");
    out
}

/**
  | Return the name of a temporary file owned by
  | the db named "dbname".
  |
  | The result will be prefixed with "dbname".
  */
pub fn temp_file_name(dbname: &String, number: u64) -> String {
    use tracing::{debug, trace};

    assert!(number > 0);
    trace!(dbname = %dbname, number, "temp_file_name");
    let out = make_file_name(dbname, number, b"dbtmp\0".as_ptr());
    debug!(result = %out, "temp_file_name constructed");
    out
}

/**
  | Return the name of the info log file for
  | "dbname".
  |
  */
pub fn info_log_file_name(dbname: &String) -> String {
    use tracing::{debug, trace};

    trace!(dbname = %dbname, "info_log_file_name");
    let out = format!("{}/LOG", dbname);
    debug!(result = %out, "info_log_file_name constructed");
    out
}

/**
  | Return the name of the old info log file
  | for "dbname".
  |
  */
pub fn old_info_log_file_name(dbname: &String) -> String {
    use tracing::{debug, trace};

    trace!(dbname = %dbname, "old_info_log_file_name");
    let out = format!("{}/LOG.old", dbname);
    debug!(result = %out, "old_info_log_file_name constructed");
    out
}

/**
  | Return the name of the log file with the
  | specified number in the db named by "dbname".
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn log_file_name(dbname: &String, number: u64) -> String {
    use tracing::{debug, trace};

    assert!(number > 0);
    trace!(dbname = %dbname, number, "log_file_name");
    let out = make_file_name(dbname, number, b"log\0".as_ptr());
    debug!(result = %out, "log_file_name constructed");
    out
}

/**
  | Return the name of the lock file for the
  | db named by "dbname". The result will
  | be prefixed with "dbname".
  |
  */
pub fn lock_file_name(dbname: &String) -> String {
    use tracing::{debug, trace};

    trace!(dbname = %dbname, "lock_file_name");
    let out = format!("{}/LOCK", dbname);
    debug!(result = %out, "lock_file_name constructed");
    out
}
