// ---------------- [ File: bitcoinleveldb-posixenv/src/background_thread_main.rs ]
crate::ix!();

impl PosixEnv {

    /// Background worker loop which dequeues scheduled work items and
    /// executes them one by one.
    ///
    /// This mirrors the C++ loop:
    ///
    /// ```c++
    /// while (true) {
    ///   background_work_mutex_.Lock();
    ///   while (background_work_queue_.empty()) {
    ///     background_work_cv_.Wait();
    ///   }
    ///   auto f = background_work_queue_.front().function;
    ///   void* a = background_work_queue_.front().arg;
    ///   background_work_queue_.pop();
    ///   background_work_mutex_.Unlock();
    ///   f(a);
    /// }
    /// ```
    pub fn background_thread_main(&mut self) {
        trace!("PosixEnv::background_thread_main: background thread started");

        loop {
            // Take one work item out of the shared queue while holding the
            // background_work_mutex. We keep the actual user callback and
            // argument outside the critical section.
            let mut maybe_item: Option<PosixEnvBackgroundWorkItem> = None;

            {
                // Lock the shared background-work state.
                let mut work_guard = self.background_work_mutex_mut().lock();

                // Wait until there is at least one work item.
                while work_guard.background_work_queue().is_empty() {
                    trace!(
                        "PosixEnv::background_thread_main: queue empty, waiting for work"
                    );

                    // The Condvar API expects a mutable guard. Using raw pointers
                    // here avoids borrow-checker limitations while preserving the
                    // logical relationship between the condvar and the guard.
                    let cv_raw: *const _ = work_guard.background_work_cv();
                    let guard_raw: *mut _ = &mut work_guard;

                    unsafe {
                        (*cv_raw).wait(&mut *guard_raw);
                    }
                }

                debug!(
                    queue_len = work_guard.background_work_queue().len(),
                    "PosixEnv::background_thread_main: work available; dequeuing item"
                );

                // Pop a single work item.
                maybe_item = work_guard.background_work_queue().pop();
                // `work_guard` is dropped here, releasing the mutex before we run
                // the user callback.
            }

            let item = match maybe_item {
                Some(item) => item,
                None => {
                    // This should be unreachable given the is_empty() check, but
                    // we handle it defensively.
                    warn!(
                        "PosixEnv::background_thread_main: queue reported non-empty \
                         but pop() returned None; continuing"
                    );
                    continue;
                }
            };

            let function = *item.function();
            let arg      = *item.arg();

            let func_ptr = function as *const ();
            let arg_ptr  = arg;

            debug!(
                function = ?func_ptr,
                arg      = ?arg_ptr,
                "PosixEnv::background_thread_main: executing background work item"
            );

            unsafe {
                function(arg);
            }

            trace!(
                function = ?func_ptr,
                arg      = ?arg_ptr,
                "PosixEnv::background_thread_main: background work item completed"
            );
        }
    }
}
