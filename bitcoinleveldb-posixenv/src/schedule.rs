// ---------------- [ File: bitcoinleveldb-posixenv/src/schedule.rs ]
crate::ix!();

impl Schedule for PosixEnv {

    fn schedule(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        let func_ptr = function as *const ();

        // Capture the PosixEnv pointer as an integer **before** we take any
        // other mutable borrows of `self`. This avoids overlapping mutable
        // borrows (which previously triggered E0499) while still giving the
        // background thread a stable pointer to the same PosixEnv instance.
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

#[cfg(test)]
mod posix_env_schedule_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    fn scheduled_increment(arg: *mut c_void) -> c_void {
        trace!(?arg, "scheduled_increment: invoked");

        assert!(
            !arg.is_null(),
            "scheduled_increment: expected non-null argument pointer"
        );

        unsafe {
            let counter = &*(arg as *const AtomicUsize);
            let previous = counter.fetch_add(1, Ordering::SeqCst);

            debug!(
                previous,
                current = previous + 1,
                "scheduled_increment: counter updated"
            );
        }

        unsafe { std::mem::zeroed() }
    }

    #[traced_test]
    fn schedule_enqueues_multiple_work_items_and_executes_all() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let counter_a: &'static AtomicUsize =
            Box::leak(Box::new(AtomicUsize::new(0)));
        let counter_b: &'static AtomicUsize =
            Box::leak(Box::new(AtomicUsize::new(0)));

        let ptr_a = counter_a as *const AtomicUsize as *mut c_void;
        let ptr_b = counter_b as *const AtomicUsize as *mut c_void;

        env.schedule(scheduled_increment, ptr_a);
        env.schedule(scheduled_increment, ptr_b);

        let deadline = Instant::now() + Duration::from_secs(2);

        while (counter_a.load(Ordering::SeqCst) == 0
            || counter_b.load(Ordering::SeqCst) == 0)
            && Instant::now() < deadline
        {
            std::thread::sleep(Duration::from_millis(10));
        }

        assert!(
            counter_a.load(Ordering::SeqCst) >= 1,
            "first scheduled task should have run at least once"
        );
        assert!(
            counter_b.load(Ordering::SeqCst) >= 1,
            "second scheduled task should have run at least once"
        );
    }
}
