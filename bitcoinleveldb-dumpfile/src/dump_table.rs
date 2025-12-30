// ---------------- [ File: bitcoinleveldb-dumpfile/src/dump_table.rs ]
crate::ix!();

pub fn dump_table(
    env:   Rc<RefCell<dyn crate::Env>>,
    fname: &String,
    dst:   *mut dyn WritableFile,
) -> crate::Status {

    trace!(file = %fname, dst_is_null = dst.is_null(), "dump_table: start");

    if dst.is_null() {
        error!(file = %fname, "dump_table: dst is null");
        let msg = format!("{fname}: null destination");
        let msg_slice = Slice::from(&msg);
        return Status::invalid_argument(&msg_slice, None);
    }

    let mut file_size: u64 = 0;
    let mut file: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();
    let mut table: *mut Table = std::ptr::null_mut();

    let mut s = env
        .borrow_mut()
        .get_file_size(fname, &mut file_size as *mut u64);

    if s.is_ok() {
        debug!(file = %fname, file_size, "dump_table: got file size");
        s = env
            .borrow_mut()
            .new_random_access_file(fname, &mut file as *mut *mut Box<dyn RandomAccessFile>);
    }

    if s.is_ok() {
        debug!(file = %fname, "dump_table: opened random access file");

        let options = Options::default();

        if file.is_null() {
            error!(
                file = %fname,
                "dump_table: NewRandomAccessFile returned ok but file pointer is null"
            );
            let msg = format!("{fname}: null RandomAccessFile");
            let msg_slice = Slice::from(&msg);
            s = Status::io_error(&msg_slice, None);
        } else {
            // We use the default comparator, which may or may not match the
            // comparator used in this database. However this should not cause
            // problems since we only use Table operations that do not require
            // any comparisons.  In particular, we do not call Seek or Prev.
            let raf: *mut dyn RandomAccessFile = unsafe { &mut **file };
            s = Table::open(options, raf, file_size, &mut table as *mut *mut Table);
        }
    }

    if !s.is_ok() {
        error!(file = %fname, status = %s.to_string(), "dump_table: open failed");

        unsafe {
            if !table.is_null() {
                drop(Box::from_raw(table));
            }
            if !file.is_null() {
                drop(Box::from_raw(file));
            }
        }

        return s;
    }

    let mut ro = ReadOptions::default();
    ro.set_fill_cache(false);

    let iter = unsafe { (*table).new_iterator(ro) };

    if iter.is_null() {
        error!(file = %fname, "dump_table: table returned null iterator");

        unsafe {
            drop(Box::from_raw(table));
            if !file.is_null() {
                drop(Box::from_raw(file));
            }
        }

        let msg = format!("{fname}: null iterator");
        let msg_slice = Slice::from(&msg);
        return Status::io_error(&msg_slice, None);
    }

    let mut r = String::new();

    unsafe {
        (*iter).seek_to_first();

        while (*iter).valid() {
            r.clear();

            let ikey: Slice = (*iter).key();
            let ival: Slice = (*iter).value();

            let mut parsed = ParsedInternalKey::default();
            let ok = parse_internal_key(&ikey, &mut parsed as *mut ParsedInternalKey);

            if !ok {
                r.push_str("badkey '");
                r.push_str(&escape_for_debug(slice_as_bytes(&ikey)));
                r.push_str("' => '");
                r.push_str(&escape_for_debug(slice_as_bytes(&ival)));
                r.push_str("'\n");
            } else {
                let user_key = parsed.user_key();
                let seq = *parsed.sequence();
                let ty = *parsed.ty();

                r.push('\'');
                r.push_str(&escape_for_debug(slice_as_bytes(user_key)));
                r.push_str("' @ ");
                r.push_str(&seq.to_string());
                r.push_str(" : ");

                match ty {
                    ValueType::TypeDeletion => r.push_str("del"),
                    ValueType::TypeValue => r.push_str("val"),
                }

                r.push_str(" => '");
                r.push_str(&escape_for_debug(slice_as_bytes(&ival)));
                r.push_str("'\n");
            }

            let line = Slice::from(&r);
            let append_status = (&mut *dst).append(&line);

            if !append_status.is_ok() {
                error!(
                    file = %fname,
                    append_status = %append_status.to_string(),
                    "dump_table: dst append failed"
                );
            }

            (*iter).next();
        }

        let iter_status = (*iter).status();
        if !iter_status.is_ok() {
            let msg = format!("iterator error: {}\n", iter_status.to_string());
            let msg_slice = Slice::from(&msg);
            let append_status = (&mut *dst).append(&msg_slice);

            if !append_status.is_ok() {
                error!(
                    file = %fname,
                    append_status = %append_status.to_string(),
                    "dump_table: failed to append iterator error"
                );
            }
        }

        drop(Box::from_raw(iter));
        drop(Box::from_raw(table));
        if !file.is_null() {
            drop(Box::from_raw(file));
        }
    }

    info!(file = %fname, "dump_table: complete");
    Status::ok()
}

#[cfg(test)]
mod dump_table_behavior_suite {
    use super::*;

    fn unique_temp_dir_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let mut dir = std::env::temp_dir();
        dir.push(format!("bitcoinleveldb_dump_table_{tag}_{nanos}"));
        dir
    }

    #[traced_test]
    fn dump_table_rejects_null_destination_pointer() {
        trace!("dump_table_rejects_null_destination_pointer: start");

        let env = posix_default_env();
        let fname = "000001.sst".to_string();

        let s = dump_table(env, &fname, std::ptr::null_mut());

        assert!(s.is_invalid_argument());
        trace!(status = %s.to_string(), "dump_table returned");

        trace!("dump_table_rejects_null_destination_pointer: end");
    }

    #[traced_test]
    fn dump_table_returns_error_for_missing_file() {
        trace!("dump_table_returns_error_for_missing_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing_sst_dir");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.sst");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_table(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_table returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_table_returns_error_for_missing_file: end");
    }

    #[traced_test]
    fn dump_table_returns_error_for_empty_file_not_a_valid_table() {
        trace!("dump_table_returns_error_for_empty_file_not_a_valid_table: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("empty_sst");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.sst");
        std::fs::write(&file_path, &[0u8; 0]).expect("write empty file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_table(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_table returned");
        assert!(!s.is_ok(), "empty table files should not open successfully");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_table_returns_error_for_empty_file_not_a_valid_table: end");
    }

    #[traced_test]
    fn dump_table_returns_error_for_garbage_file_not_a_valid_table() {
        trace!("dump_table_returns_error_for_garbage_file_not_a_valid_table: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("garbage_sst");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.sst");
        std::fs::write(&file_path, b"this-is-not-a-leveldb-table").expect("write garbage file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = dump_table(env, &fname, &mut dst);

        debug!(status = %s.to_string(), "dump_table returned");
        assert!(!s.is_ok(), "garbage table files should not open successfully");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_table_returns_error_for_garbage_file_not_a_valid_table: end");
    }

    #[traced_test]
    fn dump_table_does_not_write_any_output_when_open_fails() {
        trace!("dump_table_does_not_write_any_output_when_open_fails: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("open_fail_no_output");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.sst");
        std::fs::write(&file_path, b"not-a-table").expect("write invalid file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");
        let s = dump_table(env, &fname, &mut dst);

        debug!(status = %s.to_string(), output_len = dst.contents_string().len(), "dump_table returned");
        assert!(!s.is_ok());
        assert_eq!(dst.contents_string(), "", "dump_table should not emit partial output if open fails");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("dump_table_does_not_write_any_output_when_open_fails: end");
    }
}
