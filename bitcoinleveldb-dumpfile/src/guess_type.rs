// ---------------- [ File: bitcoinleveldb-dumpfile/src/guess_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dumpfile.cc]

pub fn guess_type(fname: &String, ty: *mut FileType) -> bool {
    trace!(file = %fname, ty_is_null = ty.is_null(), "guess_type: start");

    if ty.is_null() {
        error!(file = %fname, "guess_type: ty out-param is null");
        return false;
    }

    let basename: String = match fname.rfind('/') {
        Some(pos) => fname[(pos + 1)..].to_string(),
        None => fname.clone(),
    };

    let mut ignored: u64 = 0;
    let ok = parse_file_name(&basename, &mut ignored as *mut u64, ty);

    debug!(
        file = %fname,
        basename = %basename,
        ok,
        "guess_type: parse_file_name completed"
    );

    ok
}

#[cfg(test)]
mod guess_type_behavior_suite {
    use super::*;

    fn assert_file_type(expected: FileType, actual: FileType) {
        let ok = match (expected, actual) {
            (FileType::LogFile, FileType::LogFile) => true,
            (FileType::DBLockFile, FileType::DBLockFile) => true,
            (FileType::TableFile, FileType::TableFile) => true,
            (FileType::DescriptorFile, FileType::DescriptorFile) => true,
            (FileType::CurrentFile, FileType::CurrentFile) => true,
            (FileType::TempFile, FileType::TempFile) => true,
            (FileType::InfoLogFile, FileType::InfoLogFile) => true,
            _ => false,
        };
        assert!(ok, "file type mismatch");
    }

    #[traced_test]
    fn guess_type_returns_false_when_out_param_is_null() {
        trace!("guess_type_returns_false_when_out_param_is_null: start");

        let fname = "000001.log".to_string();
        let ok = guess_type(&fname, std::ptr::null_mut());

        assert!(!ok);

        trace!("guess_type_returns_false_when_out_param_is_null: end");
    }

    #[traced_test]
    fn guess_type_parses_log_file_from_basename() {
        trace!("guess_type_parses_log_file_from_basename: start");

        let fname = "000001.log".to_string();
        let mut ty = FileType::TempFile;

        let ok = guess_type(&fname, &mut ty as *mut FileType);

        assert!(ok);
        assert_file_type(FileType::LogFile, ty);

        trace!("guess_type_parses_log_file_from_basename: end");
    }

    #[traced_test]
    fn guess_type_parses_log_file_from_path_with_directories() {
        trace!("guess_type_parses_log_file_from_path_with_directories: start");

        let fname = "/tmp/some/dir/000001.log".to_string();
        let mut ty = FileType::TempFile;

        let ok = guess_type(&fname, &mut ty as *mut FileType);

        assert!(ok);
        assert_file_type(FileType::LogFile, ty);

        trace!("guess_type_parses_log_file_from_path_with_directories: end");
    }

    #[traced_test]
    fn guess_type_parses_descriptor_file_from_manifest_name() {
        trace!("guess_type_parses_descriptor_file_from_manifest_name: start");

        let fname = "MANIFEST-000123".to_string();
        let mut ty = FileType::TempFile;

        let ok = guess_type(&fname, &mut ty as *mut FileType);

        assert!(ok);
        assert_file_type(FileType::DescriptorFile, ty);

        trace!("guess_type_parses_descriptor_file_from_manifest_name: end");
    }

    #[traced_test]
    fn guess_type_parses_table_file_from_sst_name() {
        trace!("guess_type_parses_table_file_from_sst_name: start");

        let fname = "000777.sst".to_string();
        let mut ty = FileType::TempFile;

        let ok = guess_type(&fname, &mut ty as *mut FileType);

        assert!(ok);
        assert_file_type(FileType::TableFile, ty);

        trace!("guess_type_parses_table_file_from_sst_name: end");
    }

    #[traced_test]
    fn guess_type_returns_false_for_unrecognized_names() {
        trace!("guess_type_returns_false_for_unrecognized_names: start");

        let fname = "definitely-not-a-leveldb-filename".to_string();
        let mut ty = FileType::TempFile;

        let ok = guess_type(&fname, &mut ty as *mut FileType);

        assert!(!ok);

        trace!("guess_type_returns_false_for_unrecognized_names: end");
    }
}
