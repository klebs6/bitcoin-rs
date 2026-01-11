// ---------------- [ File: bitcoin-scheduler/src/are_threads_servicing_queue.rs ]
crate::ix!();

impl AreThreadsServicingQueue for Scheduler {
    /// Returns true if there are threads actively running in serviceQueue()
    ///
    fn are_threads_servicing_queue(&self) -> bool {
        trace!("Scheduler::are_threads_servicing_queue: acquiring new_task_mutex");

        let mutex_ref = self.new_task_mutex().borrow();
        let inner = mutex_ref.lock();

        let n_threads_servicing_queue = inner.n_threads_servicing_queue();
        let result = n_threads_servicing_queue != 0;

        trace!(
            n_threads_servicing_queue,
            result,
            "Scheduler::are_threads_servicing_queue: done"
        );

        result
    }
}

#[cfg(test)]
mod are_threads_servicing_queue_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn are_threads_servicing_queue_reports_false_before_start_and_after_exit() {
        let mut scheduler = scheduler_for_unit_testing();

        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));

        let scheduler_ptr: *mut Scheduler = &mut scheduler;
        let observed_during_task = Arc::new(AtomicBool::new(false));
        let observed_during_task_cb = observed_during_task.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("checking AreThreadsServicingQueue from inside a queued task");
                let value =
                    unsafe { AreThreadsServicingQueue::are_threads_servicing_queue(&*scheduler_ptr) };
                observed_during_task_cb.store(value, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(observed_during_task.load(Ordering::SeqCst));
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }

    #[traced_test]
    fn are_threads_servicing_queue_is_false_for_fresh_scheduler_with_no_service_queue_run() {
        let scheduler = scheduler_for_unit_testing();
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }
}
