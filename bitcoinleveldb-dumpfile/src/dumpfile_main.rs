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
