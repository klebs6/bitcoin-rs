// ---------------- [ File: bitcoinleveldb-posixenv/src/schedule.rs ]
crate::ix!();

impl Schedule for PosixEnv {

    fn schedule(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        let func_ptr = function as *const ();

        // Capture the PosixEnv pointer as an integer *before* we take the
        // background_work_mutex lock to avoid borrowing `self` while the
        // mutex guard is alive (which would violate Rust's aliasing rules).
        let env_ptr_value: usize = self as *mut PosixEnv as usize;

        trace!(
            function = ?func_ptr,
            arg      = ?arg,
            "PosixEnv::schedule: scheduling background work item"
        );

        // Lock the background work state.
        let mut work_guard = self.background_work_mutex_mut().lock();

        // Start the background thread if we have not done so yet.
        if !work_guard.started_background_thread() {
            debug!(
                "PosixEnv::schedule: starting background worker thread for the first time"
            );

            work_guard.set_started_background_thread(true);

            let builder = std::thread::Builder::new()
                .name("bitcoinleveldb-posixenv-bg".to_owned());

            let spawn_result = builder.spawn(move || unsafe {
                let env_ptr = env_ptr_value as *mut PosixEnv;

                trace!(
                    env_ptr = ?env_ptr,
                    "PosixEnv::schedule: background thread entry"
                );

                PosixEnv::background_thread_entry_point(env_ptr);

                trace!(
                    env_ptr = ?env_ptr,
                    "PosixEnv::schedule: background thread exit"
                );
            });

            match spawn_result {
                Ok(_handle) => {
                    // Dropping the JoinHandle detaches the thread.
                    debug!(
                        "PosixEnv::schedule: background thread spawned successfully"
                    );
                }
                Err(err) => {
                    error!(
                        error = %err,
                        "PosixEnv::schedule: failed to spawn background thread; \
                         scheduled work item will be dropped"
                    );
                    // Without a background thread there is no place to execute the work.
                    // We deliberately drop this item rather than enqueue it.
                    return;
                }
            }
        }

        // Check if the queue is currently empty before we enqueue.
        let was_empty = work_guard.background_work_queue().is_empty();

        // Enqueue the new work item.
        work_guard
            .background_work_queue()
            .push(PosixEnvBackgroundWorkItem::new(function, arg));

        debug!(
            function   = ?func_ptr,
            arg        = ?arg,
            queue_len  = work_guard.background_work_queue().len(),
            was_empty,
            "PosixEnv::schedule: enqueued background work item"
        );

        // If the queue was previously empty, the background thread may be
        // waiting on the condition variable.
        if was_empty {
            trace!(
                "PosixEnv::schedule: queue was empty before enqueue; \
                 notifying background thread"
            );
            work_guard.background_work_cv().notify_one();
        }
        // `work_guard` drops here, releasing the mutex.
    }
}
