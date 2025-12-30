// ---------------- [ File: bitcoinleveldb-dumpfile/src/print_log_contents.rs ]
crate::ix!();

/// Print contents of a log file. (*func)() is called on every record.
///
pub fn print_log_contents(
    env: Rc<RefCell<dyn crate::Env>>,
    fname: &String,
    func: fn(_0: u64, _1: Slice, _2: *mut dyn WritableFile) -> c_void,
    dst: *mut dyn WritableFile,
) -> crate::Status {
    trace!(file = %fname, dst_is_null = dst.is_null(), "print_log_contents: start");

    if dst.is_null() {
        error!(file = %fname, "print_log_contents: dst is null");
        let msg = format!("{fname}: null destination");
        let msg_slice = Slice::from(&msg);
        return Status::invalid_argument(&msg_slice, None);
    }

    let mut file: *mut Box<dyn SequentialFile> = std::ptr::null_mut();
    let s = env.borrow_mut().new_sequential_file(
        fname,
        &mut file as *mut *mut Box<dyn SequentialFile>,
    );

    if !s.is_ok() {
        error!(
            file = %fname,
            status = %s.to_string(),
            "print_log_contents: NewSequentialFile failed"
        );
        return s;
    }

    if file.is_null() {
        error!(
            file = %fname,
            "print_log_contents: NewSequentialFile returned ok but file pointer is null"
        );
        let msg = format!("{fname}: null SequentialFile");
        let msg_slice = Slice::from(&msg);
        return Status::io_error(&msg_slice, None);
    }

    let mut reporter = CorruptionReporter::new(dst);

    let seq_file: *mut dyn SequentialFile = unsafe { &mut **file };
    let reporter_ptr: *mut dyn LogReaderReporter = &mut reporter;

    let mut reader = LogReader::new(seq_file, reporter_ptr, true, 0);

    let mut record: Slice = Slice::default();
    let mut scratch: String = String::new();

    while reader.read_record(&mut record, &mut scratch) {
        (func)(reader.last_record_offset(), record, dst);
        record = Slice::default();
    }

    unsafe {
        drop(Box::from_raw(file));
    }

    info!(file = %fname, "print_log_contents: complete");
    Status::ok()
}

#[cfg(test)]
mod print_log_contents_behavior_suite {
    use super::*;

    fn unique_temp_dir_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let mut dir = std::env::temp_dir();
        dir.push(format!("bitcoinleveldb_print_log_contents_{tag}_{nanos}"));
        dir
    }

    fn mask_crc32c(crc: u32) -> u32 {
        ((crc >> 15) | (crc << 17)).wrapping_add(0xa282ead8)
    }

    fn crc32c_castagnoli(data: &[u8]) -> u32 {
        let mut crc: u32 = 0xffffffff;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0x82f63b78u32 & mask);
            }
        }
        !crc
    }

    fn build_full_record_fragment(data: &[u8]) -> Vec<u8> {
        let record_type: u8 = 1; // kFullType in LevelDB log format.

        let mut crc_bytes: Vec<u8> = Vec::with_capacity(1 + data.len());
        crc_bytes.push(record_type);
        crc_bytes.extend_from_slice(data);

        let crc = crc32c_castagnoli(&crc_bytes);
        let masked = mask_crc32c(crc);

        let mut out: Vec<u8> = Vec::with_capacity(7 + data.len());
        out.extend_from_slice(&masked.to_le_bytes());
        out.extend_from_slice(&(data.len() as u16).to_le_bytes());
        out.push(record_type);
        out.extend_from_slice(data);
        out
    }

    fn build_corrupt_full_record_fragment_with_bad_checksum(data: &[u8]) -> Vec<u8> {
        let record_type: u8 = 1; // kFullType.
        let bad_checksum: u32 = 0; // almost certainly not correct.

        let mut out: Vec<u8> = Vec::with_capacity(7 + data.len());
        out.extend_from_slice(&bad_checksum.to_le_bytes());
        out.extend_from_slice(&(data.len() as u16).to_le_bytes());
        out.push(record_type);
        out.extend_from_slice(data);
        out
    }

    fn record_echo_printer(pos: u64, record: Slice, dst: *mut dyn WritableFile) -> c_void {
        trace!(
            pos,
            record_len = slice_as_bytes(&record).len(),
            dst_is_null = dst.is_null(),
            "record_echo_printer invoked"
        );

        if dst.is_null() {
            return ();
        }

        let escaped = escape_for_debug(slice_as_bytes(&record));
        let line = format!("pos={}; record='{}'\n", pos, escaped);
        let slice = Slice::from(&line);

        let s = unsafe { (&mut *dst).append(&slice) };
        debug!(status = %s.to_string(), "record_echo_printer append status");

        ()
    }

    #[traced_test]
    fn print_log_contents_rejects_null_destination_pointer() {
        trace!("print_log_contents_rejects_null_destination_pointer: start");

        let env = posix_default_env();
        let fname = "000001.log".to_string();

        let s = print_log_contents(env, &fname, record_echo_printer, std::ptr::null_mut());

        assert!(s.is_invalid_argument());

        trace!("print_log_contents_rejects_null_destination_pointer: end");
    }

    #[traced_test]
    fn print_log_contents_propagates_error_when_file_missing() {
        trace!("print_log_contents_propagates_error_when_file_missing: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("missing");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = print_log_contents(env, &fname, record_echo_printer, &mut dst);

        debug!(status = %s.to_string(), "print_log_contents returned");
        assert!(!s.is_ok());

        let _ = std::fs::remove_dir_all(&dir);

        trace!("print_log_contents_propagates_error_when_file_missing: end");
    }

    #[traced_test]
    fn print_log_contents_returns_ok_for_empty_file_and_emits_nothing() {
        trace!("print_log_contents_returns_ok_for_empty_file_and_emits_nothing: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("empty");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        std::fs::write(&file_path, &[]).expect("write empty file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = print_log_contents(env, &fname, record_echo_printer, &mut dst);

        debug!(status = %s.to_string(), "print_log_contents returned");
        assert!(s.is_ok());
        assert_eq!(dst.contents_string(), "");

        let _ = std::fs::remove_dir_all(&dir);

        trace!("print_log_contents_returns_ok_for_empty_file_and_emits_nothing: end");
    }

    #[traced_test]
    fn print_log_contents_invokes_callback_for_valid_full_record() {
        trace!("print_log_contents_invokes_callback_for_valid_full_record: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("valid_record");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");

        let record_data = b"abc";
        let bytes = build_full_record_fragment(record_data);

        std::fs::write(&file_path, bytes).expect("write log record bytes");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = print_log_contents(env, &fname, record_echo_printer, &mut dst);

        debug!(status = %s.to_string(), out = %dst.contents_string(), "print_log_contents returned");
        assert!(s.is_ok());

        let out = dst.contents_string();
        assert!(
            out.contains("pos=0;"),
            "expected callback to see first record at offset 0"
        );
        assert!(
            out.contains("record='abc'"),
            "expected callback to see record payload"
        );
        assert!(
            !out.contains("corruption:"),
            "did not expect corruption output for valid record"
        );

        let _ = std::fs::remove_dir_all(&dir);

        trace!("print_log_contents_invokes_callback_for_valid_full_record: end");
    }

    #[traced_test]
    fn print_log_contents_reports_corruption_when_checksum_is_bad() {
        trace!("print_log_contents_reports_corruption_when_checksum_is_bad: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("bad_checksum");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");

        let record_data = b"x";
        let bytes = build_corrupt_full_record_fragment_with_bad_checksum(record_data);

        std::fs::write(&file_path, bytes).expect("write corrupted log record bytes");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let mut dst = CapturingWritableFile::new_named("dst");

        let s = print_log_contents(env, &fname, record_echo_printer, &mut dst);

        debug!(status = %s.to_string(), out = %dst.contents_string(), "print_log_contents returned");
        assert!(s.is_ok());

        let out = dst.contents_string();
        assert!(
            out.contains("corruption:"),
            "expected corruption reporter output for bad checksum"
        );
        assert!(
            !out.contains("record='x'"),
            "expected callback to not run for a corrupted record"
        );

        let _ = std::fs::remove_dir_all(&dir);

        trace!("print_log_contents_reports_corruption_when_checksum_is_bad: end");
    }
}
