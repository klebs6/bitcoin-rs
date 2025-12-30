// ---------------- [ File: bitcoinleveldb-dumpfile/src/dumpfile.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/dumpfile.h]

/// Dump the contents of the file named by fname in text format to *dst.  Makes a sequence of
/// dst->Append() calls; each call is passed the newline-terminated text corresponding to a single
/// item found in the file.
/// 
/// Returns a non-OK result if fname does not name a leveldb storage file, or if the file cannot be
/// read.
///
pub fn dump_file(
    env: Rc<RefCell<dyn Env>>,
    fname: &String,
    dst: *mut dyn WritableFile,
) -> crate::Status {
    trace!(file = %fname, dst_is_null = dst.is_null(), "dump_file: start");

    if dst.is_null() {
        error!(file = %fname, "dump_file: dst is null");
        let msg = format!("{fname}: null destination");
        let msg_slice = Slice::from(&msg);
        return Status::invalid_argument(&msg_slice, None);
    }

    let mut ftype: FileType = FileType::TempFile;

    if !guess_type(fname, &mut ftype as *mut FileType) {
        let msg = format!("{fname}: unknown file type");
        let msg_slice = Slice::from(&msg);
        warn!(file = %fname, "dump_file: unknown file type");
        return Status::invalid_argument(&msg_slice, None);
    }

    let s = match ftype {
        FileType::LogFile => dump_log(env, fname, dst),
        FileType::DescriptorFile => dump_descriptor(env, fname, dst),
        FileType::TableFile => dump_table(env, fname, dst),
        _ => {
            let msg = format!("{fname}: not a dump-able file type");
            let msg_slice = Slice::from(&msg);
            Status::invalid_argument(&msg_slice, None)
        }
    };

    if !s.is_ok() {
        error!(
            file = %fname,
            status = %s.to_string(),
            "dump_file: failed"
        );
    } else {
        info!(file = %fname, "dump_file: complete");
    }

    s
}

#[cfg(test)]
mod dump_file_behavior_suite {
    use super::*;

    fn unique_temp_dir_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let mut dir = std::env::temp_dir();
        dir.push(format!("bitcoinleveldb_dump_file_{tag}_{nanos}"));
        dir
    }

    #[traced_test]
    fn dump_file_rejects_null_destination_pointer() {
        trace!("dump_file_rejects_null_destination_pointer: start");

        let env = posix_default_env();
        let fname = "000001.log".to_string();

        let s = dump_file(env, &fname, std::ptr::null_mut());

        assert!(s.is_invalid_argument());
        trace!(status = %s.to_string(), "dump_file returned");

        trace!("dump_file_rejects_null_destination_pointer: end");
    }

    #[traced_test]
    fn dump_file_rejects_unknown_file_type() {
        trace!("dump_file_rejects_unknown_file_type: start");

        let env = posix_default_env();
        let fname = "not-a-leveldb-file.type".to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        assert!(s.is_invalid_argument());
        trace!(status = %s.to_string(), "dump_file returned");
        assert_eq!(dst.contents_string(), "");

        trace!("dump_file_rejects_unknown_file_type: end");
    }

    #[traced_test]
    fn dump_file_rejects_non_dumpable_current_file_type() {
        trace!("dump_file_rejects_non_dumpable_current_file_type: start");

        let env = posix_default_env();
        let fname = "CURRENT".to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        assert!(s.is_invalid_argument());
        trace!(status = %s.to_string(), "dump_file returned");

        trace!("dump_file_rejects_non_dumpable_current_file_type: end");
    }

    #[traced_test]
    fn dump_file_dispatches_log_file_and_propagates_open_error_for_missing_file() {
        trace!("dump_file_dispatches_log_file_and_propagates_open_error_for_missing_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing_log");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_file returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_file_dispatches_log_file_and_propagates_open_error_for_missing_file: end");
    }

    #[traced_test]
    fn dump_file_dispatches_descriptor_file_and_propagates_open_error_for_missing_file() {
        trace!("dump_file_dispatches_descriptor_file_and_propagates_open_error_for_missing_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing_manifest");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("MANIFEST-000001");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_file returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_file_dispatches_descriptor_file_and_propagates_open_error_for_missing_file: end");
    }

    #[traced_test]
    fn dump_file_dispatches_table_file_and_propagates_error_for_missing_file() {
        trace!("dump_file_dispatches_table_file_and_propagates_error_for_missing_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing_sst");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.sst");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_file returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_file_dispatches_table_file_and_propagates_error_for_missing_file: end");
    }

    #[traced_test]
    fn dump_file_succeeds_for_empty_log_file() {
        trace!("dump_file_succeeds_for_empty_log_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("empty_log");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000002.log");
        std::fs::write(&file_path, &[]).expect("write empty log");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_file(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_file returned");
        assert!(s.is_ok());
        assert_eq!(dst.contents_string(), "");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_file_succeeds_for_empty_log_file: end");
    }
}
