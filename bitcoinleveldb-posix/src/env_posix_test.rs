// ---------------- [ File: bitcoinleveldb-posix/src/env_posix_test.rs ]
crate::ix!();

#[derive(Debug, Default)]
pub struct EnvPosixTest;

impl EnvPosixTest {
    pub fn set_file_limits(read_only_file_limit: i32, mmap_limit: i32) {
        info!(
            "EnvPosixTest::set_file_limits: read_only_file_limit={}, mmap_limit={}",
            read_only_file_limit,
            mmap_limit
        );

        EnvPosixTestHelper::set_read_only_fd_limit(read_only_file_limit);
        EnvPosixTestHelper::set_read_only_mmap_limit(mmap_limit);
    }
}

pub fn make_posix_env_for_tests() -> Rc<RefCell<dyn Env>> {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Once;

    static INIT_LIMITS: Once = Once::new();

    trace!("make_posix_env_for_tests: start");

    INIT_LIMITS.call_once(|| {
        use crate::{TEST_MMAP_LIMIT, TEST_READ_ONLY_FILE_LIMIT};

        info!(
            "make_posix_env_for_tests: configuring POSIX Env file limits \
             (read_only_file_limit={}, mmap_limit={})",
            TEST_READ_ONLY_FILE_LIMIT,
            TEST_MMAP_LIMIT
        );

        EnvPosixTest::set_file_limits(TEST_READ_ONLY_FILE_LIMIT, TEST_MMAP_LIMIT);
    });

    info!("make_posix_env_for_tests: constructing EnvTest and shared Env instance");

    let env_test = EnvTest::default();
    let env_rc = env_test.env().clone();

    debug!("make_posix_env_for_tests: Env instance ready");
    env_rc
}

pub fn testenv_posix_test_main(argc: i32, argv: *mut *mut u8) -> i32 {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    info!(
        "testenv_posix_test_main: entry with argc={}",
        argc
    );

    #[cfg(have_o_cloexec)]
    unsafe {
        if argv.is_null() {
            warn!(
                "testenv_posix_test_main: argv pointer is null; \
                 skipping helper dispatch and argv[0] capture"
            );
        } else {
            // First, check whether we are invoked as a helper process.
            for i in 1..argc {
                let arg_ptr = *argv.add(i as usize);
                if arg_ptr.is_null() {
                    continue;
                }

                let arg_cstr = CStr::from_ptr(arg_ptr as *const c_char);
                let arg_bytes = arg_cstr.to_bytes();

                if arg_bytes == crate::TEST_CLOSE_ON_EXEC_SWITCH.as_bytes() {
                    let helper_arg_ptr = if i + 1 < argc {
                        *argv.add((i + 1) as usize)
                    } else {
                        std::ptr::null_mut()
                    };

                    info!(
                        "testenv_posix_test_main: detected helper invocation \
                         with switch {}; delegating to test_close_on_exec_helper_main",
                        crate::TEST_CLOSE_ON_EXEC_SWITCH
                    );

                    return crate::test_close_on_exec_helper_main(helper_arg_ptr);
                }
            }

            // Not a helper invocation; cache argv[0] for potential debugging use.
            let argv0_ptr = *argv;
            if argv0_ptr.is_null() {
                warn!(
                    "testenv_posix_test_main: argv[0] is null; \
                     not caching program name"
                );
            } else {
                let argv0_cstr = CStr::from_ptr(argv0_ptr as *const c_char);
                let argv0_bytes = argv0_cstr.to_bytes();

                let argv0_display = String::from_utf8_lossy(argv0_bytes);
                debug!(
                    "testenv_posix_test_main: caching argv[0] value '{}'",
                    argv0_display
                );

                let argv0_buf_ptr = crate::get_argv_zero();
                let argv0_buf = &mut *argv0_buf_ptr;

                argv0_buf.clear();
                argv0_buf.extend_from_slice(argv0_bytes);
                argv0_buf.push(0);
            }
        }
    }

    {
        use crate::{TEST_MMAP_LIMIT, TEST_READ_ONLY_FILE_LIMIT};

        info!(
            "testenv_posix_test_main: configuring file limits \
             read_only={}, mmap={}",
            TEST_READ_ONLY_FILE_LIMIT,
            TEST_MMAP_LIMIT
        );

        EnvPosixTest::set_file_limits(TEST_READ_ONLY_FILE_LIMIT, TEST_MMAP_LIMIT);
    }

    info!(
        "testenv_posix_test_main: returning success; \
         Rust tests are executed via the standard test harness"
    );

    0
}
