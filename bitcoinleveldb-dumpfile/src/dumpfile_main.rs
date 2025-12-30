// ---------------- [ File: bitcoinleveldb-dumpfile/src/dumpfile_main.rs ]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/leveldbutil.cc]
crate::ix!();

/// Print usage instructions for `leveldbutil`.
pub fn usage() {
    info!("Printing usage message for leveldbutil");
    eprintln!(
        "Usage: leveldbutil command...\n   dump files...         -- dump contents of specified files"
    );
}

pub fn dbleveldbutil_main(argc: i32, argv: *mut *mut u8) -> i32 {
    trace!(argc, "dbleveldbutil_main: entry");

    let env: Rc<RefCell<dyn Env>> = posix_default_env();

    let mut ok = true;

    if argc < 2 {
        trace!(
            argc,
            "dbleveldbutil_main: insufficient arguments, calling usage()"
        );
        usage();
        ok = false;
    } else {
        let cmd_ptr = unsafe { *argv.add(1) };

        if cmd_ptr.is_null() {
            warn!(
                "dbleveldbutil_main: argv[1] is null, treating as invalid command"
            );
            usage();
            ok = false;
        } else {
            let cstr    = unsafe { CStr::from_ptr(cmd_ptr as *const c_char) };
            let command = cstr.to_string_lossy().into_owned();

            debug!(command = %command, "dbleveldbutil_main: parsed command");

            if command == "dump" {
                let files_ptr = unsafe { argv.add(2) };
                let num_files = argc - 2;

                trace!(
                    num_files,
                    "dbleveldbutil_main: invoking handle_dump_command for 'dump'"
                );

                ok = handle_dump_command(env, files_ptr, num_files);
            } else {
                trace!(
                    command = %command,
                    "dbleveldbutil_main: unknown command, calling usage()"
                );
                usage();
                ok = false;
            }
        }
    }

    let rc = if ok { 0 } else { 1 };
    info!(return_code = rc, "dbleveldbutil_main: exiting");
    rc
}

#[cfg(test)]
mod dumpfile_main_cli_behavior_suite {
    use super::*;

    fn build_argv(args: &[&str]) -> (Vec<std::ffi::CString>, Vec<*mut u8>) {
        let mut cstrings: Vec<std::ffi::CString> = args
            .iter()
            .map(|s| std::ffi::CString::new(*s).expect("CString::new"))
            .collect();

        let mut argv: Vec<*mut u8> = cstrings
            .iter_mut()
            .map(|cs| cs.as_ptr() as *mut u8)
            .collect();

        (cstrings, argv)
    }

    #[traced_test]
    fn dbleveldbutil_main_returns_error_when_no_command_provided() {
        trace!("dbleveldbutil_main_returns_error_when_no_command_provided: start");

        let (_cs, mut argv) = build_argv(&["leveldbutil"]);
        let rc = dbleveldbutil_main(argv.len() as i32, argv.as_mut_ptr());

        assert_eq!(rc, 1);

        trace!("dbleveldbutil_main_returns_error_when_no_command_provided: end");
    }

    #[traced_test]
    fn dbleveldbutil_main_returns_error_for_unknown_command() {
        trace!("dbleveldbutil_main_returns_error_for_unknown_command: start");

        let (_cs, mut argv) = build_argv(&["leveldbutil", "unknown"]);
        let rc = dbleveldbutil_main(argv.len() as i32, argv.as_mut_ptr());

        assert_eq!(rc, 1);

        trace!("dbleveldbutil_main_returns_error_for_unknown_command: end");
    }

    #[traced_test]
    fn dbleveldbutil_main_returns_error_when_command_pointer_is_null() {
        trace!("dbleveldbutil_main_returns_error_when_command_pointer_is_null: start");

        let (_cs, mut argv) = build_argv(&["leveldbutil", "dump"]);
        argv[1] = std::ptr::null_mut();

        let rc = dbleveldbutil_main(argv.len() as i32, argv.as_mut_ptr());

        assert_eq!(rc, 1);

        trace!("dbleveldbutil_main_returns_error_when_command_pointer_is_null: end");
    }

    #[traced_test]
    fn dbleveldbutil_main_dump_with_no_files_returns_success() {
        trace!("dbleveldbutil_main_dump_with_no_files_returns_success: start");

        let (_cs, mut argv) = build_argv(&["leveldbutil", "dump"]);
        let rc = dbleveldbutil_main(argv.len() as i32, argv.as_mut_ptr());

        assert_eq!(rc, 0);

        trace!("dbleveldbutil_main_dump_with_no_files_returns_success: end");
    }

    #[traced_test]
    fn dbleveldbutil_main_dump_returns_error_when_any_file_fails() {
        trace!("dbleveldbutil_main_dump_returns_error_when_any_file_fails: start");

        let (_cs, mut argv) = build_argv(&["leveldbutil", "dump", "not-a-leveldb-file.type"]);
        let rc = dbleveldbutil_main(argv.len() as i32, argv.as_mut_ptr());

        assert_eq!(rc, 1);

        trace!("dbleveldbutil_main_dump_returns_error_when_any_file_fails: end");
    }
}
