// ---------------- [ File: bitcoinleveldb-dumpfile/src/dump_log.rs ]
crate::ix!();

pub fn dump_log(
    env: Rc<RefCell<dyn crate::Env>>,
    fname: &String,
    dst: *mut dyn WritableFile,
) -> crate::Status {
    trace!(file = %fname, dst_is_null = dst.is_null(), "dump_log: start");

    if dst.is_null() {
        error!(file = %fname, "dump_log: dst is null");
        let msg = format!("{fname}: null destination");
        let msg_slice = Slice::from(&msg);
        return Status::invalid_argument(&msg_slice, None);
    }

    let s = print_log_contents(env, fname, write_batch_printer, dst);

    if !s.is_ok() {
        error!(
            file = %fname,
            status = %s.to_string(),
            "dump_log: failed"
        );
    } else {
        info!(file = %fname, "dump_log: complete");
    }

    s
}

#[cfg(test)]
mod dump_log_behavior_suite {
    use super::*;

    fn unique_temp_dir_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let mut dir = std::env::temp_dir();
        dir.push(format!("bitcoinleveldb_dump_log_{tag}_{nanos}"));
        dir
    }

    #[traced_test]
    fn dump_log_rejects_null_destination_pointer() {
        trace!("dump_log_rejects_null_destination_pointer: start");

        let env = posix_default_env();
        let fname = "000001.log".to_string();

        let s = dump_log(env, &fname, std::ptr::null_mut());

        assert!(s.is_invalid_argument());
        trace!(status = %s.to_string(), "dump_log returned");

        trace!("dump_log_rejects_null_destination_pointer: end");
    }

    #[traced_test]
    fn dump_log_returns_ok_for_empty_existing_file_and_writes_nothing() {
        trace!("dump_log_returns_ok_for_empty_existing_file_and_writes_nothing: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("empty_log");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        std::fs::write(&file_path, &[]).expect("write empty file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_log(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_log returned");
        assert!(s.is_ok());
        assert_eq!(dst.contents_string(), "");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_log_returns_ok_for_empty_existing_file_and_writes_nothing: end");
    }

    #[traced_test]
    fn dump_log_propagates_error_when_file_missing() {
        trace!("dump_log_propagates_error_when_file_missing: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing_log_dir");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_log(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_log returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_log_propagates_error_when_file_missing: end");
    }
}
