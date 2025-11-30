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
