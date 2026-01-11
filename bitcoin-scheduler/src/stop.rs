// ---------------- [ File: bitcoin-scheduler/src/stop.rs ]
crate::ix!();

impl Stop for Scheduler {
    /// Tell any threads running serviceQueue to stop as soon as the current task is done
    ///
    fn stop(&mut self) {
        info!("Scheduler::stop: stop requested");

        {
            let mutex_ref = self.new_task_mutex().borrow();
            let mut inner = mutex_ref.lock();
            inner.set_stop_requested(true);

            trace!(
                stop_requested = inner.stop_requested(),
                "Scheduler::stop: stop_requested set"
            );
        }

        self.new_task_scheduled().notify_all();
        trace!("Scheduler::stop: notified all waiters");

        info!("Scheduler::stop: serviceQueue thread join is managed externally");

        /*
        [&]() { LOCK(newTaskMutex);  stopRequested = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }
}

#[cfg(test)]
mod stop_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn stop_sets_stop_requested_and_prevents_task_execution_when_called_before_service_queue() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_cb = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("task should not run after stop()");
                ran_cb.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        Stop::stop(&mut scheduler);

        assert!(ShouldStop::should_stop(&scheduler));

        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn stop_is_idempotent_and_does_not_panic_when_called_multiple_times() {
        let mut scheduler = scheduler_for_unit_testing();

        Stop::stop(&mut scheduler);
        Stop::stop(&mut scheduler);

        assert!(ShouldStop::should_stop(&scheduler));
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }
}
