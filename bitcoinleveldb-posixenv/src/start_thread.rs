// ---------------- [ File: bitcoinleveldb-posixenv/src/start_thread.rs ]
crate::ix!();

impl StartThread for PosixEnv {

    fn start_thread(
        &mut self,
        thread_main:     fn(thread_main_arg: *mut c_void) -> c_void,
        thread_main_arg: *mut c_void,
    ) {
        let func_ptr = thread_main as *const ();
        let arg_raw  = thread_main_arg as usize;

        trace!(
            function = ?func_ptr,
            arg      = ?thread_main_arg,
            "PosixEnv::start_thread: starting new detached thread"
        );

        let builder = std::thread::Builder::new()
            .name("bitcoinleveldb-posixenv-thread".to_owned());

        let spawn_result = builder.spawn(move || unsafe {
            let local_arg = arg_raw as *mut c_void;

            trace!(
                function = ?(thread_main as *const ()),
                arg      = ?local_arg,
                "PosixEnv::start_thread: thread main entry"
            );

            thread_main(local_arg);

            trace!(
                function = ?(thread_main as *const ()),
                arg      = ?local_arg,
                "PosixEnv::start_thread: thread main exit"
            );
        });

        match spawn_result {
            Ok(_handle) => {
                debug!(
                    function = ?func_ptr,
                    arg      = ?thread_main_arg,
                    "PosixEnv::start_thread: thread spawned successfully"
                );
            }
            Err(err) => {
                error!(
                    function = ?func_ptr,
                    arg      = ?thread_main_arg,
                    error    = %err,
                    "PosixEnv::start_thread: failed to spawn thread"
                );
            }
        }
    }
}
