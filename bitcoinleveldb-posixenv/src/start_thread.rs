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

#[cfg(test)]
mod posix_env_start_thread_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    fn start_thread_increment(arg: *mut c_void) -> c_void {
        trace!(?arg, "start_thread_increment: invoked");

        assert!(
            !arg.is_null(),
            "start_thread_increment: expected non-null argument"
        );

        unsafe {
            let counter = &*(arg as *const AtomicUsize);
            let previous = counter.fetch_add(1, Ordering::SeqCst);

            debug!(
                previous,
                current = previous + 1,
                "start_thread_increment: counter updated"
            );
        }

        unsafe { std::mem::zeroed() }
    }

    #[traced_test]
    fn start_thread_launches_detached_thread_and_runs_callback() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let counter: &'static AtomicUsize =
            Box::leak(Box::new(AtomicUsize::new(0)));

        let ptr = counter as *const AtomicUsize as *mut c_void;

        env.start_thread(start_thread_increment, ptr);

        let deadline = Instant::now() + Duration::from_secs(2);

        while counter.load(Ordering::SeqCst) == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }

        assert!(
            counter.load(Ordering::SeqCst) >= 1,
            "start_thread should have executed callback in detached thread"
        );
    }
}
