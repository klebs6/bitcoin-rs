// ---------------- [ File: bitcoinleveldb-dumpfile/src/handle_dump_command.rs ]
crate::ix!();

pub fn handle_dump_command(
    env:   Rc<RefCell<dyn Env>>,
    files: *mut *mut u8,
    num:   i32,
) -> bool {

    trace!(num, "handle_dump_command: start");

    let mut printer = StdoutPrinter {};
    let mut ok      = true;

    for i in 0..(num as usize) {

        let fname_ptr = unsafe { *files.add(i) };
        let cstr      = unsafe { CStr::from_ptr(fname_ptr as *const c_char) };
        let fname: String = cstr.to_string_lossy().into_owned();

        debug!(index = i, file = %fname, "handle_dump_command: dumping file");

        let status = dump_file(env.clone(), &fname, &mut printer);

        if !status.is_ok() {
            let status_str = status.to_string();
            error!(
                index = i,
                file  = %fname,
                status_str = %status_str,
                "handle_dump_command: DumpFile returned error"
            );
            eprintln!("{}", status_str);
            ok = false;
        }
    }

    info!(ok, "handle_dump_command: completed");
    ok
}

#[cfg(test)]
mod handle_dump_command_behavior_suite {
    use super::*;

    fn unique_temp_dir_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let mut dir = std::env::temp_dir();
        dir.push(format!("bitcoinleveldb_handle_dump_command_{tag}_{nanos}"));
        dir
    }

    fn build_file_ptrs(paths: &[String]) -> (Vec<std::ffi::CString>, Vec<*mut u8>) {
        let mut cstrings: Vec<std::ffi::CString> = paths
            .iter()
            .map(|p| std::ffi::CString::new(p.as_str()).expect("CString::new"))
            .collect();

        let mut ptrs: Vec<*mut u8> = cstrings
            .iter_mut()
            .map(|cs| cs.as_ptr() as *mut u8)
            .collect();

        (cstrings, ptrs)
    }

    #[traced_test]
    fn handle_dump_command_returns_true_when_no_files_provided() {
        trace!("handle_dump_command_returns_true_when_no_files_provided: start");

        let env = posix_default_env();

        let ok = handle_dump_command(env, std::ptr::null_mut(), 0);

        assert!(ok);

        trace!("handle_dump_command_returns_true_when_no_files_provided: end");
    }

    #[traced_test]
    fn handle_dump_command_returns_false_for_unknown_file_type() {
        trace!("handle_dump_command_returns_false_for_unknown_file_type: start");

        let env = posix_default_env();

        let paths = vec!["not-a-leveldb-file.type".to_string()];
        let (_cs, mut ptrs) = build_file_ptrs(&paths);

        let ok = handle_dump_command(env, ptrs.as_mut_ptr(), ptrs.len() as i32);

        assert!(!ok);

        trace!("handle_dump_command_returns_false_for_unknown_file_type: end");
    }

    #[traced_test]
    fn handle_dump_command_returns_true_for_empty_log_file() {
        trace!("handle_dump_command_returns_true_for_empty_log_file: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("empty_log");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let file_path = dir.join("000001.log");
        std::fs::write(&file_path, &[]).expect("write empty log file");

        let fname = file_path.to_str().expect("utf-8 path").to_string();

        let paths = vec![fname];
        let (_cs, mut ptrs) = build_file_ptrs(&paths);

        let ok = handle_dump_command(env, ptrs.as_mut_ptr(), ptrs.len() as i32);

        assert!(ok);

        let _ = std::fs::remove_dir_all(&dir);

        trace!("handle_dump_command_returns_true_for_empty_log_file: end");
    }

    #[traced_test]
    fn handle_dump_command_returns_false_when_any_one_file_fails_among_many() {
        trace!("handle_dump_command_returns_false_when_any_one_file_fails_among_many: start");

        let env = posix_default_env();

        let dir = unique_temp_dir_path("mixed");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let good_log = dir.join("000010.log");
        std::fs::write(&good_log, &[]).expect("write empty log file");

        let good_name = good_log.to_str().expect("utf-8 path").to_string();
        let bad_name = "not-a-leveldb-file.type".to_string();

        let paths = vec![good_name, bad_name];
        let (_cs, mut ptrs) = build_file_ptrs(&paths);

        let ok = handle_dump_command(env, ptrs.as_mut_ptr(), ptrs.len() as i32);

        assert!(!ok);

        let _ = std::fs::remove_dir_all(&dir);

        trace!("handle_dump_command_returns_false_when_any_one_file_fails_among_many: end");
    }
}
