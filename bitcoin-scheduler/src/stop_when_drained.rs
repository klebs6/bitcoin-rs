// ---------------- [ File: bitcoin-scheduler/src/stop_when_drained.rs ]
crate::ix!();

impl StopWhenDrained for Scheduler {
    /// Tell any threads running serviceQueue to stop when there is no work left to be done
    ///
    fn stop_when_drained(&mut self) {
        info!("Scheduler::stop_when_drained: stop when empty requested");

        {
            let mutex_ref = self.new_task_mutex().borrow();
            let mut inner = mutex_ref.lock();
            inner.set_stop_when_empty(true);

            trace!(
                stop_when_empty = inner.stop_when_empty(),
                "Scheduler::stop_when_drained: stop_when_empty set"
            );
        }

        self.new_task_scheduled().notify_all();
        trace!("Scheduler::stop_when_drained: notified all waiters");

        info!("Scheduler::stop_when_drained: serviceQueue thread join is managed externally");

        /*
        [&]() { LOCK(newTaskMutex);  stopWhenEmpty = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }
}

#[cfg(test)]
mod stop_when_drained_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn stop_when_drained_allows_service_queue_to_exit_immediately_when_queue_is_empty() {
        let mut scheduler = scheduler_for_unit_testing();

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }

    #[traced_test]
    fn stop_when_drained_processes_all_tasks_then_exits_when_queue_becomes_empty() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_a = ran.clone();
        let ran_b = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("task A executed");
                ran_a.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(2),
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("task B executed");
                ran_b.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 2);
    }
}
