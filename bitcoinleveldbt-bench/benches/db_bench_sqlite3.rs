
pub fn bitcoinleveldb_bench_sqlite3_custom_harness_process_argv_or_exit(
) -> (Vec<std::ffi::CString>, Vec<*mut u8>, i32) {
    let mut owned_arguments: Vec<std::ffi::CString> = Vec::new();

    for argument_os in std::env::args_os() {
        let argument_text = argument_os.to_string_lossy().into_owned();

        if argument_text == "--bench" {
            continue;
        }

        match std::ffi::CString::new(argument_text) {
            Ok(value) => {
                owned_arguments.push(value);
            }
            Err(_) => {
                eprintln!("benchmark argument contains interior NUL");
                std::process::exit(1);
            }
        }
    }

    if owned_arguments.len() > i32::MAX as usize {
        eprintln!("too many benchmark arguments");
        std::process::exit(1);
    }

    let argc = owned_arguments.len() as i32;
    let mut raw_arguments: Vec<*mut u8> = Vec::with_capacity(owned_arguments.len());

    let mut index = 0usize;
    while index < owned_arguments.len() {
        raw_arguments.push(owned_arguments[index].as_ptr() as *mut u8);
        index += 1;
    }

    (owned_arguments, raw_arguments, argc)
}

fn main() {
    let (_owned_arguments, mut raw_arguments, argc) =
        bitcoinleveldb_bench_sqlite3_custom_harness_process_argv_or_exit();

    let exit_code = bitcoinleveldb_bench::benchdb_bench_sqlite3_main(
        argc,
        raw_arguments.as_mut_ptr(),
    );

    std::process::exit(exit_code);
}
